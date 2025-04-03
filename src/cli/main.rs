use std::env;

use db::Connection;
use nbd::db;

mod actions;
mod commander;

use clap::Parser;
use commander::{Cli, Commands};
use sqlx::SqlitePool;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    let pool = SqlitePool::connect(&env::var("DATABASE_URL")?).await?;

    let data_repo = Connection::new(pool);

    let cli = Cli::parse();

    match &cli.command {
        Commands::Create(value) => actions::create_contact(value, &data_repo).await?,
        Commands::Edit(value) => actions::edit_contact(value, &data_repo).await?,
        Commands::Show => actions::show_all_contacts(&data_repo).await?,
        Commands::Get(value) => actions::get_contact(value, &data_repo).await?,
        Commands::Delete(value) => actions::delete_contact(value, &data_repo).await?,
        Commands::Import(value) => actions::import_contacts(value, &data_repo).await?,
    }

    Ok(())
}
