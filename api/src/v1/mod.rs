use actix_web::web;

mod auth;
mod search;

use crate::v1::{auth::configure_auth_routes, search::configure_search_routes};

pub fn configure_v1_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/v1")
            .configure(configure_auth_routes)
            .configure(configure_search_routes),
    );
}
