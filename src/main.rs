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

    let contact_repo = PostgresContactRepo::new(pool);

    let contact = parse_arguments()?;

    let _ = contact_repo.save(contact).await?;

    println!("Successfully saved contact.");

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

#[cfg(test)]

mod tests {
    use assert_cmd::Command;

    #[test]
    fn test_connect_works() {
        let mut cmd = Command::cargo_bin("connect").unwrap();

        cmd.arg("First")
            .arg("Last")
            .arg("test@test.com")
            .arg("123-321-1233");

        cmd.assert()
            .success()
            .stdout(predicates::str::contains("Successfully saved contact."));
    }

    #[test]
    fn test_connect_invalid_email() {
        let mut cmd = Command::cargo_bin("connect").unwrap();

        cmd.arg("First")
            .arg("Last")
            .arg("test@.com")
            .arg("123-321-1233");

        cmd.assert()
            .failure()
            .stderr(predicates::str::contains("Error: test@.com is invalid.\n"));
    }

    #[test]
    fn test_connect_invalid_phone() {
        let mut cmd = Command::cargo_bin("connect").unwrap();

        cmd.arg("First")
            .arg("Last")
            .arg("test@test.com")
            .arg("32321123");

        cmd.assert()
            .failure()
            .stderr(predicates::str::contains("Error: 32321123 is invalid.\n"));
    }

    #[test]
    fn test_connect_invalid_args() {
        let mut cmd = Command::cargo_bin("connect").unwrap();

        cmd.arg("First").arg("Last").arg("32321123");

        cmd.assert()
            .failure()
            .stderr(predicates::str::contains("Error: Invalid argument\n"));
    }
}
