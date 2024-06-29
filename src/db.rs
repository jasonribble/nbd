use crate::models::Contact;
use sqlx::{migrate::MigrateDatabase, PgPool, Postgres};

const DB_URL: &str = "postgres://postgres:test@localhost/contacts";

pub async fn connect() -> Result<PgPool, sqlx::Error> {
    let database_url = DB_URL;
    println!("Connected to {database_url}");
    PgPool::connect(database_url).await
}

pub async fn save_contact(pool: &PgPool, contact: &Contact) -> Result<(), sqlx::Error> {
    let query = "INSERT INTO contacts (first_name, last_name, display_name, email, phone_number)
                 VALUES ($1, $2, $3, $4, $5)";

    sqlx::query(query)
        .bind(&contact.first_name)
        .bind(&contact.last_name)
        .bind(&contact.display_name)
        .bind(&contact.email)
        .bind(&contact.phone_number)
        .execute(pool)
        .await?;

    Ok(())
}

pub async fn create_contacts_table(pool: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS contacts (
            id SERIAL PRIMARY KEY,
            first_name TEXT NOT NULL,
            last_name TEXT NOT NULL,
            display_name TEXT NOT NULL,
            email TEXT NOT NULL,
            phone_number TEXT NOT NULL
        );",
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn create_database() {
    let db_url = DB_URL;

    if !Postgres::database_exists(db_url).await.unwrap_or(false) {
        println!("Creating database {db_url}");
        match Postgres::create_database(db_url).await {
            Ok(()) => println!("Create db success"),
            Err(error) => panic!("error: {error}"),
        }
    }
}
