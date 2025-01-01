# Rust Axum PostgreSQL API

A modern, high-performance REST API built with Rust, using Axum web framework and PostgreSQL database.

## Prerequisites

1. Rust (1.75.0 or later)
2. PostgreSQL (14.0 or later)

## Getting Started
clone the repository
```bash
git clone https://github.com/ochomoswill/rust-axum-postgres-api.git
cd rust-axum-postgres-api
```
create a `.env` file on the root directory

```dotenv
DATABASE_URL=postgres://{database_username}:{database_password}@localhost:5432/{database_name}
SERVER_PORT=9000
```

install dependencies
```bash
cargo install
```

run app in hot reload:
```bash
cargo watch -q -c -w src/ -x run
```

run database migrations
```bash
sqlx migrate run
```

In case, you need to undo the migration
```bash
sqlx migrate revert
```

## References
[Rust CRUD API Example with Axum and PostgreSQL](https://codevoweb.com/rust-crud-api-example-with-axum-and-postgresql/)