use std::env;

mod commander;
mod db;
mod errors;
mod models;
mod utils;

use clap::Parser;
use commander::{Cli, Commands};
use db::{ContactRepo, PostgresContactRepo};
use models::Contact;
use sqlx::PgPool;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    let pool = PgPool::connect(&env::var("DATABASE_URL")?).await?;

    let contact_repo = PostgresContactRepo::new(pool);

    let cli = Cli::parse();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Commands::Create(value) => {
            let contact = Contact::new(
                value.first_name.as_deref().unwrap_or(""),
                value.last_name.as_deref().unwrap_or(""),
                value.email.as_deref().unwrap_or(""),
                value.phone_number.as_deref().unwrap_or(""),
            );

            let contact = contact.unwrap();

            let _ = contact_repo.save(contact).await?;

            println!("Successfully saved contact.");
        }
        Commands::Edit(value) => {
            println!("This is an edit command {value:?}");
        }
    }

    Ok(())
}
