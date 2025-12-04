use actix_web::web;

pub mod routes;

use routes::{login, onboard};

pub fn configure_auth_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .route("/login", web::post().to(login))
            .route("/onboard", web::post().to(onboard)),
    );
}
