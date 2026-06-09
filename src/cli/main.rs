use std::env;

mod actions;
mod commander;

use actions::Actions;
use clap::Parser;
use commander::{Cli, Commands};
use nbd::db::Repo;
use sqlx::SqlitePool;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    let cli = Cli::parse();

    if matches!(cli.command, Commands::Init) {
        let config_dir = nbd::utils::get_config_dir();

        let db_path = nbd::utils::build_database_path(&config_dir);

        if nbd::utils::is_already_initialized(&db_path) {
            print!("A contact book has already been initialized");
            return Ok(());
        }

        nbd::db::setup::initialize(&config_dir).await?;

        return Ok(());
    }

    let pool = SqlitePool::connect(&env::var("DATABASE_URL")?).await?;
    let data_repo = Repo::new(pool);
    let actions = Actions::new(data_repo);

    match &cli.command {
        Commands::Init => {} // handled above (branch early)
        Commands::Create(value) => actions.create_contact(value).await?,
        Commands::Edit(value) => actions.edit_contact(value).await?,
        Commands::Show => actions.show_all_contacts().await?,
        Commands::Get(value) => actions.get_contact(value).await?,
        Commands::Delete(value) => actions.delete_contact(value).await?,
        Commands::Import(value) => actions.import_contacts(value).await?,
    }

    Ok(())
}
