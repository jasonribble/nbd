# Companion Connect

## Description

This is a personal CRM to help people create thriving relationship in their life.

## Development

This Rust project requires the following:

- Docker
- Docker Compose
- Cargo
- sqlx-cli

## Setup

1. Run `docker-compose up -d` to run Postgres in the background.

2. Declare the database URL, either by exporting it:

   ```
   export DATABASE_URL="postgres://postgres:test@localhost/contacts"
   ```

   or by making a `.env` file:

   ```
   cp .env.example .env
   ```

3. Create the database.

   ```
   $ sqlx db create
   ```

4. Run sql migrations

   ```
   $ sqlx migrate run
   ```

## Usage

Add a contact

```
cargo run -- John Test john@example.com 9738978633
```

## Cleanup

To destroy the Postgres database, run:

```
docker-compose down --volumes
```
