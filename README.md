# Rust CRUD with Postgres (SQL) database

## How to run?

**IMPORTANT NOTE:**
This program was developed in VS code.

**Prerequisites:**

- You need to have [Rust compiler](https://www.rust-lang.org/tools/install) installed and set up on your machine.
- Docker desktop needs to be installed to run the Postgres database in a container.
- Docker compose needs to be installed.

**Running program:**

1. Before running the program, create a copy of file `.env.example` and rename it to `.env`.
2. It should have the following contents:
```env
POSTGRES_HOST=127.0.0.1
POSTGRES_PORT=5432
POSTGRES_USER=admin
POSTGRES_PASSWORD=password123
POSTGRES_DB=rust_sqlx

DATABASE_URL=postgresql://admin:password123@localhost:5432/rust_sqlx?schema=public
```
3. Go to the root of the project and run `docker compose up --build` to and wait for it to start the Postgres container.
4. Install the Rust SQLx tools to work with database migrations, run `cargo install sqlx-cli`.
5. Run the migrations `sqlx migrate run`.
6. Run `cargo build` to build and run the rust app using `cargo run`.
7. Navigate to [http://127.0.0.1:8000](http://127.0.0.1:8000). 
8. If everything is OK, the index page of application should be visible.