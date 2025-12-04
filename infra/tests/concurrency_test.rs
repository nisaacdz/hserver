#[cfg(test)]
mod tests {
    use bigdecimal::BigDecimal;
    use chrono::{TimeZone, Utc};
    use diesel_async::RunQueryDsl;
    use std::collections::Bound;

    use config::{Config, File};
    use uuid::Uuid;

    use app::AppSettings;
    use infra::db;
    use infra::models::{NewBlock, NewRoom, NewRoomClass};
    use infra::schema::{blocks, room_classes, rooms};

    #[tokio::test]
    async fn test_double_booking_prevention() {
        let app_config = {
            dotenvy::from_filename(".env.test").ok();

            let app_config: AppSettings = Config::builder()
                .add_source(File::with_name("../config/default"))
                .add_source(config::Environment::with_prefix("APP").separator("__"))
                .build()
                .expect("Failed to build configuration")
                .try_deserialize()
                .expect("Failed to deserialize configuration");

            app_config
        };

        let pool =
            db::init_pool(&app_config.database).expect("Failed to initialize pg connection pool");

        let mut conn = pool.get().await.unwrap();

        // Clean up before test
        diesel::sql_query("TRUNCATE bookings, blocks, rooms, room_classes, users CASCADE")
            .execute(&mut conn)
            .await
            .ok();

        // 1. Create Room Class
        let class_id = Uuid::new_v4();
        let new_class = NewRoomClass {
            id: Some(class_id),
            name: "Standard",
            base_price: BigDecimal::from(100),
        };

        diesel::insert_into(room_classes::table)
            .values(&new_class)
            .execute(&mut conn)
            .await
            .unwrap();

        // 2. Create Room
        let room_id = Uuid::new_v4();
        let new_room = NewRoom {
            id: Some(room_id),
            label: "101",
            class_id,
        };

        diesel::insert_into(rooms::table)
            .values(&new_room)
            .execute(&mut conn)
            .await
            .unwrap();

        // 3. Attempt Overlapping Blocks

        // Block 1: Jan 1 to Jan 5
        let start1 = Utc.with_ymd_and_hms(2024, 1, 1, 14, 0, 0).unwrap();
        let end1 = Utc.with_ymd_and_hms(2024, 1, 5, 10, 0, 0).unwrap();

        let block1 = NewBlock {
            id: None,
            room_id,
            interval: (Bound::Included(start1), Bound::Excluded(end1)),
        };

        let result1 = diesel::insert_into(blocks::table)
            .values(&block1)
            .execute(&mut conn)
            .await;

        assert!(result1.is_ok(), "First block should succeed");

        // Block 2: Jan 4 to Jan 6 (Overlaps Jan 4-5)
        let start2 = Utc.with_ymd_and_hms(2024, 1, 4, 14, 0, 0).unwrap();
        let end2 = Utc.with_ymd_and_hms(2024, 1, 6, 10, 0, 0).unwrap();

        let block2 = NewBlock {
            id: None,
            room_id,
            interval: (Bound::Included(start2), Bound::Excluded(end2)),
        };

        let result2 = diesel::insert_into(blocks::table)
            .values(&block2)
            .execute(&mut conn)
            .await;

        assert!(result2.is_err(), "Second block should fail due to overlap");
    }
}
