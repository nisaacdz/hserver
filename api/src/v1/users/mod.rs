use actix_web::web;

pub mod routes;

use crate::auth::AuthMiddleware;
use routes::*;

pub fn configure_users_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/users").route("/{id}", web::get().to(get_user).wrap(AuthMiddleware)));
}
