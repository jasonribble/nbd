use dialoguer::Input;

mod contact;
mod db;
mod phone_number;
mod email;
mod validation;

use crate::contact::Contact;
use rusqlite::Connection;

fn main() -> rusqlite::Result<()> {
    let conn = Connection::open("contacts.db")?;

    db::create_contacts_table(&conn)?;

    println!("Welcome. Below insert the contact information");

    let first_name = Input::new()
        .with_prompt("First name")
        .interact_text()
        .unwrap();

    let last_name = Input::new()
        .with_prompt("Last name")
        .interact_text()
        .unwrap();

    let email = Input::new().with_prompt("Email").interact_text().unwrap();

    let phone = Input::new().with_prompt("Phone").interact_text().unwrap();

    let person = Contact::new(first_name, last_name, email, phone);

    println!();
    println!("Contact name: {}", person.display_name);
    println!("Contact number: {}", person.phone_number);
    println!("Contact email {}", person.email);

    let _ = db::save_contact(&conn, &person);

    Ok(())
}
