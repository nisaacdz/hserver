use actix_web::web;

mod auth;
mod rooms;
mod search;
mod users;

use crate::v1::{
    auth::configure_auth_routes, rooms::configure_rooms_routes, search::configure_search_routes,
    users::configure_users_routes,
};

pub fn configure_v1_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/v1")
            .configure(configure_auth_routes)
            .configure(configure_search_routes)
            .configure(configure_rooms_routes)
            .configure(configure_users_routes),
    );
}
