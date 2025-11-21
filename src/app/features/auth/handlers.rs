use actix_web::{web, HttpResponse};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;

use crate::app::features::auth::models::{LoginRequest, LoginResponse};
use crate::app::features::user::entities::User;
use crate::error::AppError;
use crate::utils::db::DbPool;
use crate::utils::hasher;
use crate::utils::token;

pub async fn login(
    pool: web::Data<DbPool>,
    req: web::Json<LoginRequest>,
) -> Result<HttpResponse, AppError> {
    let mut conn = pool.get().await?;

    // Find user by email using direct query instead of repository
    let user = {
        use crate::schema::users::dsl::*;
        users
            .filter(email.eq(&req.email))
            .get_result::<User>(&mut conn)
            .await
            .map_err(|_| AppError::unauthorized("Invalid credentials"))?
    };

    // Verify password
    let is_valid = hasher::verify(&req.password, &user.password_hash)
        .map_err(|_| AppError::unauthorized("Invalid credentials"))?;

    if !is_valid {
        return Err(AppError::unauthorized("Invalid credentials"));
    }

    // Generate JWT token
    let now = chrono::Utc::now().timestamp();
    let token = token::generate(user.user_id, now)?;

    Ok(HttpResponse::Ok().json(LoginResponse { token }))
}
