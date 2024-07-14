use std::env;

mod db;
mod errors;
mod models;
mod utils;

use db::{ContactRepo, PostgresContactRepo};
use errors::AppError;
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

    let most_recent_contact = &all_contact[all_contact.len() - 1..];

    println!("{most_recent_contact:?}");

    let edits = models::ContactBuilder::new(id)
        .set_first_name("New Name")
        .set_last_name("Yep")
        .set_email("completely@new.com")
        .set_phone_number("1233211233")
        .set_display_name("Nickname")
        .build()
        .unwrap();

    let _ = contact_repo.update_contact(edits).await;

    let my_first_contact = contact_repo.get_by_id(1).await.unwrap();

    println!("My first contact is {my_first_contact:?}");

    Ok(())
}

fn parse_arguments() -> Result<models::Contact, AppError> {
    let args: Vec<String> = env::args().collect();

    let has_correct_number_of_args = args.len() != 5;

    if has_correct_number_of_args {
        return Err(AppError::InvalidArguments);
    }

    models::Contact::new(&args[1], &args[2], &args[3], &args[4])
}
