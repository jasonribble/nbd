use std::env;

mod commander;
mod db;
mod errors;
mod models;
mod utils;

use clap::Parser;
use commander::{Cli, Commands};
use db::{ContactRepo, PostgresContactRepo};
use models::{Contact, ContactBuilder};
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

            let id = contact_repo.save(contact).await?;

            println!("Successfully saved contact {id}");
        }
        Commands::Edit(value) => {
            let contact = ContactBuilder::new(value.id)
                .set_email(value.email.clone())
                .set_first_name(value.first_name.clone())
                .set_last_name(value.last_name.clone())
                .set_phone_number(value.phone_number.clone())
                .set_display_name(value.display_name.clone());

            let _ = contact_repo.update(contact).await;
        }
        Commands::Show => {
            let contacts = contact_repo.get_all().await?;

            println!("{contacts:?}")
        }
        Commands::Get(value) => {
            let id = value.id;

            let contact = contact_repo.get_by_id(id).await?;

            println!("{contact:?}");
        }
    }

    Ok(())
}
