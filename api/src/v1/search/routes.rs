use actix_web::{HttpResponse, web};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use infrastructure::db::DbPool;
use infrastructure::models::{Block, Room};
use infrastructure::schema::{blocks, rooms};
use std::collections::HashSet;

use crate::v1::search::dtos::{FindRoomQuery, FindRoomResponse, RoomSummary};
use crate::v1::search::errors::SearchError;

pub async fn find_room(
    pool: web::Data<DbPool>,
    web::Query(query): web::Query<FindRoomQuery>,
) -> Result<HttpResponse, SearchError> {
    if query.start >= query.end {
        return Err(SearchError::InvalidDateRange);
    }

    let mut conn = pool.get().await.map_err(|_| SearchError::InternalError)?;

    // 1. Fetch candidate rooms
    let mut rooms_query = rooms::table.into_boxed();

    if let Some(cid) = query.class_id {
        rooms_query = rooms_query.filter(rooms::class_id.eq(cid));
    }

    let candidate_rooms = rooms_query
        .load::<Room>(&mut conn)
        .await
        .map_err(|_| SearchError::InternalError)?;

    if candidate_rooms.is_empty() {
        return Ok(HttpResponse::Ok().json(FindRoomResponse { rooms: vec![] }));
    }

    let room_ids: Vec<uuid::Uuid> = candidate_rooms.iter().map(|r| r.id).collect();

    // 2. Fetch blocks for these rooms that might overlap
    // We fetch all blocks for these rooms and filter in memory for simplicity regarding Range types
    let existing_blocks = blocks::table
        .filter(blocks::room_id.eq_any(&room_ids))
        .load::<Block>(&mut conn)
        .await
        .map_err(|_| SearchError::InternalError)?;

    // 3. Identify blocked room IDs
    let mut blocked_room_ids = HashSet::new();
    for block in existing_blocks {
        // block.interval is (Bound<DateTime<Utc>>, Bound<DateTime<Utc>>)
        // We need to check overlap with (query.start, query.end)

        let block_start = match block.interval.0 {
            std::ops::Bound::Included(t) => t,
            std::ops::Bound::Excluded(t) => t, // Should be rare for start, but handle it
            std::ops::Bound::Unbounded => chrono::DateTime::<chrono::Utc>::MIN_UTC,
        };

        let block_end = match block.interval.1 {
            std::ops::Bound::Included(t) => t,
            std::ops::Bound::Excluded(t) => t,
            std::ops::Bound::Unbounded => chrono::DateTime::<chrono::Utc>::MAX_UTC,
        };

        // Overlap condition: StartA < EndB && EndA > StartB
        if block_start < query.end && block_end > query.start {
            blocked_room_ids.insert(block.room_id);
        }
    }

    // 4. Filter candidates
    let available_rooms: Vec<RoomSummary> = candidate_rooms
        .into_iter()
        .filter(|r| !blocked_room_ids.contains(&r.id))
        .map(|r| RoomSummary {
            id: r.id,
            label: r.label,
            class_id: r.class_id,
        })
        .collect();

    Ok(HttpResponse::Ok().json(FindRoomResponse {
        rooms: available_rooms,
    }))
}
