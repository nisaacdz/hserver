use actix_web::web;

mod dtos;
mod errors;
mod routes;

use routes::{availability, details};

use crate::auth::AuthMiddleware;

pub fn configure_rooms_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/rooms")
            .route(
                "/availability/{id}",
                web::get().to(availability).wrap(AuthMiddleware),
            )
            .route("/{id}", web::get().to(details).wrap(AuthMiddleware)),
    );
}
