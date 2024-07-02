use std::env;

mod db;
mod errors;
mod models;
mod utils;

use errors::AppError;
use models::Contact;

#[tokio::main]
async fn main() -> Result<(), AppError> {
    db::create_database().await;

    let pool = db::connect().await?;

    db::create_contacts_table(&pool).await?;

    println!("Welcome. You must run a postgres container to have this work");

    let contact = parse_arguments()?;
    println!("{contact:?}");

    db::save_contact(&pool, &contact).await?;

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
