use rusqlite::{params, Connection};

use crate::models::Contact;

pub fn connect() -> Result<Connection, rusqlite::Error> {
    let conn = Connection::open("contacts.db")?;
    Ok(conn)
}

pub fn save_contact(conn: &Connection, contact: &Contact) -> Result<(), rusqlite::Error> {
    let query = "INSERT INTO contacts (first_name, last_name, display_name, email, phone_number)
                 VALUES (?1, ?2, ?3, ?4, ?5)";

    conn.execute(
        query,
        params![
            contact.first_name,
            contact.last_name,
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

#[cfg(test)]
mod tests {
    use super::save_contact;
    use crate::models::Contact;
    use rusqlite::{Connection, Result};

    fn setup_test_db() -> Result<Connection> {
        let conn = Connection::open_in_memory()?;
        conn.execute(
            "CREATE TABLE contacts (
                id INTEGER PRIMARY KEY,
                first_name TEXT,
                last_name TEXT,
                display_name TEXT,
                email TEXT,
                phone_number TEXT
            )",
            [],
        )?;
        Ok(conn)
    }

    #[test]
    fn test_save_contact() -> Result<()> {
        let conn = setup_test_db()?;

        let contact = Contact::new(
            "John".to_string(),
            "Doe".to_string(),
            "john@example.com".to_string(),
            "1234567890".to_string(),
        );

        save_contact(&conn, &contact)?;

        // Verify the contact was saved correctly
        let saved_contact: Contact = conn.query_row(
            "SELECT first_name, last_name, display_name, email, phone_number FROM contacts WHERE id = 1",
            [],
            |row| {
                Ok(Contact {
                    first_name: row.get(0)?,
                    last_name: row.get(1)?,
                    display_name: row.get(2)?,
                    email: row.get(3)?,
                    phone_number: row.get(4)?,
                })
            },
        )?;

        assert_eq!(contact.first_name, saved_contact.first_name);
        assert_eq!(contact.last_name, saved_contact.last_name);
        assert_eq!(contact.display_name, saved_contact.display_name);
        assert_eq!(contact.email, saved_contact.email);
        assert_eq!(contact.phone_number, saved_contact.phone_number);

        Ok(())
    }
}
