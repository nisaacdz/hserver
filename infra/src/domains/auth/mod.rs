use crate::db::DbPool;
use crate::models::User as DbUser;
use crate::schema::{otps, staff, users};
use app::auth::login::LoginRequest;
use app::auth::onboard::OnboardRequest;
use app::auth::{AuthError, SessionUser, hash_password, verify_password};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use uuid::Uuid;

pub async fn login(pool: &DbPool, req: LoginRequest) -> Result<SessionUser, AuthError> {
    let mut conn = pool.get().await.map_err(|_| AuthError::InternalError)?;

    let user: DbUser = users::table
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

    Ok(SessionUser {
        id: user.id,
        staff_id,
        email: user.email,
    })
}

pub async fn onboard(pool: &DbPool, req: OnboardRequest) -> Result<SessionUser, AuthError> {
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

    let updated_user: DbUser = diesel::update(users::table.filter(users::id.eq(user_id)))
        .set(users::password_hash.eq(password_hash))
        .get_result(&mut conn)
        .await
        .map_err(|_| AuthError::InternalError)?;

    Ok(SessionUser {
        id: updated_user.id,
        staff_id: None,
        email: updated_user.email,
    })
}
