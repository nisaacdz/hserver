pub mod auth;
pub mod openapi;
pub mod v1;

use crate::openapi::ApiDoc;
use crate::{auth::TokenEngine, v1::configure_v1_routes};
use actix_web::{App, HttpServer, web};
use app::AppSettings;
use diesel_async::AsyncPgConnection;
use diesel_async::pooled_connection::deadpool::Pool;

use tracing_actix_web::TracingLogger;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

pub async fn run(pool: Pool<AsyncPgConnection>, settings: AppSettings) -> std::io::Result<()> {
    let token_engine = TokenEngine::new(&settings.security);

    println!(
        "Starting server at {}:{}",
        settings.server.host, settings.server.port
    );

    let pool = web::Data::new(pool.clone());
    let token_engine = web::Data::new(token_engine.clone());
    let app_settings = web::Data::new(settings.clone());

    HttpServer::new(move || {
        let pool = pool.clone();
        let token_engine = token_engine.clone();
        let app_settings = app_settings.clone();
        App::new()
            .wrap(TracingLogger::default())
            .app_data(pool)
            .app_data(app_settings)
            .app_data(token_engine)
            .configure(|cfg| {
                cfg.service(web::scope("/api").configure(configure_v1_routes))
                    .service(
                        SwaggerUi::new("/swagger-ui/{_:.*}")
                            .url("/api-docs/openapi.json", ApiDoc::openapi()),
                    );
            })
    })
    .bind((settings.server.host.as_str(), settings.server.port))?
    .run()
    .await
}
