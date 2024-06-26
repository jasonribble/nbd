use crate::models::Contact;
use sqlx::{migrate::MigrateDatabase, Sqlite, SqlitePool};

const DB_URL: &str = "sqlite://contacts.db";

pub async fn connect() -> Result<SqlitePool, sqlx::Error> {
    let database_url = DB_URL;
    println!("Connected to {database_url}",);
    SqlitePool::connect(database_url).await
}

pub async fn save_contact(pool: &SqlitePool, contact: &Contact) -> Result<(), sqlx::Error> {
    let query = "INSERT INTO contacts (first_name, last_name, display_name, email, phone_number)
                 VALUES (?, ?, ?, ?, ?)";

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

pub async fn create_contacts_table(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS contacts (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
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

    if !Sqlite::database_exists(db_url).await.unwrap_or(false) {
        println!("Creating database {db_url}");
        match Sqlite::create_database(db_url).await {
            Ok(()) => println!("Create db success"),
            Err(error) => panic!("error: {error}"),
        }
    }
}
