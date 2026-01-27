use crate::{
    db::DbPool,
    models::{
        Block, Booking, NewBlock, NewBooking, NewBookingReservation, NewReservation, Reservation,
        ReservationStatus, Room, RoomClass,
    },
    schema::{blocks, bookings, bookings_reservations, reservations, room_classes, rooms},
    services::pricing::*,
};
use app::{
    ReservationSettings,
    reservations::{
        PA,
        dtos::{CreateReservationRequest, ReservationItemRequest},
        errors::ReservationError,
        types::{CartItem, ReservationCart},
    },
};
use bigdecimal::{BigDecimal, FromPrimitive, ToPrimitive};
use chrono::{Duration, Utc};
use diesel::prelude::*;
use diesel_async::{AsyncConnection, RunQueryDsl, scoped_futures::ScopedFutureExt};
use std::ops::Bound;
use uuid::Uuid;

pub async fn create_reservation(
    pool: &DbPool,
    user_id: Uuid,
    req: CreateReservationRequest,
    settings: &ReservationSettings,
) -> Result<ReservationCart, ReservationError> {
    let mut conn = pool
        .get()
        .await
        .map_err(|_| ReservationError::InternalError("Db Pool".to_string()))?;

    // 1. Transaction
    let reservation_id = conn
        .transaction::<Uuid, ReservationError, _>(|conn| {
            async move {
                // A. Create Reservation
                let new_reservation = NewReservation {
                    id: None,
                    guest_id: user_id,
                    status: ReservationStatus::Pending,
                    total_amount: BigDecimal::from(0), // Calculate later
                    currency: req.currency.clone(),
                    expires_at: Some(Utc::now() + Duration::minutes(15)),
                };

                let reservation: Reservation = diesel::insert_into(reservations::table)
                    .values(&new_reservation)
                    .get_result(conn)
                    .await?;

                // B. Process Items
                for item in req.items {
                    process_item(conn, &reservation, item).await?;
                }

                Ok(reservation.id)
            }
            .scope_boxed()
        })
        .await?;

    // 2. Fetch Rich Response (Re-query)
    // We can do this outside the write transaction to keep it short,
    // but inside a read transaction or just simple query is fine.
    // We need a helper to fetch the cart.

    // For now, let's implement the fetch logic here inline or call a hypothetical 'get'.
    // To avoid duplication, I'll implement a flexible fetch here.

    let cart = get_reservation_cart_internal(&mut conn, reservation_id, settings).await?;

    Ok(cart)
}

async fn process_item(
    conn: &mut diesel_async::AsyncPgConnection,
    reservation: &Reservation,
    item: ReservationItemRequest,
) -> Result<(), ReservationError> {
    // 1. Check Availability (Overlap)
    // We rely on the unique exclusion constraint on the blocks table.
    // blocks.room_id WITH =, blocks.interval WITH &&

    let validity_range = (
        Bound::Included(item.start_time),
        Bound::Excluded(item.end_time),
    );

    // 2. Create Block
    let new_block = NewBlock {
        id: None,
        room_id: item.room_id,
        interval: validity_range,
    };

    let block: Block = diesel::insert_into(blocks::table)
        .values(&new_block)
        .get_result(conn)
        .await
        .map_err(|e| {
            if let diesel::result::Error::DatabaseError(
                diesel::result::DatabaseErrorKind::UniqueViolation,
                _,
            ) = e
            {
                return ReservationError::RoomUnavailable {
                    room_id: item.room_id,
                    conflict_start: item.start_time,
                    conflict_end: item.end_time,
                };
            }
            ReservationError::from(e)
        })?;

    // 4. Create Booking
    let new_booking = NewBooking {
        block_id: block.id,
        guest_id: reservation.guest_id,
    };
    diesel::insert_into(bookings::table)
        .values(&new_booking)
        .execute(conn)
        .await?;

    // 5. Link
    let link = NewBookingReservation {
        booking_id: block.id,
        reservation_id: reservation.id,
    };
    diesel::insert_into(bookings_reservations::table)
        .values(&link)
        .execute(conn)
        .await?;

    Ok(())
}

// Helper to fetch the cart structure
async fn get_reservation_cart_internal(
    conn: &mut diesel_async::AsyncPgConnection,
    res_id: Uuid,
    settings: &ReservationSettings,
) -> Result<ReservationCart, ReservationError> {
    // Fetch Reservation
    let reservation: Reservation = reservations::table.find(res_id).first(conn).await?;

    // Fetch Items: Join bookings_reservations -> bookings -> blocks -> rooms -> room_classes
    // and booking_addons -> addons (TODO)

    // Complex join!
    // We want `(Booking, Block, Room, RoomClass)` tuples
    let items_data: Vec<(Booking, Block, Room, RoomClass)> = bookings_reservations::table
        .filter(bookings_reservations::reservation_id.eq(res_id))
        .inner_join(bookings::table.on(bookings_reservations::booking_id.eq(bookings::block_id)))
        .inner_join(blocks::table.on(bookings::block_id.eq(blocks::id)))
        .inner_join(rooms::table.on(blocks::room_id.eq(rooms::id)))
        .inner_join(room_classes::table.on(rooms::class_id.eq(room_classes::id)))
        .select((
            bookings::all_columns,
            blocks::all_columns,
            rooms::all_columns,
            room_classes::all_columns,
        ))
        .load(conn)
        .await?;

    let mut cart_items = Vec::new();
    let mut total_cost = BigDecimal::default();

    for (booking, block, room, room_class) in items_data {
        // Extract start/end from block.interval
        // Bound matching is tedious.
        let (start, end) = match block.interval {
            (Bound::Included(s), Bound::Excluded(e)) => (s, e),
            (Bound::Included(s), Bound::Included(e)) => (s, e), // Should not happen for tstzrange usually?
            (Bound::Excluded(s), Bound::Excluded(e)) => (s, e),
            _ => (Utc::now(), Utc::now()), // Fallback
        };

        let amenities_total = BigDecimal::from_f64(0f64).unwrap_or_default();

        let item_cost = calculate_item_cost(room_class.base_price, amenities_total, settings);
        total_cost += item_cost.clone();

        cart_items.push(CartItem {
            booking_id: booking.block_id,
            room_id: room.id,
            room_label: room.label,
            room_class_name: room_class.name,
            start_time: start,
            end_time: end,
            item_total: item_cost.clone().to_f64().unwrap_or_default(),
            addons: vec![], // TODO: Fetch addons (include their costs)
        });
    }

    let discount = calculate_reservation_discount(total_cost.clone(), settings);

    let vat = calculate_reservation_vat(total_cost.clone(), settings);

    let payable_amount =
        calculate_reservation_final(total_cost.clone(), discount.clone(), vat.clone());

    Ok(ReservationCart {
        id: reservation.id,
        code: reservation.code,
        status: app::reservations::types::ReservationStatus::Pending, // Map enum
        currency: reservation.currency,
        total_amount: total_cost.to_f64().unwrap_or_default(),
        discount: PA {
            amount: discount.to_f64().unwrap_or_default(),
            percent: settings.discount_rate,
        },
        vat: PA {
            amount: vat.to_f64().unwrap_or_default(),
            percent: settings.vat_rate,
        },
        payable_amount: payable_amount.to_f64().unwrap_or_default(),
        expires_at: reservation.expires_at,
        items: cart_items,
    })
}
