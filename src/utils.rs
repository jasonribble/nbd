use rusqlite::{Connection, params};
use crate::contact::Contact;

pub fn save_contact(conn: &Connection, contact: &Contact) -> Result<(), rusqlite::Error> {
    let query = "INSERT INTO contacts (first_name, last_name, display_name, email, phone_number)
                 VALUES (?1, ?2, ?3, ?4, ?5)";

    conn.execute(
        query,
        params![
            contact._first_name,
            contact._last_name,
            contact.display_name,
            contact.email.to_string(),
            contact.phone_number.to_string(),
        ],
    )?;

    Ok(())
}

pub fn create_contacts_table(conn: &Connection) -> Result<(), rusqlite::Error> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS contacts (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            first_name TEXT NOT NULL,
            last_name TEXT NOT NULL,
            display_name TEXT NOT NULL,
            email TEXT NOT NULL,
            phone_number TEXT NOT NULL
        );",
        [],
    )?;
    Ok(())
}
