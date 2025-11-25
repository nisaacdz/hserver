use actix_web::{web, HttpResponse};
use infrastructure::db::DbPool;

use crate::{dtos::LoginRequest, errors::LoginError};

pub async fn login(
    _pool: web::Data<DbPool>,
    web::Json(LoginRequest { email, password }): web::Json<LoginRequest>,
) -> Result<HttpResponse, LoginError> {
    dbg!(email, password);
    todo!()
}
