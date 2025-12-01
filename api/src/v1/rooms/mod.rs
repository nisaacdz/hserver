use actix_web::web;

mod dtos;
mod errors;
mod routes;

use routes::{get_room_availability, get_room_details, find_room, get_room_classes};

use crate::auth::AuthMiddleware;

pub fn configure_rooms_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/rooms")
            .route("/find", web::get().to(find_room))
            .route("/classes", web::get().to(get_room_classes))
            .route("/{id}", web::get().to(get_room_details).wrap(AuthMiddleware))
            .route(
                "/{id}/availability",
                web::get().to(get_room_availability).wrap(AuthMiddleware),
            ),
    );
}
