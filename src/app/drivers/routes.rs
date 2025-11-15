use actix_web::{web, HttpResponse};

async fn healthcheck() -> HttpResponse {
    HttpResponse::Ok().body("OK")
}

pub fn api(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .route("/healthcheck", web::get().to(healthcheck))
    );
}
