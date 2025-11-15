# hserver

A modern Rust web server using `actix-web` 4.x and `diesel-async` with PostgreSQL.

## Overview

This is a modernized version of the project with:
- **Async database operations** using `diesel-async` 0.7
- **Connection pooling** with `deadpool`  
- **Updated dependencies** - All dependencies updated to latest stable versions
- **Simplified structure** - Clean architecture with minimal boilerplate

## Getting Started

```zsh
# ready
$ sh ./scripts/copy-env.sh

# start
$ docker compose up -d

# healthcheck
$ curl http://localhost:8080/api/healthcheck
# => OK
```

```sh
# Check app can connect to DB
$ curl http://localhost:8080/api/tags
# => {"tags":[]}

# Check app can insert data into DB
curl -X POST http://localhost:8080/api/users -d '{"user": {"email": "a@a.a", "username": "a", "password": "a" }}' -H "Content-Type: application/json"
```

## E2E Test

Running E2E tests using [POSTMAN scripts](https://github.com/gothinkster/realworld/tree/main/api) on CI

```zsh
# run e2e
$ APIURL=http://localhost:8080/api zsh e2e/run-api-tests.sh
```

## Tech Stack

- **Rust Edition 2021**
- **Actix-Web 4.11** - Fast, pragmatic web framework
- **Diesel 2.3** + **diesel-async 0.7** - Async ORM and query builder
- **PostgreSQL** - Database
- **Deadpool** - Async connection pooling
- **Chrono 0.4** - Date/time handling
- **Serde 1.0** - Serialization
- **jsonwebtoken 10.2** - JWT auth (with rust_crypto)
- **bcrypt 0.17** - Password hashing
- **dotenvy 0.15** - Environment configuration

## Architecture

Clean architecture with async/await support:

```
/src
  /app
    /drivers         - Middleware & routing
    /features        - Feature modules (user, etc.)
      /user
        entities.rs  - Data models
        repository.rs - Async database operations
  /utils             - Database pool, DI, helpers
  schema.rs          - Diesel schema
  error.rs           - Error types (AppError pattern)
```

### Key Patterns

- **AppError** - Unified error handling with HTTP response mapping
- **Async repositories** - All database operations use async/await
- **Connection pooling** - Deadpool for efficient async connection management
- **Clean separation** - Routes → Repositories → Database

## LICENSE

MIT
