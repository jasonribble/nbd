use std::path::Path;

use sqlx::Sqlite;
use sqlx::{migrate::MigrateDatabase, SqlitePool};

use crate::utils::{build_database_path, build_database_url, ensure_config_dir};

/// # Errors
///
/// Will return sqlite errors
pub async fn create_database(url: &str) -> anyhow::Result<()> {
    Sqlite::create_database(url).await?;

    let pool = SqlitePool::connect(url).await?;
    sqlx::migrate!("./migrations").run(&pool).await?;

    Ok(())
}

/// Bootstraps a fresh contact book on disk.
///
/// Ensures the config directory exists, then creates the database
/// file inside it. This is the imperative shell that composes the pure
/// path helpers with the database-creating I/O.
///
/// # Errors
///
/// Returns an error if the config directory cannot be created or the
/// database file cannot be created.
pub async fn initialize(config_dir: &Path) -> anyhow::Result<()> {
    ensure_config_dir(config_dir)?;

    let db_path = build_database_path(config_dir);

    let db_url = build_database_url(&db_path);

    create_database(&db_url).await?;

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn create_database_creates_file_at_path() -> anyhow::Result<()> {
        let temp = tempfile::TempDir::new()?;
        let db_path = temp.path().join("contacts.db");
        let url = build_database_url(&db_path);
        assert!(!db_path.exists());

        create_database(&url).await?;

        assert!(db_path.exists());
        Ok(())
    }

    #[tokio::test]
    async fn initialize_creates_database_in_config_dir() -> anyhow::Result<()> {
        let temp = tempfile::TempDir::new()?;
        let config_dir = temp.path().join("nbd");
        let db_path = config_dir.join("contacts.db");
        assert!(!db_path.exists());

        initialize(&config_dir).await?;

        assert!(db_path.exists(), "expected database at {db_path:?}");
        Ok(())
    }
}
