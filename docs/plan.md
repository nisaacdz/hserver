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
└── migrator/       # Standalone binary for DB ops
    ├── Cargo.toml
    └── src/main.rs
```

**Root `Cargo.toml`**:

```toml
[workspace]
members = ["api", "core", "infrastructure", "migrator"]
resolver = "2"
```

-----

### **2. Database Schema (The "Killer Feature")**

We will use **PostgreSQL Exclusion Constraints** to mathematically guarantee no double-bookings. We are adopting a **Polymorphic Block Pattern** where a central `blocks` table handles the timeline and constraints, while specific tables (`bookings`, `housekeeping`, `maintenance`) hold the details.

**SQL Migration Strategy:**

```sql
-- 1. Enable btree_gist to allow mixing Scalar (UUID) and Range types in one index
CREATE EXTENSION IF NOT EXISTS btree_gist;

-- 2. Room Classes Table
CREATE TABLE room_classes (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name TEXT NOT NULL,
    base_price DECIMAL(10, 2) NOT NULL,
    features JSONB NOT NULL DEFAULT '{}',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- 3. Rooms Table
CREATE TABLE rooms (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    number TEXT NOT NULL,
    class_id UUID NOT NULL REFERENCES room_classes(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- 4. Blocks Table (Base Table)
-- Handles the "Space-Time" uniqueness.
CREATE TABLE blocks (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    room_id UUID NOT NULL REFERENCES rooms(id),
    
    block_type TEXT NOT NULL, -- 'BOOKING', 'HOUSEKEEPING', 'MAINTENANCE'
    stay_period TSRANGE NOT NULL, -- Time Range [start, end)
    
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    -- THE GUARD RAIL:
    -- Prevents overlap only if room_id is the same.
    -- We can add a 'cancelled' flag here if we want soft-deletes, 
    -- but for now, deleting the row removes the block.
    CONSTRAINT no_double_blocks EXCLUDE USING GIST (
        room_id WITH =,
        stay_period WITH &&
    )
);

-- 5. Bookings Table (Child of Blocks)
CREATE TABLE bookings (
    block_id UUID PRIMARY KEY REFERENCES blocks(id) ON DELETE CASCADE,
    guest_id UUID NOT NULL, -- Links to Users table
    status TEXT NOT NULL DEFAULT 'CONFIRMED', -- 'CONFIRMED', 'CHECKED_IN', 'CANCELLED'
    payment_status TEXT NOT NULL DEFAULT 'PENDING'
);

-- 6. Housekeeping Table (Child of Blocks)
CREATE TABLE housekeeping (
    block_id UUID PRIMARY KEY REFERENCES blocks(id) ON DELETE CASCADE,
    instructions TEXT,
    status TEXT NOT NULL DEFAULT 'PENDING' -- 'PENDING', 'IN_PROGRESS', 'COMPLETED'
);

-- 7. Maintenance Table (Child of Blocks)
CREATE TABLE maintenance (
    block_id UUID PRIMARY KEY REFERENCES blocks(id) ON DELETE CASCADE,
    reason TEXT NOT NULL,
    severity TEXT NOT NULL -- 'ROUTINE', 'URGENT'
);
```

-----

### **3. Implementation Sprints (The Roadmap)**

This order minimizes risk by building the foundational data integrity first.

#### **Sprint 1: The Iron Foundation**

*   **Goal:** Core database tables and safe concurrency.
*   **Tasks:**
    1.  Set up the Workspace and Docker Compose (Postgres 15+).
    2.  Implement `diesel_async` + `deadpool` boilerplate.
    3.  Write SQL migrations for `users`, `room_classes`, `rooms`, `blocks`, `bookings`, `housekeeping`, and `maintenance`.
    4.  **Critical:** Write a unit test that attempts to insert two overlapping blocks (e.g., a Booking and a Housekeeping task). *Ensure the database rejects the second one.*

#### **Sprint 2: Inventory & Classification**

*   **Goal:** Allow staff to define what they are selling.
*   **Tasks:**
    1.  Implement JWT Authentication (Middleware).
    2.  Create **Room Class APIs**: `POST /api/admin/classes`, `PUT /api/admin/classes/{id}`.
    3.  Create **Room APIs**: `POST /api/admin/rooms`.
    4.  Implement **Availability Search**: Query `rooms` joined with `room_classes`, filtering out those with overlapping `blocks`.

#### **Sprint 3: The Block Engine (Polymorphic)**

*   **Goal:** Managing time on the calendar.
*   **Tasks:**
    1.  Implement `POST /bookings` (Transactional: Insert into `blocks` then `bookings`).
    2.  Implement `POST /admin/housekeeping` (Transactional: Insert into `blocks` then `housekeeping`).
    3.  Implement `POST /admin/maintenance` (Transactional: Insert into `blocks` then `maintenance`).
    4.  Implement `POST /bookings/{id}/cancel` (Deletes or updates the block).

#### **Sprint 4: Public Facade & Automation**

*   **Goal:** Customer facing endpoints and automated rules.
*   **Tasks:**
    1.  Read-only public APIs (`GET /public/availability`).
    2.  **Auto-Block Logic**: When a booking is created, optionally trigger a "Housekeeping" block creation for N hours after checkout.
    3.  Integration with Payment Gateway.