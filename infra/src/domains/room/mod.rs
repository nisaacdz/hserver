use crate::db::DbPool;
use crate::models::Room as DbRoom;
use crate::models::{Amenity, RoomClass, RoomClassAmenity};
use crate::models::{Block, Booking, Maintenance};
use crate::schema::amenities;
use crate::schema::{blocks, bookings, maintenance};
use crate::schema::{room_classes, rooms};
use app::api::ApiResponse;
use app::auth::SessionUser;
use app::interval::{LowerBound, UpperBound};
use app::rooms::availability::*;
use app::rooms::classes::*;
use app::rooms::details::*;
use app::rooms::find::*;
use app::rooms::list::*;
use diesel::dsl::*;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use std::ops::Bound;

impl From<DbRoom> for app::rooms::list::ListedRoom {
    fn from(room: DbRoom) -> Self {
        app::rooms::list::ListedRoom {
            id: room.id,
            label: room.label,
            class_id: room.class_id,
            created_at: room.created_at,
        }
    }
}

pub async fn list(
    pool: &DbPool,
    options: ListRoomOptions,
    user: &SessionUser,
) -> ApiResponse<ListRoomSuccess, ListRoomError> {
    if user.staff_id.is_none() {
        return ApiResponse::error(ListRoomError::Unauthorized);
    }

    let mut conn = match pool.get().await {
        Ok(conn) => conn,
        Err(_) => return ApiResponse::error(ListRoomError::InternalError),
    };

    let mut count_query = rooms::table.into_boxed();
    let mut list_query = rooms::table.into_boxed();

    if let Some(search) = &options.search {
        let pattern = format!("%{}%", search);
        count_query = count_query.filter(rooms::label.ilike(pattern.clone()));
        list_query = list_query.filter(rooms::label.ilike(pattern));
    }

    let total_rooms: i64 = match count_query.count().get_result(&mut conn).await {
        Ok(total) => total,
        Err(e) => return ApiResponse::error(ListRoomError::DatabaseError(e.to_string())),
    };

    let page = options.page.max(1);
    let per_page = options.per_page.max(1);
    let offset = (page - 1) * per_page;

    let rooms_list: Vec<DbRoom> = match list_query
        .limit(per_page)
        .offset(offset)
        .load::<DbRoom>(&mut conn)
        .await
    {
        Ok(rooms) => rooms,
        Err(e) => return ApiResponse::error(ListRoomError::DatabaseError(e.to_string())),
    };

    let domain_rooms: Vec<ListedRoom> = rooms_list.into_iter().map(Into::into).collect();

    ApiResponse::success(ListRoomSuccess {
        rooms: domain_rooms,
        total: total_rooms as usize,
    })
}

pub async fn get_availability(
    pool: &DbPool,
    options: GetAvailabilityOptions,
    user: &SessionUser,
) -> ApiResponse<GetAvailabilitySuccess, GetAvailabilityError> {
    if user.staff_id.is_none() {
        return ApiResponse::error(GetAvailabilityError::Unauthorized);
    }

    let mut conn = match pool.get().await {
        Ok(conn) => conn,
        Err(_) => return ApiResponse::error(GetAvailabilityError::InternalError),
    };

    let period = (Bound::Included(options.start), Bound::Excluded(options.end));

    let data: Vec<(Block, Option<Booking>, Option<Maintenance>)> = match blocks::table
        .filter(blocks::room_id.eq(options.room_id))
        .filter(blocks::interval.overlaps_with(period))
        .left_join(bookings::table)
        .left_join(maintenance::table)
        .order(blocks::interval.asc())
        .load::<(Block, Option<Booking>, Option<Maintenance>)>(&mut conn)
        .await
    {
        Ok(data) => data,
        Err(_) => return ApiResponse::error(GetAvailabilityError::NotFound),
    };

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

    ApiResponse::success(GetAvailabilitySuccess {
        room_id: options.room_id,
        period,
        blocks: calendar_blocks,
    })
}

pub async fn get_details(
    pool: &DbPool,
    options: GetDetailsOptions,
) -> ApiResponse<GetDetailsSuccess, GetDetailsError> {
    let mut conn = match pool.get().await {
        Ok(conn) => conn,
        Err(_) => return ApiResponse::error(GetDetailsError::InternalError),
    };

    let (room, room_class): (DbRoom, RoomClass) = match rooms::table
        .find(options.room_id)
        .inner_join(room_classes::table)
        .select((DbRoom::as_select(), RoomClass::as_select()))
        .first(&mut conn)
        .await
    {
        Ok(data) => data,
        Err(diesel::result::Error::NotFound) => {
            return ApiResponse::error(GetDetailsError::NotFound);
        }
        Err(_) => return ApiResponse::error(GetDetailsError::InternalError),
    };

    ApiResponse::success(GetDetailsSuccess {
        id: room.id,
        label: room.label,
        class_id: room.class_id,
        class: RoomClassSummary {
            id: room_class.id,
            name: room_class.name,
            base_price: room_class.base_price,
        },
    })
}

pub async fn get_classes(
    pool: &DbPool,
) -> ApiResponse<Vec<RoomClassWithAmenities>, GetClassesError> {
    let mut conn = match pool.get().await {
        Ok(conn) => conn,
        Err(_) => return ApiResponse::error(GetClassesError::InternalError),
    };

    let classes: Vec<RoomClass> = match room_classes::table.load::<RoomClass>(&mut conn).await {
        Ok(classes) => classes,
        Err(_) => return ApiResponse::error(GetClassesError::InternalError),
    };

    let amenities_data: Vec<(RoomClassAmenity, Amenity)> =
        match RoomClassAmenity::belonging_to(&classes)
            .inner_join(amenities::table)
            .select((RoomClassAmenity::as_select(), Amenity::as_select()))
            .load::<(RoomClassAmenity, Amenity)>(&mut conn)
            .await
        {
            Ok(data) => data,
            Err(_) => return ApiResponse::error(GetClassesError::InternalError),
        };

    let amenities_grouped = amenities_data.grouped_by(&classes);

    let response: Vec<RoomClassWithAmenities> = classes
        .into_iter()
        .zip(amenities_grouped)
        .map(|(room_class, class_amenities)| RoomClassWithAmenities {
            id: room_class.id,
            name: room_class.name,
            base_price: room_class.base_price,
            amenities: class_amenities
                .into_iter()
                .map(|(_, amenity)| app::rooms::classes::Amenity {
                    id: amenity.id,
                    name: amenity.name,
                    icon_key: amenity.icon_key,
                })
                .collect(),
        })
        .collect();

    ApiResponse::success(response)
}

pub async fn find(
    pool: &DbPool,
    options: FindRoomOptions,
) -> ApiResponse<FindRoomSuccess, FindRoomError> {
    if options.start >= options.end {
        return ApiResponse::error(FindRoomError::InvalidDateRange);
    }

    let mut conn = match pool.get().await {
        Ok(conn) => conn,
        Err(_) => return ApiResponse::error(FindRoomError::InternalError),
    };

    let search_range = (Bound::Included(options.start), Bound::Excluded(options.end));

    let mut db_query = rooms::table.into_boxed();

    if let Some(cid) = options.class_id {
        db_query = db_query.filter(rooms::class_id.eq(cid));
    }

    db_query = db_query.filter(not(exists(
        blocks::table
            .filter(blocks::room_id.eq(rooms::id))
            .filter(blocks::interval.overlaps_with(search_range)),
    )));

    let available_rooms: Vec<DbRoom> =
        match db_query.select(DbRoom::as_select()).load(&mut conn).await {
            Ok(rooms) => rooms,
            Err(_) => return ApiResponse::error(FindRoomError::InternalError),
        };

    let response_rooms: Vec<RoomSummary> = available_rooms
        .into_iter()
        .map(|r| RoomSummary {
            id: r.id,
            label: r.label,
            class_id: r.class_id,
        })
        .collect();

    ApiResponse::success(FindRoomSuccess {
        rooms: response_rooms,
    })
}
