use std::sync::Arc;

use crate::models;
use async_trait::async_trait;
use sqlx::postgres::PgPool;

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait ContactRepo {
    async fn save_contact(&self, contact: models::Contact) -> anyhow::Result<()>;
}

pub struct PostgresContactRepo {
    pg_pool: Arc<PgPool>,
}

impl PostgresContactRepo {
    pub fn new(pg_pool: PgPool) -> Self {
        Self {
            pg_pool: Arc::new(pg_pool),
        }
    }
}

#[async_trait]
impl ContactRepo for PostgresContactRepo {
    async fn save_contact(&self, contact: models::Contact) -> anyhow::Result<()> {
        let query = "INSERT INTO contacts
            (first_name, last_name, display_name, email, phone_number)
            VALUES ($1, $2, $3, $4, $5)";

        sqlx::query(query)
            .bind(&contact.first_name)
            .bind(&contact.last_name)
            .bind(&contact.display_name)
            .bind(&contact.email)
            .bind(&contact.phone_number)
            .execute(&*self.pg_pool)
            .await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::predicate::*;
    use models;

    #[tokio::test]
    async fn test_save_contact() {
        let mut mock_contact_repo = MockContactRepo::new();

        let test_contact = models::Contact {
            first_name: "John".to_string(),
            last_name: "Smith".to_string(),
            display_name: "John Smith".to_string(),
            email: "johndoe@example.com".to_string(),
            phone_number: "123-456-7890".to_string(),
        };

        mock_contact_repo
            .expect_save_contact()
            .times(1)
            .with(eq(test_contact.clone()))
            .returning(|_| Ok(()));

        let result = mock_contact_repo.save_contact(test_contact).await;

        assert!(result.is_ok());
    }
}
