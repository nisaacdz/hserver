# Hotel Management System (hserver)

A Rust-based Hotel Management System built with Actix-Web and Diesel (async PostgreSQL).

## Features

- **Web Framework**: Actix-Web for high-performance HTTP server
- **Database**: Diesel with async PostgreSQL support using diesel_async
- **Configuration**: Type-safe configuration management with the `config` crate
- **Error Handling**: Custom AppError with status, message, and cause fields
- **Logging**: Structured logging with env_logger

## Prerequisites

- Rust 1.70 or higher
- PostgreSQL 12 or higher
- Diesel CLI (for migrations): `cargo install diesel_cli --no-default-features --features postgres`

## Setup

1. Clone the repository:
   ```bash
   git clone https://github.com/nisaacdz/hserver.git
   cd hserver
   ```

2. Copy the example environment file and configure it:
   ```bash
   cp .env.example .env
   ```

3. Update the database URL in `.env` or `config/default.toml` to match your PostgreSQL setup.

4. Create the database:
   ```bash
   createdb hserver_db
   ```

5. Run the application:
   ```bash
   cargo run
   ```

## Configuration

The application uses a layered configuration system:

1. **Default configuration**: `config/default.toml`
2. **Environment-specific**: `config/{RUN_MODE}.toml` (e.g., `config/production.toml`)
3. **Local overrides**: `config/local.toml` (gitignored)
4. **Environment variables**: Prefix with `APP__` and use `__` as separator

Example environment variable:
```bash
APP_SERVER__PORT=9090
APP_DATABASE__MAX_CONNECTIONS=20
```

## API Endpoints

- `GET /health` - Health check endpoint
- `GET /config` - View current configuration (server and application info)

## Project Structure

```
hserver/
├── src/
│   ├── main.rs         # Application entry point
│   ├── lib.rs          # Library root
│   ├── config.rs       # Configuration management
│   ├── db.rs           # Database connection pool
│   ├── error.rs        # Custom error types
│   ├── models.rs       # Database models
│   └── schema.rs       # Diesel schema
├── config/
│   └── default.toml    # Default configuration
├── migrations/         # Database migrations
├── Cargo.toml          # Dependencies
└── diesel.toml         # Diesel configuration
```

## Development

Build the project:
```bash
cargo build
```

Run tests:
```bash
cargo test
```

Run with logging:
```bash
RUST_LOG=info cargo run
```

## License

MIT