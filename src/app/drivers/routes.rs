use actix_web::{web, HttpResponse};

use crate::app::features::auth::handlers;

async fn healthcheck() -> HttpResponse {
    HttpResponse::Ok().body("OK")
}

pub fn api(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .route("/healthcheck", web::get().to(healthcheck))
            .service(
                web::scope("/auth")
                    .route("/login", web::post().to(handlers::login))
            )
    );
}
