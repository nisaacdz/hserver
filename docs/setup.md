# Project Setup

This project is a Hotel Management System built with Rust, Actix Web, Diesel Async, and PostgreSQL.

## Prerequisites

- **Rust**: Ensure you have the latest stable version of Rust installed.
- **PostgreSQL**: You need a running PostgreSQL instance (version 15+ recommended).
- **Docker** (Optional): For running the database easily.

## Configuration

The project uses the `config` crate for configuration management.
Default settings are in `config/default.toml`.
You can override these settings using environment variables prefixed with `APP__`.

Example `.env` file:
```bash
APP__DATABASE__URL=postgres://user:password@localhost/hserver_db
APP__SERVER__PORT=8081
```

## Running the Project

1.  **Database Setup**:
    Ensure your database is running and the URL is correctly set in `config/default.toml` or environment variables.

2.  **Run the API**:
    ```bash
    cargo run -p api
    ```

3.  **Run Migrations** (Future):
    ```bash
    cargo run -p migrator
    ```

## Workspace Structure

-   `api`: The HTTP layer (Actix Web).
-   `core`: Domain logic, types, and configuration structs.
-   `infrastructure`: Database interaction (Diesel Async).
-   `migrator`: Standalone binary for database operations.
