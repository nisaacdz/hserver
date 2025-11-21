#[cfg(test)]
mod tests {
    use diesel_async::pooled_connection::deadpool::Pool;
    use diesel_async::pooled_connection::AsyncDieselConnectionManager;
    use diesel_async::{AsyncPgConnection, RunQueryDsl};
    use uuid::Uuid;
    use std::env;

    #[tokio::test]
    async fn test_double_booking_prevention() {
        dotenvy::from_filename(".env").ok();
        
        let database_url = env::var("APP__DATABASE__URL")
            .or_else(|_| env::var("DATABASE_URL"))
            .expect("DATABASE_URL must be set");

        let manager = AsyncDieselConnectionManager::<AsyncPgConnection>::new(database_url);
        let pool = Pool::builder(manager).build().unwrap();

        // NOTE: Run `diesel migration run` before running tests
        // Migrations should be handled by diesel_cli, not programmatically
        
        let mut conn = pool.get().await.unwrap();

        // Clean up before test
        diesel::sql_query("TRUNCATE bookings, rooms, users CASCADE")
            .execute(&mut conn)
            .await
            .ok();

        // 3. Insert Data
        // Create a room
        let room_id = Uuid::new_v4();
        diesel::sql_query("INSERT INTO rooms (id, number, room_type, price_per_night) VALUES ($1, '101', 'Standard', 100.00)")
            .bind::<diesel::sql_types::Uuid, _>(room_id)
            .execute(&mut conn)
            .await
            .unwrap();

        // Create a user
        let user_id = Uuid::new_v4();
        diesel::sql_query(
            "INSERT INTO users (id, email, password_hash) VALUES ($1, 'test@test.com', 'hash')",
        )
        .bind::<diesel::sql_types::Uuid, _>(user_id)
        .execute(&mut conn)
        .await
        .unwrap();

        // 4. Attempt Overlapping Bookings

        // Booking 1: Jan 1 to Jan 5
        let result1 = diesel::sql_query(
            "INSERT INTO bookings (room_id, guest_id, stay_period) VALUES ($1, $2, '[2024-01-01 14:00:00, 2024-01-05 10:00:00)')"
        )
        .bind::<diesel::sql_types::Uuid, _>(room_id)
        .bind::<diesel::sql_types::Uuid, _>(user_id)
        .execute(&mut conn)
        .await;

        assert!(result1.is_ok(), "First booking should succeed");

        // Booking 2: Jan 4 to Jan 6 (Overlaps Jan 4-5)
        let result2 = diesel::sql_query(
            "INSERT INTO bookings (room_id, guest_id, stay_period) VALUES ($1, $2, '[2024-01-04 14:00:00, 2024-01-06 10:00:00)')"
        )
        .bind::<diesel::sql_types::Uuid, _>(room_id)
        .bind::<diesel::sql_types::Uuid, _>(user_id)
        .execute(&mut conn)
        .await;

        assert!(
            result2.is_err(),
            "Second booking should fail due to overlap"
        );
    }
}
