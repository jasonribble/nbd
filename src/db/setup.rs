use sqlx::migrate::MigrateDatabase;
use sqlx::Sqlite;

/// # Errors
///
/// Will return sqlite errors
pub async fn create_database(url: &str) -> anyhow::Result<()> {
    Sqlite::create_database(url).await?;
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils::build_database_url;

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
}
