# No Big Deal - A Connection Cultivator

[![Commit Phase](https://github.com/jasonribble/nbd/actions/workflows/ci.yml/badge.svg)](https://github.com/jasonribble/nbd/actions/workflows/ci.yml)

## Description

This is a personal contact management to help people create and maintain thriving relationship in their life.

## Motivation

The world needs a privacy-first, offline-first, personal contact manager.

## Development

This Rust project requires the following:

- Cargo
- sqlx-cli

You can also use the nix flake. Install nix through the [nix-installer](https://github.com/DeterminateSystems/nix-installer). Once installed, run:

`$ nix develop`

## Setup

1. Copy the `.env.example` .env

```
cp .env.example .env
```

Optionally update environment variables

2. Create the database.

```
sqlx db create
```

3. Run sql migrations

```
sqlx migrate run
```

## Usage

Create a contact

```
Usage: nbd-cli <COMMAND>

Commands:
  create  Create a contact
  edit    Edit a contact by ID
  show    Get all contacts
  get     Get a contact
  delete  Delete a contact
  import  Import contact via CSV
  help    Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

For example

```
cargo run create --first-name test --last-name last --email test@ttest.com --phone-number 123-231-1122 --birthday 1970-01-01
```

Then, you can see the contact using the `show` command

```
cargo run show
```

Edit a contact

```
Arguments:
  <ID>  ID of contact to edit

Options:
  -f, --first-name <First Name>
  -l, --last-name <Last Name>
  -d, --display-name <Display Name>
  -e, --email <EMAIL>
  -p, --phone-number <Phone>
  -h, --help                         Print help
```

For example

```
cargo run edit 1 -f Jason
```

## Cleanup

To destroy the database, delete `contacts.db`
