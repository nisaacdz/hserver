use actix_web::{HttpResponse, web};
use infra::db::DbPool;

use crate::auth::{TokenEngine, generate_auth_cookie};
use app::auth::login::LoginRequest;
use app::auth::onboard::OnboardRequest;
use app::auth::{AuthError, SessionUser};
use infra::domains::auth;

#[utoipa::path(
    post,
    path = "/api/v1/auth/login",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Login successful", body = SessionUser),
        (status = 401, description = "Invalid credentials")
    )
)]
pub async fn login(
    pool: web::Data<DbPool>,
    token_engine: web::Data<TokenEngine>,
    web::Json(req): web::Json<LoginRequest>,
) -> Result<HttpResponse, AuthError> {
    let user = auth::login(&pool, req).await?;

    let cookie =
        generate_auth_cookie(&token_engine, user.clone()).map_err(|_| AuthError::InternalError)?;

    // Return simple JSON with id and email as requested
    Ok(HttpResponse::Ok().cookie(cookie).json(serde_json::json!({
        "id": user.id,
        "email": user.email
    })))
}

#[utoipa::path(
    post,
    path = "/api/v1/auth/onboard",
    request_body = OnboardRequest,
    responses(
        (status = 200, description = "Onboarding successful", body = SessionUser),
        (status = 401, description = "Invalid credentials or expired OTP")
    )
)]
pub async fn onboard(
    pool: web::Data<DbPool>,
    token_engine: web::Data<TokenEngine>,
    web::Json(req): web::Json<OnboardRequest>,
) -> Result<HttpResponse, AuthError> {
    let user = auth::onboard(&pool, req).await?;

    let cookie =
        generate_auth_cookie(&token_engine, user.clone()).map_err(|_| AuthError::InternalError)?;

    Ok(HttpResponse::Ok().cookie(cookie).json(serde_json::json!({
        "id": user.id,
        "email": user.email
    })))
}
