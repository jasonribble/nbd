use std::sync::Arc;

use crate::models;
use async_trait::async_trait;
use sqlx::postgres::PgPool;

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
