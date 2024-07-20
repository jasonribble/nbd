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
            println!("This is an edit command {value:?}")
        }
    }

    Ok(())
}

#[cfg(test)]

mod tests {
    use assert_cmd::Command;

    #[test]
    fn test_help() {
        let mut cmd = Command::cargo_bin("connect").unwrap();

        cmd.arg("--help");

        cmd.assert()
            .success()
            .stdout(predicates::str::contains("Usage: connect <COMMAND>"));
    }

    #[test]
    fn test_connect_works() {
        let mut cmd = Command::cargo_bin("connect").unwrap();

        cmd.arg("create")
            .arg("--first-name")
            .arg("First")
            .arg("--last-name")
            .arg("Last")
            .arg("--email")
            .arg("test@test.com")
            .arg("--phone-number")
            .arg("123-321-1233");

        cmd.assert()
            .success()
            .stdout(predicates::str::contains("Successfully saved contact."));
    }

    #[test]
    fn test_connect_invalid_email() {
        let mut cmd = Command::cargo_bin("connect").unwrap();

        cmd.arg("create")
            .arg("--first-name")
            .arg("First")
            .arg("--last-name")
            .arg("Last")
            .arg("--email")
            .arg("test@.com")
            .arg("--phone-number")
            .arg("123-321-1233");

        cmd.assert()
            .failure()
            .stderr(predicates::str::contains("InvalidEmail"));
    }

    #[test]
    fn test_connect_invalid_phone() {
        let mut cmd = Command::cargo_bin("connect").unwrap();

        cmd.arg("create")
            .arg("--first-name")
            .arg("First")
            .arg("--last-name")
            .arg("Last")
            .arg("--email")
            .arg("test@com.com")
            .arg("--phone-number")
            .arg("123-321-123");

        cmd.assert()
            .failure()
            .stderr(predicates::str::contains("InvalidPhone"));
    }

    #[test]
    fn test_connect_invalid_args() {
        let mut cmd = Command::cargo_bin("connect").unwrap();

        cmd.arg("First").arg("Last").arg("32321123");

        cmd.assert()
            .failure()
            .stderr(predicates::str::contains("Usage: connect <COMMAND>"));
    }
}
