use actix_web::{HttpResponse, web};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use infrastructure::db::DbPool;
use std::rc::Rc;
use uuid::Uuid;

use crate::auth::SessionUser;
use crate::v1::rooms::dtos::{BlockKind, CalendarBlock, RoomAvailability, RoomAvailabilityQuery};
use crate::v1::rooms::errors::RoomError;
use infrastructure::models::{Block, Booking, Maintenance};
use infrastructure::schema::{blocks, bookings, maintenance};

pub async fn availability(
    pool: web::Data<DbPool>,
    user: web::ReqData<Rc<SessionUser>>,
    path: web::Path<Uuid>,
    query: web::Query<RoomAvailabilityQuery>,
) -> Result<HttpResponse, RoomError> {
    if user.staff_id.is_none() {
        return Err(RoomError::Unauthorized);
    }

    let room_id = path.into_inner();

    let mut conn = pool.get().await.map_err(|_| RoomError::InternalError)?;

    let data: Vec<(Block, Option<Booking>, Option<Maintenance>)> = blocks::table
        .filter(blocks::room_id.eq(room_id))
        .filter(blocks::interval.overlaps_with(query.period))
        .left_join(bookings::table)
        .left_join(maintenance::table)
        .order(blocks::interval.asc())
        .load::<(Block, Option<Booking>, Option<Maintenance>)>(&mut conn)
        .await
        .map_err(|e| {
            eprintln!("DB Error: {:?}", e);
            RoomError::InternalError
        })?;

    let mut calendar_blocks = Vec::new();

    for (block, booking, maintenance_record) in data {
        let (kind, label) = if let Some(_) = booking {
            (BlockKind::Booking, Some("Booking".to_string()))
        } else if let Some(m) = maintenance_record {
            (BlockKind::Maintenance, Some(format!("{:?}", m.kind)))
        } else {
            (BlockKind::Unknown, None)
        };

        calendar_blocks.push(CalendarBlock {
            id: block.id,
            period: block.interval,
            kind,
            label,
        });
    }

    let response = RoomAvailability {
        room_id,
        period: query.period,
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
