use crate::{models, utils};
use async_trait::async_trait;

use super::connection::Connection;

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait ContactRepo {
    async fn save_contact(&self, contact: models::Contact) -> anyhow::Result<i64>;
    async fn save_optional_contact(&self, contact: models::OptionalContact) -> anyhow::Result<i64>;
    async fn import_contacts_by_csv(&self, filename: &str) -> anyhow::Result<i64>;
    async fn get_all_contacts(&self) -> anyhow::Result<Vec<models::IndexedContact>>;
    async fn update_contact(&self, update: models::ContactBuilder) -> anyhow::Result<()>;
    async fn get_contact_by_id(&self, id: i64) -> anyhow::Result<models::IndexedContact>;
    async fn delete_contact_by_id(&self, id: i64) -> anyhow::Result<i64>;
}

#[async_trait]
impl ContactRepo for Connection {
    async fn save_contact(&self, contact: models::Contact) -> anyhow::Result<i64> {
        let query = "INSERT INTO contacts
        (first_name, last_name, display_name, email, phone_number, birthday, starred, is_archived, created_at, updated_at, last_seen_at, frequency, last_reminder_at)
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)";
        let result = sqlx::query(query)
            .bind(&contact.first_name)
            .bind(&contact.last_name)
            .bind(&contact.display_name)
            .bind(&contact.email)
            .bind(&contact.phone_number)
            .bind(contact.birthday)
            .bind(contact.starred)
            .bind(contact.is_archived)
            .bind(contact.created_at)
            .bind(contact.updated_at)
            .bind(contact.last_seen_at)
            .bind(&contact.frequency)
            .bind(contact.last_reminder_at)
            .execute(&*self.sqlite_pool)
            .await?;

        let contact_id = result.last_insert_rowid();

        Ok(contact_id)
    }

    async fn get_all_contacts(&self) -> anyhow::Result<Vec<models::IndexedContact>> {
        let get_contacts_query = "SELECT *
             FROM contacts
             ORDER BY id";

        let contacts_with_id: Vec<models::IndexedContact> =
            sqlx::query_as::<_, models::IndexedContact>(get_contacts_query)
                .fetch_all(&*self.sqlite_pool)
                .await?;

        Ok(contacts_with_id)
    }

    async fn update_contact(&self, contact: models::ContactBuilder) -> anyhow::Result<()> {
        use chrono::Utc;
        
        let now = Utc::now();
        
        sqlx::query!(
            r#"
            UPDATE contacts
            SET
                first_name = COALESCE($1, first_name),
                last_name = COALESCE($2, last_name),
                display_name = COALESCE($3, display_name),
                email = COALESCE($4, email),
                phone_number = COALESCE($5, phone_number),
                birthday = COALESCE($6, birthday),
                starred = COALESCE($7, starred),
                is_archived = COALESCE($8, is_archived),
                updated_at = $9,
                last_seen_at = COALESCE($10, last_seen_at),
                frequency = COALESCE($11, frequency),
                last_reminder_at = COALESCE($12, last_reminder_at)
            WHERE id = $13
            "#,
            contact.optional_contact.first_name,
            contact.optional_contact.last_name,
            contact.optional_contact.display_name,
            contact.optional_contact.email,
            contact.optional_contact.phone_number,
            contact.optional_contact.birthday,
            contact.optional_contact.starred,
            contact.optional_contact.is_archived,
            now,
            contact.optional_contact.last_seen_at,
            contact.optional_contact.frequency,
            contact.optional_contact.last_reminder_at,
            contact.id
        )
        .execute(&*self.sqlite_pool)
        .await?;

        println!("Contact updated");

        Ok(())
    }

    async fn get_contact_by_id(&self, id: i64) -> anyhow::Result<models::IndexedContact> {
        let query_get_by_id = "SELECT * FROM contacts WHERE id=$1";

        let contact: models::IndexedContact =
            sqlx::query_as::<_, models::IndexedContact>(query_get_by_id)
                .bind(id)
                .fetch_one(&*self.sqlite_pool)
                .await?;

        Ok(contact)
    }

    async fn delete_contact_by_id(&self, id: i64) -> anyhow::Result<i64> {
        let query_delete_by_id = "DELETE FROM contacts WHERE id=$1 RETURNING id";

        let contact_id = sqlx::query(query_delete_by_id)
            .bind(id)
            .execute(&*self.sqlite_pool)
            .await?;

        Ok(contact_id.last_insert_rowid())
    }

    async fn save_optional_contact(&self, contact: models::OptionalContact) -> anyhow::Result<i64> {
        use chrono::Utc;
        
        let mut display_name = contact.display_name.clone();

        if display_name.is_none() {
            let firstname = contact.first_name.clone();
            let lastname = contact.last_name.clone();
            display_name = Some(format!(
                "{} {}",
                firstname.unwrap_or_default(),
                lastname.unwrap_or_default()
            ));
        }

        let query =
            "INSERT INTO contacts (first_name, last_name, display_name, phone_number, email, birthday, starred, is_archived, created_at, updated_at, last_seen_at, frequency, last_reminder_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)";

        let birthday = contact
            .birthday
            .unwrap_or(chrono::NaiveDate::from_ymd_opt(1, 1, 1).unwrap());

        let now = Utc::now();

        let result = sqlx::query(query)
            .bind(&contact.first_name)
            .bind(&contact.last_name)
            .bind(display_name)
            .bind(&contact.phone_number)
            .bind(&contact.email)
            .bind(birthday)
            .bind(contact.starred.unwrap_or(false))
            .bind(contact.is_archived.unwrap_or(false))
            .bind(now)
            .bind(now)
            .bind(contact.last_seen_at)
            .bind(&contact.frequency)
            .bind(contact.last_reminder_at)
            .execute(&*self.sqlite_pool)
            .await?;

        let contact_id = result.last_insert_rowid();

        Ok(contact_id)
    }
    async fn import_contacts_by_csv(&self, filename: &str) -> anyhow::Result<i64> {
        let contacts = utils::process_csv_to_contacts(filename)?;

        let mut number_of_contacts_added = 0;
        for contact in &contacts {
            self.save_optional_contact(contact.clone()).await?;
            number_of_contacts_added += 1;
        }

        Ok(number_of_contacts_added)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::predicate::*;
    use test_utils::setup_in_memory_db;

    #[tokio::test]
    async fn test_save_contact() {
        let mut mock_contact_repo = MockContactRepo::new();

        let test_contact = models::Contact::builder()
            .first_name("John")
            .last_name("Smith")
            .email("johndoe@example.com")
            .phone_number("123-456-7890")
            .birthday("1970-1-1")
            .build()
            .unwrap();

        mock_contact_repo
            .expect_save_contact()
            .times(1)
            .with(eq(test_contact.clone()))
            .returning(|_| Ok(1));

        let result = mock_contact_repo.save_contact(test_contact).await;

        let result = result.unwrap();

        assert_eq!(result, 1);
    }

    #[tokio::test]
    async fn test_get_all_contacts() {
        let mut mock_contact_repo = MockContactRepo::new();

        let contacts = vec![models::IndexedContact {
            id: 1,
            contact: models::Contact::builder()
                .first_name("John")
                .last_name("Doe")
                .email("johndoe@example.com")
                .phone_number("1234567890")
                .birthday("1970-01-01")
                .build()
                .unwrap(),
        }];

        mock_contact_repo
            .expect_get_all_contacts()
            .times(1)
            .return_once(move || Ok(contacts));

        let result = mock_contact_repo.get_all_contacts().await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_update_contact() {
        let mut mock_contact_repo = MockContactRepo::new();

        mock_contact_repo
            .expect_update_contact()
            .times(1)
            .return_once(|_| Ok(()));

        let edits = models::ContactBuilder::new(
            1,
            None,
            None,
            Some("some@email.com".to_string()),
            None,
            None,
            None,
        )
        .unwrap();

        let result = mock_contact_repo.update_contact(edits).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_contact_by_id() {
        let mut mock_contact_repo = MockContactRepo::new();

        let contact = models::IndexedContact {
            id: 1,
            contact: models::Contact::builder()
                .first_name("John")
                .last_name("Doe")
                .email("johndoe@example.com")
                .phone_number("1234567890")
                .birthday("1970-01-01")
                .build()
                .unwrap(),
        };

        mock_contact_repo
            .expect_get_contact_by_id()
            .times(1)
            .with(eq(contact.id))
            .return_once(|_| Ok(contact));

        let result = mock_contact_repo.get_contact_by_id(1).await;

        assert!(result.is_ok());

        let actual_contact = result.unwrap();

        assert_eq!(actual_contact.id, 1);
    }

    #[tokio::test]
    async fn should_save_option_contact_in_database() -> anyhow::Result<()> {
        let pool = setup_in_memory_db().await;

        let data_repo = Connection::new(pool);

        let test_contact = models::OptionalContact {
            first_name: Some("Jason".to_string()),
            ..models::OptionalContact::template()
        };

        let result = data_repo.save_optional_contact(test_contact).await?;

        assert_eq!(result, 1);

        Ok(())
    }

    #[tokio::test]
    async fn should_be_able_to_retrieve_full_contact_when_saved_full_contact() -> anyhow::Result<()>
    {
        let pool = setup_in_memory_db().await;

        let data_repo = Connection::new(pool);

        let test_contact = models::OptionalContact {
            first_name: Some("Ada".to_string()),
            last_name: Some("Lovelace".to_string()),
            display_name: Some("Addy".to_string()),
            email: Some("ada@lovelace.rs".to_string()),
            phone_number: Some("1233211233".to_string()),
            birthday: Some(chrono::NaiveDate::default()),
            starred: None,
            is_archived: None,
            last_seen_at: None,
            frequency: None,
            last_reminder_at: None,
        };

        let contact_id = data_repo
            .save_optional_contact(test_contact.clone())
            .await?;

        let saved_contact = data_repo.get_contact_by_id(contact_id).await?;

        let saved_contact = saved_contact.contact;

        assert_eq!(saved_contact.first_name, test_contact.first_name.unwrap());
        assert_eq!(saved_contact.last_name, test_contact.last_name.unwrap());
        assert_eq!(
            saved_contact.phone_number,
            test_contact.phone_number.unwrap()
        );

        assert_eq!(saved_contact.email, test_contact.email.unwrap());

        assert_eq!(
            saved_contact.display_name,
            test_contact.display_name.unwrap()
        );

        Ok(())
    }

    #[tokio::test]
    async fn should_default_to_first_and_last_name_for_display_name() -> anyhow::Result<()> {
        let pool = setup_in_memory_db().await;

        let data_repo = Connection::new(pool);

        let test_contact = models::OptionalContact {
            first_name: Some("Ada".to_string()),
            last_name: Some("Lovelace".to_string()),
            birthday: Some(chrono::NaiveDate::default()),
            ..models::OptionalContact::template()
        };

        let contact_id = data_repo
            .save_optional_contact(test_contact.clone())
            .await?;

        let saved_contact = data_repo.get_contact_by_id(contact_id).await?;

        let saved_contact = saved_contact.contact;

        assert_eq!(saved_contact.display_name, "Ada Lovelace".to_string());

        let test_contact = models::OptionalContact {
            first_name: Some("Jason".to_string()),
            last_name: Some("Ribble".to_string()),
            ..models::OptionalContact::template()
        };

        let contact_id = data_repo
            .save_optional_contact(test_contact.clone())
            .await?;

        let saved_contact = data_repo.get_contact_by_id(contact_id).await?;

        let saved_contact = saved_contact.contact;

        assert_eq!(saved_contact.display_name, "Jason Ribble".to_string());

        Ok(())
    }

    #[tokio::test]
    async fn should_save_two_option_contact_in_database() -> anyhow::Result<()> {
        let pool = setup_in_memory_db().await;

        let data_repo = Connection::new(pool);

        let test_contact = models::OptionalContact {
            first_name: Some("Jason".to_string()),
            ..models::OptionalContact::template()
        };

        let contact_id = data_repo.save_optional_contact(test_contact).await?;

        assert_eq!(contact_id, 1);

        let another_contact = models::OptionalContact {
            first_name: Some("Alice".to_string()),
            ..models::OptionalContact::template()
        };

        let contact_id = data_repo.save_optional_contact(another_contact).await?;

        assert_eq!(contact_id, 2);

        Ok(())
    }

    #[tokio::test]
    async fn should_store_one_contact_when_given_alice_csv() -> anyhow::Result<()> {
        let pool = setup_in_memory_db().await;

        let data_repo = Connection::new(pool);

        let example_csv = "tests/fixtures/alice.csv";

        let number_of_imported_contacts = data_repo.import_contacts_by_csv(example_csv).await?;

        let number_of_contacts = data_repo.get_all_contacts().await?.len() as i64;

        assert_eq!(number_of_contacts, number_of_imported_contacts);

        Ok(())
    }

    #[tokio::test]
    async fn should_store_three_contacts_when_given_example() -> anyhow::Result<()> {
        let pool = setup_in_memory_db().await;

        let data_repo = Connection::new(pool);

        let example_csv = "tests/fixtures/example.csv";

        let number_of_imported_contacts = data_repo.import_contacts_by_csv(example_csv).await?;

        let number_of_contacts = data_repo.get_all_contacts().await?.len() as i64;

        assert_eq!(number_of_contacts, number_of_imported_contacts);

        Ok(())
    }

    #[tokio::test]
    async fn should_create_metadata_when_importing_csv() -> anyhow::Result<()> {
        let pool = setup_in_memory_db().await;

        let data_repo = Connection::new(pool);

        let example_csv = "tests/fixtures/example.csv";

        let number_of_imported_contacts = data_repo.import_contacts_by_csv(example_csv).await?;

        let contacts = data_repo.get_all_contacts().await?;
        let last_contact = contacts.last().unwrap();
        
        assert_eq!(number_of_imported_contacts, last_contact.id);

        Ok(())
    }

    #[tokio::test]
    async fn should_save_csv_with_one_row_and_birthday() -> anyhow::Result<()> {
        let pool = setup_in_memory_db().await;

        let data_repo = Connection::new(pool);

        let example_csv = "tests/fixtures/birthday.csv";

        data_repo.import_contacts_by_csv(example_csv).await?;

        let contacts = data_repo.get_all_contacts().await?;

        let aldous_huxley_birthday = contacts[0].contact.birthday;

        assert_eq!(
            aldous_huxley_birthday,
            chrono::NaiveDate::from_ymd_opt(1894, 07, 26).unwrap()
        );

        Ok(())
    }

    #[tokio::test]
    async fn should_save_csv_with_threes_row_and_birthday() -> anyhow::Result<()> {
        let pool = setup_in_memory_db().await;

        let data_repo = Connection::new(pool);

        let example_csv = "tests/fixtures/three_birthdays.csv";

        data_repo.import_contacts_by_csv(example_csv).await?;

        let contacts = data_repo.get_all_contacts().await?;

        let aldous_huxley_birthday = contacts[0].contact.birthday;

        assert_eq!(
            aldous_huxley_birthday,
            chrono::NaiveDate::from_ymd_opt(1894, 07, 26).unwrap()
        );

        let cs_lewis_birthday = contacts[2].contact.birthday;

        assert_eq!(
            cs_lewis_birthday,
            chrono::NaiveDate::from_ymd_opt(1898, 11, 29).unwrap()
        );

        Ok(())
    }
}
