# Companion Connect

## Description

This is a personal CRM to help people create thriving relationship in their life.

## Development

This Rust project requires the following:

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

Create a contact

```
Usage: connect create [OPTIONS]

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
cargo run create --first-name test --last-name last --email test@ttest.com --phone-number 123-231-1122
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

`cargo run edit 5 -f Jason`

## Cleanup

To destroy the database, delete `contacts.db`

