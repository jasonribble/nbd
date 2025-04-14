#[cfg(test)]
use anyhow::Ok;
#[cfg(test)]
use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};
#[cfg(test)]
use std::fs;

#[cfg(test)]
pub async fn setup_in_memory_db() -> SqlitePool {
    let pool = SqlitePoolOptions::new()
        .connect("sqlite::memory:")
        .await
        .expect("Failed to create in-memory SQLite database");

    let migrations_entries = get_migration_entries().unwrap();

    for migration_entry in migrations_entries.into_iter() {
        let migration_file_path = String::from(migration_entry.to_string_lossy());

        let migration = fs::read_to_string(migration_file_path.clone())
            .expect("Should have been able to read the file");

        let error = format!("Failed to insert {}", migration_file_path);

        sqlx::query(&migration).execute(&pool).await.expect(&error);
    }

    pool
}

#[cfg(test)]
pub fn get_migration_entries() -> Result<Vec<std::path::PathBuf>, anyhow::Error> {
    let mut entries = std::fs::read_dir("migrations")?
        .map(|res| res.map(|entry| entry.path()))
        .collect::<Result<Vec<_>, std::io::Error>>()?;

    entries.sort();

    Ok(entries)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn current_number_of_migrations() {
        let migrations = 5;

        let entries = get_migration_entries().unwrap();

        assert_eq!(migrations, entries.len())
    }
}
