use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use hserver::config::{ApplicationSettings, ServerSettings, Settings};
use hserver::db;
use serde::Serialize;
use utoipa::OpenApi;
use utoipa::ToSchema;
use utoipa_swagger_ui::SwaggerUi;

/// Health check response
#[derive(Serialize, ToSchema)]
struct HealthCheckResponse {
    /// Status of the service
    status: String,
    /// Service name
    service: String,
}

/// Configuration response containing server and application settings
#[derive(Serialize, ToSchema)]
struct ConfigResponse {
    /// Application settings
    application: ApplicationSettings,
    /// Server settings
    server: ServerSettings,
}

#[derive(OpenApi)]
#[openapi(
    paths(health_check, get_config),
    components(schemas(
        HealthCheckResponse,
        ConfigResponse,
        ApplicationSettings,
        ServerSettings
    )),
    tags(
        (name = "health", description = "Health check endpoints"),
        (name = "config", description = "Configuration endpoints")
    ),
    info(
        title = "Hotel Management System API",
        version = "0.1.0",
        description = "REST API for the Hotel Management System",
        contact(
            name = "API Support",
            url = "https://github.com/nisaacdz/hserver"
        )
    )
)]
struct ApiDoc;

/// Health check endpoint
///
/// Returns the current health status of the service
#[utoipa::path(
    get,
    path = "/health",
    tag = "health",
    responses(
        (status = 200, description = "Service is healthy", body = HealthCheckResponse)
    )
)]
async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(HealthCheckResponse {
        status: "ok".to_string(),
        service: "Hotel Management System".to_string(),
    })
}

/// Get configuration endpoint
///
/// Returns the current server and application configuration
#[utoipa::path(
    get,
    path = "/config",
    tag = "config",
    responses(
        (status = 200, description = "Configuration retrieved successfully", body = ConfigResponse)
    )
)]
async fn get_config() -> impl Responder {
    let settings = Settings::get();
    HttpResponse::Ok().json(ConfigResponse {
        application: settings.application.clone(),
        server: settings.server.clone(),
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize environment variables from .env file
    dotenv::dotenv().ok();

    // Initialize logger
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // Get configuration (lazily initialized on first access)
    let settings = Settings::get();

    log::info!(
        "Starting {} on {}:{}",
        settings.application.name,
        settings.server.host,
        settings.server.port
    );

    // Create database pool
    let pool = db::create_pool(&settings.database.url, settings.database.max_connections);
    log::info!(
        "Database pool created with max {} connections",
        settings.database.max_connections
    );

    // Store server address before moving settings
    let server_host = settings.server.host.clone();
    let server_port = settings.server.port;

    // Create OpenAPI documentation
    let openapi = ApiDoc::openapi();

    // Start HTTP server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/health", web::get().to(health_check))
            .route("/config", web::get().to(get_config))
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", openapi.clone()),
            )
    })
    .bind((server_host, server_port))?
    .run()
    .await
}
