# hserver

A Rust hotel management system built with **Actix-Web** and **Diesel-Async**, designed around database-enforced booking integrity using PostgreSQL exclusion constraints.

## What It Does

This is a backend server for hotel room management. The core idea is simple but powerful: **the database itself guarantees no double-bookings**. Using PostgreSQL's exclusion constraints, the system mathematically prevents overlapping reservations, housekeeping schedules, and maintenance windows—no race conditions, no application-level locks.

## Quick Start

```sh
# Setup environment
sh ./scripts/copy-env.sh

# Start services
docker compose up -d

# Verify
curl http://localhost:8080/api/healthcheck
# => OK
```

## Architecture

The codebase is a **Cargo workspace** with four crates, organized by responsibility:

```
hserver/
├── app/          # Domain layer
├── infra/        # Implementation layer
├── api/          # HTTP layer
└── migrator/     # Database migrations
```

### `app` — Domain Layer

Contains the **what**, not the how. This is where you define:

- **Types & DTOs** — Request/response structures, domain models
- **Traits & Contracts** — Abstract interfaces for services
- **Business Rules** — Validation logic, specifications
- **API Definitions** — Route structures and OpenAPI schemas (via `utoipa`)


### `infra` — Implementation Layer

Contains the **how**. This is where abstractions meet reality:

- **Database Schema** — Diesel schema definitions and models
- **Repositories** — Async database operations with `diesel-async`
- **Service Implementations** — Concrete implementations of `app` traits
- **External Integrations** — Third-party service adapters

The `infra` crate depends on `app` and implements its contracts. All PostgreSQL-specific logic, connection pooling (`deadpool`), and query building lives here.

### `api` — HTTP Layer

Wires everything together for the web:

- **Route Handlers** — Actix-Web endpoints
- **Middleware** — Authentication, tracing, error handling
- **Request/Response Mapping** — HTTP to domain translation
- **Swagger UI** — API documentation via `utoipa-swagger-ui`

### `migrator` — Database Migrations

Standalone binary for running Diesel migrations. Keeps migration tooling separate from the main application.

## Key Design Decision: Polymorphic Blocks

The booking system uses a **Polymorphic Block Pattern**. A central `blocks` table owns the timeline and enforces the no-overlap constraint via PostgreSQL's `EXCLUDE USING GIST`. Child tables (`bookings`, `housekeeping`, `maintenance`) hold type-specific details.

```sql
-- The guard rail: prevents any overlapping blocks on the same room
CONSTRAINT no_double_blocks EXCLUDE USING GIST (
    room_id WITH =,
    stay_period WITH &&
)
```

This means:
- **Bookings, housekeeping, and maintenance all compete for the same timeline**
- **The database rejects invalid state—the application doesn't need to check**
- **Race conditions are impossible at the data layer**

## Tech Stack

| Layer | Technology |
|-------|------------|
| Runtime | Rust 2024 Edition |
| Web Framework | Actix-Web 4.12 |
| Database | PostgreSQL 15+ |
| ORM | Diesel 2.3 + diesel-async 0.7 |
| Connection Pool | Deadpool |
| Auth | Argon2 (password), ChaCha20-Poly1305 (sessions) |
| API Docs | utoipa + Swagger UI |
| Tracing | tracing + tracing-subscriber |

## Development

```sh
# Run E2E tests
APIURL=http://localhost:8080/api sh e2e/run-api-tests.sh

# Check database connectivity
curl http://localhost:8080/api/tags
# => {"tags":[]}
```

## License

MIT
