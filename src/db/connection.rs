use sqlx::SqlitePool;
use std::sync::Arc;

pub struct Repo<D> {
    pub database: Arc<D>,
}

impl<D> Repo<D> {
    #[must_use]
    pub fn new(database: D) -> Self {
        Self {
            database: Arc::new(database),
        }
    }
}

impl Repo<SqlitePool> {
    /// # Errors
    ///
    /// Will error if the database is not connected.
    pub async fn check_connection(&self) -> anyhow::Result<()> {
        // Execute a simple query to check if the connection works
        sqlx::query("SELECT 1").execute(&*self.database).await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        db::{ContactRepo, Repo},
        models::{Contact, OptionalContact},
        test_helpers::setup_in_memory_db,
    };

    #[tokio::test]
    async fn test_save_contact() {
        let pool = setup_in_memory_db().await;

        let data_repo = Repo::new(pool);

        let example_contact = Contact::builder()
            .first_name("Lewis")
            .last_name("Carroll")
            .email("lewis@wonderland.com")
            .phone_number("777-777-7777")
            .birthday("1832-1-27")
            .build()
            .expect("Expect Louis Carroll");

        let result_contact_id = data_repo.save_contact(example_contact).await;
        let contact_id = result_contact_id.expect("Valid Contact");

        assert_eq!(contact_id, 1);
    }

    #[tokio::test]
    async fn test_save_optional_contact() {
        let pool = setup_in_memory_db().await;

        let data_repo = Repo::new(pool);

        let example_contact = OptionalContact {
            first_name: Some("Alice".to_string()),
            ..OptionalContact::template()
        };

        let result_contact_id = data_repo.save_optional_contact(example_contact).await;
        let contact_id = result_contact_id.expect("Valid Contact ID");

        assert_eq!(contact_id, 1);
    }

    #[tokio::test]
    async fn test_delete_contact() {
        let pool = setup_in_memory_db().await;

        let data_repo = Repo::new(pool);

        let example_contact = Contact::builder()
            .first_name("Lewis")
            .last_name("Carroll")
            .email("lewis@wonderland.com")
            .phone_number("777-777-7777")
            .birthday("1832-1-27")
            .build()
            .expect("Louis Carroll");

        let result_contact_id = data_repo.save_contact(example_contact.clone()).await;

        let contact_id = result_contact_id.expect("Valid ID");

        let contact_from_database = data_repo
            .get_contact_by_id(contact_id)
            .await
            .expect("Contact from database");

        assert_eq!(
            contact_from_database.contact.first_name,
            example_contact.first_name
        );

        let deleted_contact_id = data_repo
            .delete_contact_by_id(contact_id)
            .await
            .expect("Deleted from database");

        assert_eq!(deleted_contact_id, 1);
    }
}
