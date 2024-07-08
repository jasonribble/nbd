use std::env;

mod db;
mod errors;
mod models;
mod utils;

use db::{ContactRepo, PostgresContactRepo};
use errors::AppError;
use models::Contact;
use sqlx::PgPool;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    let pool = PgPool::connect(&env::var("DATABASE_URL")?).await?;

    println!("Welcome. You must run a postgres container to have this work");

    let contact_repo = PostgresContactRepo::new(pool);

    let contact = parse_arguments()?;

    let id = contact_repo.save_contact(contact).await?;

    println!("{id}");

    let all_contact = contact_repo.get_all().await?;

    println!("{all_contact:?}");

    Ok(())
}

fn parse_arguments() -> Result<Contact, AppError> {
    let args: Vec<String> = env::args().collect();

    let has_correct_number_of_args = args.len() != 5;

    if has_correct_number_of_args {
        return Err(AppError::InvalidArguments);
    }

    Contact::new(
        args[1].clone(),
        args[2].clone(),
        args[3].clone(),
        args[4].clone(),
    )
}
