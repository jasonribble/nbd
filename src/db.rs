use crate::models::Contact;
use sqlx::{sqlite::SqliteConnection, Connection};

const DB_URL: &str = "sqlite://contacts.db";

pub async fn connect() -> Result<SqliteConnection, sqlx::Error> {
    let conn = SqliteConnection::connect(DB_URL).await?;

    Ok(conn)
}

pub async fn save_contact(
    conn: &mut SqliteConnection,
    contact: &Contact,
) -> Result<(), sqlx::Error> {
    let query = "INSERT INTO contacts (first_name, last_name, display_name, email, phone_number)
                 VALUES (?, ?, ?, ?, ?)";

    sqlx::query(query)
        .bind(&contact.first_name)
        .bind(&contact.last_name)
        .bind(&contact.display_name)
        .bind(&contact.email)
        .bind(&contact.phone_number)
        .execute(conn)
        .await?;

    Ok(())
}

pub async fn create_contacts_table(conn: &mut SqliteConnection) -> Result<(), sqlx::Error> {
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
    .execute(conn)
    .await?;

    Ok(())
}
