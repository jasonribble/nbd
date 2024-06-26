use dialoguer::Input;

mod db;
mod models;
mod utils;

use models::Contact;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    db::create_database().await;

    let pool = db::connect().await?;

    db::create_contacts_table(&pool).await?;

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

    let contact = Contact::new(first_name, last_name, email, phone);

    println!();
    println!("Contact name: {}", contact.display_name);
    println!("Contact number: {}", contact.phone_number);
    println!("Contact email {}", contact.email);

    db::save_contact(&pool, &contact).await?;

    Ok(())
}
