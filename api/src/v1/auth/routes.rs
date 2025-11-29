use actix_web::{HttpResponse, web};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use domain::SecurityConfig;
use infrastructure::db::DbPool;

use crate::auth::{SessionUser, generate_auth_cookie};
use crate::v1::auth::dtos::{AuthUser, LoginRequest, LoginResponse};
use crate::v1::auth::errors::AuthError;
use infrastructure::models::User;
use infrastructure::schema::users::dsl as users_dsl;

pub async fn login(
    pool: web::Data<DbPool>,
    config: web::Data<SecurityConfig>,
    web::Json(req): web::Json<LoginRequest>,
) -> Result<HttpResponse, AuthError> {
    let mut conn = pool.get().await.map_err(|_| AuthError::InternalError)?;

    let user: User = users_dsl::users
        .filter(users_dsl::email.eq(&req.email))
        .first(&mut conn)
        .await
        .map_err(|e| match e {
            diesel::result::Error::NotFound => AuthError::InvalidCredentials,
            _ => AuthError::InternalError,
        })?;

    let stored_pass = user
        .password_hash
        .as_deref()
        .ok_or(AuthError::InvalidCredentials)?;

    // Simple check (REPLACE WITH HASH VERIFICATION)
    if stored_pass != req.password {
        return Err(AuthError::InvalidCredentials);
    }

    let session_user = SessionUser {
        id: user.id,
        staff_id: None,
        email: user.email.clone(),
    };

    let cookie = generate_auth_cookie(session_user, &config);

    let response = LoginResponse {
        user: AuthUser {
            id: user.id,
            email: user.email,
            username: None, // User model doesn't have username
        },
    };

    Ok(HttpResponse::Ok().cookie(cookie).json(response))
}
