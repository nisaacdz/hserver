use actix_web::{HttpResponse, web};
use diesel::{dsl::*, prelude::*};
use diesel_async::RunQueryDsl;
use infrastructure::db::DbPool;
use std::ops::Bound;
use std::rc::Rc;
use uuid::Uuid;

use crate::auth::SessionUser;
use crate::v1::rooms::dtos::*;
use crate::v1::rooms::errors::*;
use domain::interval::{LowerBound, UpperBound};
use infrastructure::models::*;
use infrastructure::schema::*;

pub async fn availability(
    pool: web::Data<DbPool>,
    user: web::ReqData<Rc<SessionUser>>,
    path: web::Path<Uuid>,
    query: web::Query<RoomAvailabilityQuery>,
) -> Result<HttpResponse, RoomError> {
    if user.staff_id.is_none() {
        return Err(RoomError::Unauthorized);
    }

    let period = (Bound::Included(query.start), Bound::Excluded(query.end));

    let room_id = path.into_inner();

    let mut conn = pool.get().await.map_err(|_| RoomError::InternalError)?;

    let data: Vec<(Block, Option<Booking>, Option<Maintenance>)> = blocks::table
        .filter(blocks::room_id.eq(room_id))
        .filter(blocks::interval.overlaps_with(period))
        .left_join(bookings::table)
        .left_join(maintenance::table)
        .order(blocks::interval.asc())
        .load::<(Block, Option<Booking>, Option<Maintenance>)>(&mut conn)
        .await
        .map_err(|e| {
            eprintln!("DB Error: {:?}", e);
            RoomError::NotFound
        })?;

    let calendar_blocks = data
        .into_iter()
        .map(|(block, booking, maintenance_record)| {
            let (kind, label) = if let Some(booking) = booking {
                (BlockKind::Booking, Some(booking.status))
            } else if let Some(m) = maintenance_record {
                (BlockKind::Maintenance, Some(format!("{:?}", m.kind)))
            } else {
                (BlockKind::Unknown, None)
            };

            CalendarBlock {
                id: block.id,
                period: block.interval,
                kind,
                label,
            }
        })
        .collect::<Vec<_>>();

    let period = if calendar_blocks.is_empty() {
        period
    } else {
        (
            std::cmp::min(
                LowerBound(period.0),
                LowerBound(calendar_blocks[0].period.0),
            )
            .0,
            std::cmp::max(
                UpperBound(period.1),
                UpperBound(calendar_blocks.last().unwrap().period.1),
            )
            .0,
        )
    };

    let response = RoomAvailability {
        room_id,
        period,
        blocks: calendar_blocks,
    };

    Ok(HttpResponse::Ok().json(response))
}

pub async fn details(
    _pool: web::Data<DbPool>,
    _user: web::ReqData<Rc<SessionUser>>,
    _path: web::Path<Uuid>,
    _query: web::Query<RoomAvailabilityQuery>,
) -> Result<HttpResponse, RoomError> {
    Ok(HttpResponse::Ok().json(()))
}

pub async fn get_room_classes(pool: web::Data<DbPool>) -> Result<HttpResponse, RoomError> {
    let mut conn = pool.get().await.map_err(|_| RoomError::InternalError)?;

    let classes: Vec<RoomClass> = room_classes::table
        .load::<RoomClass>(&mut conn)
        .await
        .map_err(|_| RoomError::InternalError)?;

    let amenities_data: Vec<(RoomClassAmenity, Amenity)> = RoomClassAmenity::belonging_to(&classes)
        .inner_join(amenities::table)
        .select((RoomClassAmenity::as_select(), Amenity::as_select()))
        .load::<(RoomClassAmenity, Amenity)>(&mut conn)
        .await
        .map_err(|_| RoomError::InternalError)?;

    let amenities_grouped = amenities_data.grouped_by(&classes);

    let response: Vec<RoomClassResponse> = classes
        .into_iter()
        .zip(amenities_grouped)
        .map(|(room_class, class_amenities)| RoomClassResponse {
            id: room_class.id,
            name: room_class.name,
            base_price: room_class.base_price,
            amenities: class_amenities
                .into_iter()
                .map(|(_, amenity)| AmenityDto {
                    id: amenity.id,
                    name: amenity.name,
                    icon_key: amenity.icon_key,
                })
                .collect(),
        })
        .collect();

    Ok(HttpResponse::Ok().json(response))
}

pub async fn find_room(
    pool: web::Data<DbPool>,
    web::Query(query): web::Query<FindRoomQuery>,
) -> Result<HttpResponse, SearchError> {
    if query.start >= query.end {
        return Err(SearchError::InvalidDateRange);
    }

    let mut conn = pool.get().await.map_err(|_| SearchError::InternalError)?;

    let search_range = (Bound::Included(query.start), Bound::Excluded(query.end));

    let mut db_query = rooms::table.into_boxed();

    if let Some(cid) = query.class_id {
        db_query = db_query.filter(rooms::class_id.eq(cid));
    }

    db_query = db_query.filter(not(exists(
        blocks::table
            .filter(blocks::room_id.eq(rooms::id))
            .filter(blocks::interval.overlaps_with(search_range)),
    )));

    let available_rooms: Vec<Room> = db_query
        .select(Room::as_select())
        .load(&mut conn)
        .await
        .map_err(|_| SearchError::InternalError)?;

    let response_rooms: Vec<RoomSummary> = available_rooms
        .into_iter()
        .map(|r| RoomSummary {
            id: r.id,
            label: r.label,
            class_id: r.class_id,
        })
        .collect();

    Ok(HttpResponse::Ok().json(FindRoomResponse {
        rooms: response_rooms,
    }))
}
