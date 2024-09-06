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

1. Declare the database URL, either by exporting it:

   ```
   export DATABASE_URL="sqlite:contacts.db"
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
cargo run create --first-name test --last-name last --email test@ttest.com --phone-number 123-231-1122
```

## Cleanup

To destroy the database, delete `contacts.db`

