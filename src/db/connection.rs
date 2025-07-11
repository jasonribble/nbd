use sqlx::SqlitePool;
use std::sync::Arc;

pub struct Connection {
    pub sqlite_pool: Arc<SqlitePool>,
}

impl Connection {
    #[must_use]
    pub fn new(pool: SqlitePool) -> Self {
        Self {
            sqlite_pool: Arc::new(pool),
        }
    }

    /// # Errors
    ///
    /// Will error if the database is not connected.
    pub async fn check_connection(&self) -> anyhow::Result<()> {
        // Execute a simple query to check if the connection works
        sqlx::query("SELECT 1").execute(&*self.sqlite_pool).await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        db::{Connection, ContactRepo, MetadataRepo},
        models::{Contact, OptionalContact},
        test_helpers::setup_in_memory_db,
    };

    #[tokio::test]
    async fn test_save_contact_get_metadata() {
        let pool = setup_in_memory_db().await;

        let data_repo = Connection::new(pool);

        let example_contact = Contact::new(
            "Lewis",
            "Carroll",
            "lewis@wonderland.com",
            "777-777-7777",
            "1832-1-27",
        )
        .unwrap();

        let result_contact_id = data_repo.save_contact(example_contact).await;
        let contact_id = result_contact_id.unwrap();

        let result_expected_metadata = data_repo.get_metadata_by_id(contact_id).await;

        let expected_metadata = result_expected_metadata.unwrap();

        assert_eq!(contact_id, expected_metadata.contact_id);
    }

    #[tokio::test]
    async fn test_save_optional_contact_get_metadata() {
        let pool = setup_in_memory_db().await;

        let data_repo = Connection::new(pool);

        let example_contact = OptionalContact {
            first_name: Some("Alice".to_string()),
            ..OptionalContact::template()
        };

        let result_contact_id = data_repo.save_optional_contact(example_contact).await;
        let contact_id = result_contact_id.unwrap();

        let result_expected_metadata = data_repo.get_metadata_by_id(contact_id).await;

        let expected_metadata = result_expected_metadata.unwrap();

        assert_eq!(contact_id, expected_metadata.contact_id);
    }

    #[tokio::test]
    async fn test_delete_contact_deletes_metadata() {
        let pool = setup_in_memory_db().await;

        let data_repo = Connection::new(pool);

        let example_contact = Contact::new(
            "Lewis",
            "Carroll",
            "lewis@wonderland.com",
            "777-777-7777",
            "1832-1-27",
        )
        .unwrap();

        let result_contact_id = data_repo.save_contact(example_contact.clone()).await;

        let contact_id = result_contact_id.unwrap();

        let contact_from_database = data_repo.get_contact_by_id(contact_id).await;
        let contact_from_database = contact_from_database.unwrap();

        assert_eq!(
            contact_from_database.contact.first_name,
            example_contact.first_name
        );

        let deleted_contact_id = data_repo.delete_contact_by_id(contact_id).await.unwrap();

        let failed_contact_metadata = data_repo.get_metadata_by_id(deleted_contact_id).await;

        assert!(failed_contact_metadata.is_err());
    }
}
