use actix_web::web;

pub mod dtos;
pub mod errors;
mod routes;

use routes::login;

pub fn configure_auth_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/auth").route("/login", web::post().to(login)));
}
