use nbd::{
    db::{self, Connection, ContactRepo},
    models::{self, ContactBuilder},
};
use tabled::Table;

use crate::commander::{CreateCommand, DeleteCommand, EditCommand, GetCommand, ImportCommand};

pub struct Actions {
    data_repo: db::Connection,
}

impl Actions {
    pub const fn new(data_repo: Connection) -> Self {
        Self { data_repo }
    }

    pub async fn create_contact(&self, command: &CreateCommand) -> Result<(), anyhow::Error> {
        let contact = models::Contact::new(
            command.first_name.as_deref().unwrap_or(""),
            command.last_name.as_deref().unwrap_or(""),
            command.email.as_deref().unwrap_or(""),
            command.phone_number.as_deref().unwrap_or(""),
            command.birthday.as_deref().unwrap_or(""),
        );

        let contact = contact.unwrap();

        let id = self.data_repo.save_contact(contact).await?;

        println!("Successfully saved contact {id}");

        Ok(())
    }

    pub async fn edit_contact(&self, command: &EditCommand) -> Result<(), anyhow::Error> {
        let contact = ContactBuilder::new(
            command.id,
            command.first_name.clone(),
            command.last_name.clone(),
            command.email.clone(),
            command.phone_number.clone(),
            command.display_name.clone(),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        )
        .unwrap();

        let _ = self.data_repo.update_contact(contact).await;

        Ok(())
    }

    pub async fn show_all_contacts(&self) -> Result<(), anyhow::Error> {
        let contacts = self.data_repo.get_all_contacts().await?;

        if contacts.is_empty() {
            println!("No contacts yet!");
        } else {
            let table = Table::new(contacts);
            println!("{table}");
        }

        Ok(())
    }

    pub async fn get_contact(&self, command: &GetCommand) -> Result<(), anyhow::Error> {
        let id = command.id;

        let contact = self.data_repo.get_contact_by_id(id).await?;

        println!("{contact:?}");

        Ok(())
    }

    pub async fn delete_contact(&self, command: &DeleteCommand) -> Result<(), anyhow::Error> {
        let id = command.id;

        let contact_id = self.data_repo.delete_contact_by_id(id).await?;

        println!("Successfully deleted contact {contact_id}");

        Ok(())
    }

    pub async fn import_contacts(&self, command: &ImportCommand) -> Result<(), anyhow::Error> {
        let result_of_import = self
            .data_repo
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

    pub async fn init_contact_book(&self) -> Result<(), anyhow::Error> {
        // Check if the database exists and has been initialized
        // For simplicity, we'll just check if we can connect to it
        if self.data_repo.check_connection().await.is_ok() {
            println!("A contact book has already been initalized");
        }
        Ok(())
    }
}
