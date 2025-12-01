use actix_web::web;

pub mod dtos;
pub mod errors;
pub mod routes;

use routes::{find_room, get_room_availability, get_room_classes, get_room_details};

use crate::auth::AuthMiddleware;

pub fn configure_rooms_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/rooms")
            .route("/find", web::get().to(find_room))
            .route("/classes", web::get().to(get_room_classes))
            .route(
                "/{id}",
                web::get().to(get_room_details).wrap(AuthMiddleware),
            )
            .route(
                "/{id}/availability",
                web::get().to(get_room_availability).wrap(AuthMiddleware),
            ),
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::auth::{SessionUser, TokenEngine, generate_auth_cookie};
    use actix_web::{App, test, web};
    use bigdecimal::BigDecimal;
    use config::{Config, File};
    use diesel_async::RunQueryDsl;
    use domain::AppConfig;
    use infrastructure::db;
    use infrastructure::models::{NewRoom, NewRoomClass};
    use infrastructure::schema::{room_classes, rooms};
    use uuid::Uuid;

    fn get_test_config() -> AppConfig {
        dotenvy::dotenv().ok();
        let run_mode = std::env::var("RUN_MODE").unwrap_or("development".to_string());

        Config::builder()
            .add_source(File::with_name("config/default").required(false))
            .add_source(File::with_name("../config/default").required(false))
            .add_source(File::with_name(&format!("config/{}", run_mode)).required(false))
            .add_source(File::with_name(&format!("../config/{}", run_mode)).required(false))
            .add_source(config::Environment::with_prefix("APP").separator("__"))
            .build()
            .expect("Failed to build configuration")
            .try_deserialize()
            .expect("Failed to deserialize configuration")
    }

    async fn get_test_pool(config: &AppConfig) -> db::DbPool {
        db::init_pool(&config.database).expect("Failed to init pool")
    }

    async fn setup_test_data(pool: &db::DbPool) -> (Uuid, Uuid) {
        let mut conn = pool.get().await.expect("Failed to get conn");

        let class_id = Uuid::new_v4();
        let new_class = NewRoomClass {
            id: Some(class_id),
            name: "Test Class",
            base_price: BigDecimal::from(100),
        };

        diesel::insert_into(room_classes::table)
            .values(&new_class)
            .execute(&mut conn)
            .await
            .expect("Failed to insert room class");

        let room_id = Uuid::new_v4();
        let new_room = NewRoom {
            id: Some(room_id),
            label: "Test Room",
            class_id,
        };

        diesel::insert_into(rooms::table)
            .values(&new_room)
            .execute(&mut conn)
            .await
            .expect("Failed to insert room");

        (room_id, class_id)
    }

    #[actix_web::test]
    async fn test_get_room_classes() {
        let config = get_test_config();
        let pool = get_test_pool(&config).await;
        let token_engine = TokenEngine::new(&config.security);

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(pool.clone()))
                .app_data(web::Data::new(token_engine.clone()))
                .configure(configure_rooms_routes),
        )
        .await;

        let req = test::TestRequest::get().uri("/rooms/classes").to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    async fn test_find_room() {
        let config = get_test_config();
        let pool = get_test_pool(&config).await;
        let token_engine = TokenEngine::new(&config.security);

        // Ensure at least one room exists
        setup_test_data(&pool).await;

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(pool.clone()))
                .app_data(web::Data::new(token_engine.clone()))
                .configure(configure_rooms_routes),
        )
        .await;

        let start = chrono::Utc::now();
        let end = start + chrono::Duration::days(1);
        let start_str = start.to_rfc3339().replace("+", "%2B");
        let end_str = end.to_rfc3339().replace("+", "%2B");
        let query = format!("start={}&end={}", start_str, end_str);

        let req = test::TestRequest::get()
            .uri(&format!("/rooms/find?{}", query))
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    async fn test_get_room_details() {
        let config = get_test_config();
        let pool = get_test_pool(&config).await;
        let token_engine = TokenEngine::new(&config.security);

        let (room_id, _) = setup_test_data(&pool).await;

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(pool.clone()))
                .app_data(web::Data::new(token_engine.clone()))
                .configure(configure_rooms_routes),
        )
        .await;

        let user = SessionUser {
            id: Uuid::new_v4(),
            staff_id: Some(Uuid::new_v4()), // Staff access might be needed? Routes don't check role yet, just auth.
            email: "test@test.com".to_string(),
        };
        let cookie = generate_auth_cookie(&token_engine, user).unwrap();

        let req = test::TestRequest::get()
            .uri(&format!("/rooms/{}", room_id))
            .cookie(cookie)
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    async fn test_get_room_availability() {
        let config = get_test_config();
        let pool = get_test_pool(&config).await;
        let token_engine = TokenEngine::new(&config.security);

        let (room_id, _) = setup_test_data(&pool).await;

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(pool.clone()))
                .app_data(web::Data::new(token_engine.clone()))
                .configure(configure_rooms_routes),
        )
        .await;

        let user = SessionUser {
            id: Uuid::new_v4(),
            staff_id: Some(Uuid::new_v4()), // Availability check requires staff_id in routes.rs:22
            email: "test@test.com".to_string(),
        };
        let cookie = generate_auth_cookie(&token_engine, user).unwrap();

        let start = chrono::Utc::now();
        let end = start + chrono::Duration::days(1);
        let start_str = start.to_rfc3339().replace("+", "%2B");
        let end_str = end.to_rfc3339().replace("+", "%2B");
        let query = format!("start={}&end={}", start_str, end_str);

        let req = test::TestRequest::get()
            .uri(&format!("/rooms/{}/availability?{}", room_id, query))
            .cookie(cookie)
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }
}
