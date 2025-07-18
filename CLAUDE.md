# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

NBD (No Big Deal) is a privacy-first, offline-first personal contact manager written in Rust. It's a CLI application that manages contacts in a SQLite database with support for CRUD operations, CSV imports, and relationship tracking.

## Development Setup

### Prerequisites
- Rust/Cargo
- sqlx-cli (`cargo install sqlx-cli`)
- SQLite database

### Initial Setup
1. Copy environment file: `cp .env.example .env`
2. Create database: `sqlx db create`
3. Run migrations: `sqlx migrate run`

### Common Commands

**Build and run:**
```bash
# Build the project
cargo build

# Run the CLI
cargo run -- <command>
# Example: cargo run -- create --first-name test --last-name last --email test@test.com

# Run tests
cargo test

# Run CLI tests specifically
cargo test --test cli

# Run with test database cleanup
cargo test -- --test-threads=1
```

**Database operations:**
```bash
# Create new migration
sqlx migrate add <migration_name>

# Run pending migrations
sqlx migrate run

# Revert last migration
sqlx migrate revert
```

## Architecture

### Project Structure
- `src/cli/` - CLI interface and command handling
  - `main.rs` - Entry point, dependency injection
  - `commander.rs` - Command definitions using clap
  - `actions.rs` - Business logic for CLI commands
- `src/db/` - Database layer
  - `connection.rs` - Database connection wrapper
  - `contact_repo.rs` - Contact repository implementation
  - `metadata_repo.rs` - Metadata repository implementation
- `src/models/` - Data models
  - `contact.rs` - Contact model with builder pattern
  - `metadata.rs` - Contact metadata model
  - `profile.rs` - Profile-related models
- `src/utils/` - Utility modules
  - `csv.rs` - CSV import/export functionality
  - `validation.rs` - Input validation
  - `errors.rs` - Error handling

### Key Patterns

**Repository Pattern:** Database operations are abstracted through repository traits implemented by the `Connection` struct.

**Builder Pattern:** Contact creation uses a builder pattern for flexible construction with optional fields.

**Async/Await:** All database operations are asynchronous using tokio runtime.

**Error Handling:** Uses `anyhow` for error propagation throughout the application.

### Database Schema
- `contacts` - Main contact information
- `contacts_metadata` - Additional metadata with foreign key relationship
- Migrations are in `migrations/` directory

### Testing
- Integration tests in `tests/cli.rs` use `assert_cmd` for CLI testing
- Database tests use `serial_test` to prevent race conditions
- Test utilities in `test_utils/` workspace member
- CLI tests clean the database between runs

## CLI Commands
- `init` - Initialize contact book
- `create` - Create new contact
- `edit` - Edit existing contact by ID
- `show` - Display all contacts
- `get` - Get specific contact
- `delete` - Delete contact by ID
- `import` - Import contacts from CSV file