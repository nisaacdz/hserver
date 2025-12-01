use actix_web::{HttpResponse, web};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use infrastructure::db::DbPool;
use uuid::Uuid;

use crate::auth::{SessionUser, TokenEngine, generate_auth_cookie, hash_password, verify_password};
use crate::v1::auth::dtos::{AuthUser, LoginRequest, LoginResponse, OnboardRequest};
use crate::v1::auth::errors::AuthError;
use infrastructure::models::*;
use infrastructure::schema::*;

pub async fn login(
    pool: web::Data<DbPool>,
    token_engine: web::Data<TokenEngine>,
    web::Json(req): web::Json<LoginRequest>,
) -> Result<HttpResponse, AuthError> {
    let mut conn = pool.get().await.map_err(|_| AuthError::InternalError)?;

    let user: User = users::table
        .filter(users::email.eq(&req.email))
        .first(&mut conn)
        .await
        .map_err(|e| match e {
            diesel::result::Error::NotFound => AuthError::InvalidCredentials,
            _ => AuthError::InternalError,
        })?;

    let stored_hash = user
        .password_hash
        .as_deref()
        .ok_or(AuthError::InvalidCredentials)?;

    if !verify_password(&req.password, stored_hash).map_err(|_| AuthError::InternalError)? {
        return Err(AuthError::InvalidCredentials);
    }

    let staff_id: Option<Uuid> = staff::table
        .filter(staff::user_id.eq(user.id))
        .select(staff::id)
        .first::<Uuid>(&mut conn)
        .await
        .optional()
        .map_err(|_| AuthError::InternalError)?;

    let session_user = SessionUser {
        id: user.id,
        staff_id,
        email: user.email.clone(),
    };

    let cookie =
        generate_auth_cookie(&token_engine, session_user).map_err(|_| AuthError::InternalError)?;

    let response = LoginResponse {
        user: AuthUser {
            id: user.id,
            email: user.email,
        },
    };

    Ok(HttpResponse::Ok().cookie(cookie).json(response))
}

pub async fn onboard(
    pool: web::Data<DbPool>,
    web::Json(req): web::Json<OnboardRequest>,
) -> Result<HttpResponse, AuthError> {
    let OnboardRequest {
        user_id,
        otp,
        password,
    } = req;

    let mut conn = pool.get().await.map_err(|_| AuthError::InternalError)?;

    let updated_count = diesel::update(
        otps::table
            .filter(otps::user_id.eq(user_id))
            .filter(otps::code.eq(&otp))
            .filter(otps::used_at.is_null())
            .filter(otps::expires_at.gt(diesel::dsl::now)),
    )
    .set(otps::used_at.eq(diesel::dsl::now))
    .execute(&mut conn)
    .await
    .map_err(|_| AuthError::InternalError)?;

    if updated_count == 0 {
        return Err(AuthError::InvalidCredentials);
    }

    let password_hash = hash_password(&password).map_err(|_| AuthError::InternalError)?;

    let updated_user: User = diesel::update(users::table.filter(users::id.eq(user_id)))
        .set(users::password_hash.eq(password_hash))
        .get_result(&mut conn)
        .await
        .map_err(|_| AuthError::InternalError)?;

    Ok(HttpResponse::Ok().json(AuthUser {
        id: updated_user.id,
        email: updated_user.email,
    }))
}
