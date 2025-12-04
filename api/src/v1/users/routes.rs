use actix_web::{HttpResponse, web};
use infra::db::DbPool;
use uuid::Uuid;

use app::users::details::*;
use infra::domains::user;

#[utoipa::path(
    get,
    path = "/api/v1/users/{id}",
    params(
        ("id" = Uuid, Path, description = "User ID")
    ),
    responses(
        (status = 200, description = "User found", body = GetUserDetailsSuccess),
        (status = 404, description = "User not found")
    )
)]
pub async fn get_user(pool: web::Data<DbPool>, path: web::Path<Uuid>) -> HttpResponse {
    let options = GetUserDetailsOptions {
        user_id: path.into_inner(),
    };

    user::get_details(&pool, options).await.into()
}
