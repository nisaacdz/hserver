Here is the complete, actionable architectural setup for your Hotel Management System.

This plan synthesizes the safety of **Rust**, the concurrency of **Actix**, and the data integrity of **PostgreSQL**. It resolves conflicts between your sources by prioritizing **Transactional Integrity** (Postgres Exclusion Constraints) and **Intent-Based API Design**.

### **1. The Workspace Structure**

We will use a **Cargo Workspace** to enforce Clean Architecture. This prevents your API layer (Actix) from bleeding into your business logic.

**File Tree:**

```text
hotel_monolith/
├── Cargo.toml              # Workspace definition
├── .env                    # DATABASE_URL=postgres://...
├── api/                    # HTTP Layer (Actix Web)
│   ├── Cargo.toml
│   └── src/lib.rs
├── core/                   # Pure Domain Logic (Types, Traits)
│   ├── Cargo.toml
│   └── src/lib.rs
├── infrastructure/         # Database Interaction (Diesel Async)
│   ├── Cargo.toml
│   └── src/lib.rs
└── migration_runner/       # Standalone binary for DB ops
    ├── Cargo.toml
    └── src/main.rs
```

**Root `Cargo.toml`**:

```toml
[workspace]
members = ["api", "core", "infrastructure", "migration_runner"]
resolver = "2"
```

-----

### **2. Infrastructure Setup (The Hard Parts)**

This is where `diesel_async` and `deadpool` meet. This setup is often non-trivial; copy this exactly.

**`infrastructure/Cargo.toml`**:

```toml
[dependencies]
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
diesel = { version = "2", features = ["postgres", "chrono", "uuid", "numeric"] }
diesel-async = { version = "0.5", features = ["postgres", "deadpool"] }
deadpool = "0.10" # Manages the async pool
uuid = "1.0"
chrono = "0.4"
# Internal dep
core = { path = "../core" }
```

**`infrastructure/src/db.rs`** (The Connection Pool):

```rust
use diesel_async::pooled_connection::deadpool::Pool;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::AsyncPgConnection;

pub type DbPool = Pool<AsyncPgConnection>;

pub fn init_pool(database_url: &str) -> DbPool {
    let config = AsyncDieselConnectionManager::<AsyncPgConnection>::new(database_url);
    Pool::builder(config)
        .max_size(20) // Tuning: higher for high traffic
        .build()
        .expect("Failed to create DB pool")
}
```

**`infrastructure/src/repo.rs`** (Async Query Example):
*Critical Note:* You must import `RunQueryDsl` from `diesel_async` to get `.await` on queries.

```rust
use diesel::prelude::*; // Standard traits
use diesel_async::RunQueryDsl; // <--- THE MAGIC IMPORT
use crate::db::DbPool;

pub async fn find_room(pool: &DbPool, room_id: uuid::Uuid) -> Result<Option<Room>, diesel::result::Error> {
    let mut conn = pool.get().await.expect("Pool exhausted");
    
    rooms::table
        .find(room_id)
        .first(&mut conn)
        .await // Async execution
        .optional()
}
```

-----

### **3. Database Schema (The "Killer Feature")**

We will use **PostgreSQL Exclusion Constraints** to mathematically guarantee no double-bookings. This is superior to application-side checks.

**SQL Migration:**

```sql
-- 1. Enable btree_gist to allow mixing Scalar (UUID) and Range types in one index
CREATE EXTENSION IF NOT EXISTS btree_gist;

-- 2. Rooms Table
CREATE TABLE rooms (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    number TEXT NOT NULL,
    room_type TEXT NOT NULL, -- e.g. 'Deluxe', 'Standard'
    price_per_night DECIMAL(10, 2) NOT NULL
);

-- 3. Bookings Table with CONSTRAINT
CREATE TABLE bookings (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    room_id UUID NOT NULL REFERENCES rooms(id),
    guest_id UUID NOT NULL, -- Links to Users table
    stay_period TSRANGE NOT NULL, -- Time Range [start, end)
    status TEXT NOT NULL DEFAULT 'CONFIRMED',
    
    -- THE GUARD RAIL:
    -- Prevents overlap only if room_id is the same AND status is not CANCELLED
    CONSTRAINT no_double_bookings EXCLUDE USING GIST (
        room_id WITH =,
        stay_period WITH &&
    ) WHERE (status != 'CANCELLED')
);
```

-----
```rust
use actix_web::{post, web, HttpResponse, Responder};
use infrastructure::db::DbPool;
use core::dtos::CancelRequest;

// POST /bookings/{id}/cancel
#[post("/{id}/cancel")]
pub async fn cancel_booking(
    pool: web::Data<DbPool>,
    path: web::Path<uuid::Uuid>,
    payload: web::Json<CancelRequest> // Reason for cancellation
) -> impl Responder {
    let booking_id = path.into_inner();
    
    // In a real app, call a Service in `core` that handles:
    // 1. DB update (status -> CANCELLED)
    // 2. Stripe Refund API
    // 3. Email Notification
    
    HttpResponse::Ok().json(serde_json::json!({ "status": "cancelled" }))
}
```

-----

### **5. Implementation Sprints (The Roadmap)**

This order minimizes risk by building the foundational data integrity first.

#### **Sprint 1: The Iron Foundation**

  * **Goal:** Core database tables and safe concurrency.
  * **Tasks:**
    1.  Set up the Workspace and Docker Compose (Postgres 15+).
    2.  Implement `diesel_async` + `deadpool` boilerplate.
    3.  Write SQL migrations for `users`, `rooms`, and `bookings`.
    4.  **Critical:** Write a unit test using `testcontainers` that attempts to insert two overlapping bookings. *Ensure the database rejects the second one.*

#### **Sprint 2: Inventory & Pricing (Internal First)**

  * **Goal:** Allow staff to manage rooms.
  * **Tasks:**
    1.  Implement JWT Authentication (Middleware).
    2.  Create "Back Office" APIs: `POST /api/admin/rooms`, `PATCH /api/admin/rooms/{id}/pricing`.
    3.  Implement the **Availability Search**: A complex query using logic: `(All Rooms) EXCEPT (Booked Rooms for Date Range)`.

#### **Sprint 3: The Transaction Engine**

  * **Goal:** Booking lifecycle.
  * **Tasks:**
    1.  Implement `POST /bookings` (Create).
    2.  Implement `POST /bookings/{id}/check-in` (Intent: Validates payment, assigns physical key card).
    3.  Implement `POST /bookings/{id}/cancel` (Intent: Releases strict constraint, triggers logic).

#### **Sprint 4: Public Facade**

  * **Goal:** Customer facing endpoints.
  * **Tasks:**
    1.  Read-only public APIs (`GET /public/availability`).
    2.  Performance tuning: Ensure `tsrange` queries are hitting the GiST index.
    3.  Integration with Payment Gateway (Stripe/PayPal) webhook handlers.