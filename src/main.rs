use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};
mod app;
mod constants;
mod error;
mod schema;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("start hserver...");
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let pool = utils::db::establish_connection();

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(actix_web::web::Data::new(pool.clone()))
            .wrap(app::drivers::middlewares::cors::cors())
            .configure(app::drivers::routes::api)
    })
    .bind(constants::BIND)?
    .run()
    .await
}
