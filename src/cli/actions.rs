use nbd::{
    db::{self, ContactRepo},
    models::{self, ContactBuilder},
};
use tabled::Table;

use crate::commander::{CreateCommand, DeleteCommand, EditCommand, GetCommand, ImportCommand};

pub async fn create_contact(
    command: &CreateCommand,
    data_repo: &db::Connection,
) -> Result<(), anyhow::Error> {
    let contact = models::Contact::new(
        command.first_name.as_deref().unwrap_or(""),
        command.last_name.as_deref().unwrap_or(""),
        command.email.as_deref().unwrap_or(""),
        command.phone_number.as_deref().unwrap_or(""),
        chrono::NaiveDate::default(),
    );

    let contact = contact.unwrap();

    let id = data_repo.save_contact(contact).await?;

    println!("Successfully saved contact {id}");

    Ok(())
}

pub async fn edit_contact(
    command: &EditCommand,
    data_repo: &db::Connection,
) -> Result<(), anyhow::Error> {
    let contact = ContactBuilder::new(
        command.id,
        command.first_name.clone(),
        command.last_name.clone(),
        command.display_name.clone(),
        command.email.clone(),
        command.phone_number.clone(),
    )
    .unwrap();

    let _ = data_repo.update_contact(contact).await;

    Ok(())
}

pub async fn show_all_contacts(data_repo: &db::Connection) -> Result<(), anyhow::Error> {
    let contacts = data_repo.get_all_contacts().await?;

    if contacts.is_empty() {
        println!("No contacts yet!");
    } else {
        let table = Table::new(contacts);
        println!("{table}");
    }

    Ok(())
}

pub async fn get_contact(
    command: &GetCommand,
    data_repo: &db::Connection,
) -> Result<(), anyhow::Error> {
    let id = command.id;

    let contact = data_repo.get_contact_by_id(id).await?;

    println!("{contact:?}");

    Ok(())
}

pub async fn delete_contact(
    command: &DeleteCommand,
    data_repo: &db::Connection,
) -> Result<(), anyhow::Error> {
    let id = command.id;

    let contact_id = data_repo.delete_contact_by_id(id).await?;

    println!("Successfully deleted contact {contact_id}");

    Ok(())
}

pub async fn import_contacts(
    command: &ImportCommand,
    data_repo: &db::Connection,
) -> Result<(), anyhow::Error> {
    let result_of_import = data_repo
        .import_contacts_by_csv(command.filename.as_str())
        .await;

    match result_of_import {
        Ok(number_of_imports) => {
            println!("Successfully imported {number_of_imports} contact");
        }
        Err(error) => {
            println!("{error}");
        }
    }

    Ok(())
}
