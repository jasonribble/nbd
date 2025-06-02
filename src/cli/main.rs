use std::env;

mod actions;
mod commander;

use actions::Actions;
use clap::Parser;
use commander::{Cli, Commands};
use nbd::db::Connection;
use sqlx::SqlitePool;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    let pool = SqlitePool::connect(&env::var("DATABASE_URL")?).await?;
    let data_repo = Connection::new(pool);
    let actions = Actions::new(data_repo);

    let cli = Cli::parse();

    match &cli.command {
        Commands::Init => actions.init_contact_book().await?,
        Commands::Create(value) => actions.create_contact(value).await?,
        Commands::Edit(value) => actions.edit_contact(value).await?,
        Commands::Show => actions.show_all_contacts().await?,
        Commands::Get(value) => actions.get_contact(value).await?,
        Commands::Delete(value) => actions.delete_contact(value).await?,
        Commands::Import(value) => actions.import_contacts(value).await?,
    }

    Ok(())
}
