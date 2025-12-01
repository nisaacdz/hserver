use actix_web::web;

mod dtos;
mod errors;
mod routes;

use routes::{availability, details, find_room, get_room_classes};

use crate::auth::AuthMiddleware;

pub fn configure_rooms_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/rooms")
            .route("/find", web::get().to(find_room))
            .route("/classes", web::get().to(get_room_classes))
            .route("/{id}", web::get().to(details).wrap(AuthMiddleware))
            .route(
                "/{id}/availability",
                web::get().to(availability).wrap(AuthMiddleware),
            ),
    );
}
