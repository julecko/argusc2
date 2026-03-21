# ArgusC2

ArgusC2 is a command and control server with an administrative panel for
managing connected agents.

## Requirements

-   Rust
-   Cargo
-   MySQL or MariaDB
-   sqlx-cli
-   Docker (optional, for running the database)

Install sqlx-cli:

    cargo install sqlx-cli --no-default-features --features mysql


## Setup

### 1. Create environment file

Copy the example configuration:

    cp .env.example .env

Edit `.env` and configure your database connection and other settings.

Example:

    DATABASE_URL=mysql://user:password@localhost:3306/argusc2


### 2. Setup database

You can run the database using Docker:

    docker compose up -d

Or configure and run your own MySQL server and update `DATABASE_URL`
accordingly.


### 3. Run database migrations

Apply migrations using sqlx:

    sqlx migrate run

### 4. Generate jws secret
In `.env` edit JWS_SECRET to any random value, you can generate for example using this command.

    openssl rand -hex 64


### 5. Start the server

Run the application:

    cargo run


## Development

Re-run migrations after adding new migration files:

    sqlx migrate run
