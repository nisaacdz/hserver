#[cfg(test)]
mod tests {
    use diesel::prelude::*;
    use diesel_async::{AsyncPgConnection, RunQueryDsl};
    use uuid::Uuid;
    use std::env;
    use tokio_postgres_rustls::MakeRustlsConnect;
    use rustls::ClientConfig;

    #[tokio::test]
    async fn test_double_booking_prevention() {
        dotenvy::from_filename(".env").ok();
        
        let database_url = env::var("APP__DATABASE__URL")
            .or_else(|_| env::var("DATABASE_URL"))
            .expect("DATABASE_URL must be set");

        // Setup TLS
        let mut root_store = rustls::RootCertStore::empty();
        root_store.extend(
            webpki_roots::TLS_SERVER_ROOTS
                .iter()
                .cloned()
        );
        let config = ClientConfig::builder()
            .with_root_certificates(root_store)
            .with_no_client_auth();
        let tls = MakeRustlsConnect::new(config);

        // Connect manually using tokio-postgres
        let (client, connection) = tokio_postgres::connect(&database_url, tls)
            .await
            .expect("Failed to connect to database");

        // Spawn the connection task
        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("connection error: {}", e);
            }
        });

        // Convert to AsyncPgConnection
        let mut conn = AsyncPgConnection::try_from(client).await.expect("Failed to convert client");

        // Clean up before test
        // Note: Truncate might fail if tables don't exist, so we wrap in Result
        let _ = diesel::sql_query("TRUNCATE bookings, rooms, users CASCADE")
            .execute(&mut conn)
            .await;

        // Enable extension
        diesel::sql_query("CREATE EXTENSION IF NOT EXISTS btree_gist;")
            .execute(&mut conn)
            .await
            .unwrap();

        // Run Migrations (Manually for test)
        let up_sql = std::fs::read_to_string("../migrations/2025-11-21-013625-0000_create_core_tables/up.sql")
            .expect("Failed to read migration file");

        for statement in up_sql.split(';') {
            if statement.trim().is_empty() {
                continue;
            }
            // Ignore errors if objects already exist (simple idempotency)
            let _ = diesel::sql_query(statement)
                .execute(&mut conn)
                .await;
        }

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
