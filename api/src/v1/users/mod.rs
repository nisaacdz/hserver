use actix_web::web;

pub mod dtos;
pub mod errors;
pub mod get;

use crate::auth::AuthMiddleware;
use get::*;

pub fn configure_users_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/users").route("/{id}", web::get().to(get_user).wrap(AuthMiddleware)));
}
