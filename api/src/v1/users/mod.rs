use actix_web::web;

mod dtos;
mod errors;
mod get;

use crate::auth::AuthMiddleware;
use get::get_user;

pub fn configure_users_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/users").route("/{id}", web::get().to(get_user).wrap(AuthMiddleware)));
}
