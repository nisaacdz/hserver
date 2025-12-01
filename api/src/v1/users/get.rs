use actix_web::{HttpResponse, web};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use infrastructure::db::DbPool;
use uuid::Uuid;

use crate::v1::users::dtos::{UserDetails, UserResponse};
use crate::v1::users::errors::UserError;
use infrastructure::models::User;
use infrastructure::schema::users::dsl as users_dsl;

#[utoipa::path(
    get,
    path = "/api/v1/users/{id}",
    params(
        ("id" = Uuid, Path, description = "User ID")
    ),
    responses(
        (status = 200, description = "User found", body = UserResponse),
        (status = 404, description = "User not found")
    )
)]
pub async fn get_user(
    pool: web::Data<DbPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, UserError> {
    let user_id = path.into_inner();
    let mut conn = pool.get().await.map_err(|_| UserError::InternalError)?;

    let user: User = users_dsl::users
        .find(user_id)
        .first(&mut conn)
        .await
        .map_err(|e| match e {
            diesel::result::Error::NotFound => UserError::NotFound,
            _ => UserError::InternalError,
        })?;

    let response = UserResponse {
        user: UserDetails {
            id: user.id,
            email: user.email,
        },
    };

    Ok(HttpResponse::Ok().json(response))
}
