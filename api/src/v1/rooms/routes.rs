use actix_web::{HttpResponse, web};
use infra::db::DbPool;
use std::rc::Rc;
use uuid::Uuid;

use crate::auth::SessionUser;
use crate::v1::rooms::dtos::*;
use app::rooms::availability::*;
use app::rooms::classes::*;
use app::rooms::details::*;
use app::rooms::find::*;
use app::rooms::list::*;
use infra::domains::room;

#[utoipa::path(
    get,
    path = "/api/v1/rooms/{id}/availability",
    params(
        ("id" = Uuid, Path, description = "Room ID"),
        RoomAvailabilityQuery
    ),
    responses(
        (status = 200, description = "Room availability", body = GetAvailabilitySuccess),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Room not found")
    )
)]
pub async fn get_room_availability(
    pool: web::Data<DbPool>,
    user: web::ReqData<Rc<SessionUser>>,
    path: web::Path<Uuid>,
    query: web::Query<RoomAvailabilityQuery>,
) -> HttpResponse {
    let options = GetAvailabilityOptions {
        room_id: path.into_inner(),
        start: query.start,
        end: query.end,
    };

    room::get_availability(&pool, options, &user).await.into()
}

#[utoipa::path(
    get,
    path = "/api/v1/rooms/{id}",
    params(
        ("id" = Uuid, Path, description = "Room ID")
    ),
    responses(
        (status = 200, description = "Room details", body = GetDetailsSuccess),
        (status = 404, description = "Room not found")
    )
)]
pub async fn get_room_details(
    pool: web::Data<DbPool>,
    _user: web::ReqData<Rc<SessionUser>>,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let options = GetDetailsOptions {
        room_id: path.into_inner(),
    };

    room::get_details(&pool, options).await.into()
}

#[utoipa::path(
    get,
    path = "/api/v1/rooms/classes",
    responses(
        (status = 200, description = "List of room classes", body = Vec<RoomClassWithAmenities>)
    )
)]
pub async fn get_room_classes(pool: web::Data<DbPool>) -> HttpResponse {
    room::get_classes(&pool).await.into()
}

#[utoipa::path(
    get,
    path = "/api/v1/rooms",
    params(
        FindRoomQuery
    ),
    responses(
        (status = 200, description = "List of available rooms", body = FindRoomSuccess),
        (status = 400, description = "Invalid date range")
    )
)]
pub async fn find_room(
    pool: web::Data<DbPool>,
    web::Query(query): web::Query<FindRoomQuery>,
) -> HttpResponse {
    let options = FindRoomOptions {
        start: query.start,
        end: query.end,
        class_id: query.class_id,
    };

    room::find(&pool, options).await.into()
}

#[utoipa::path(
    get,
    path = "/api/v1/rooms/list",
    params(
        RoomListQuery
    ),
    responses(
        (status = 200, description = "List of rooms", body = RoomListResponse),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn list_rooms(
    pool: web::Data<DbPool>,
    user: web::ReqData<Rc<SessionUser>>,
    web::Query(query): web::Query<RoomListQuery>,
) -> HttpResponse {
    let page = query.page.unwrap_or(1);
    let per_page = query.per_page.unwrap_or(10);

    let options = ListRoomOptions {
        search: query.search,
        page,
        per_page,
    };

    room::list(&pool, options, &user).await.into()
}
