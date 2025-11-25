use actix_web::web;

mod routes;

use routes::find_room;

pub fn configure_search_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/search").route("/room", web::get().to(find_room)));
}
