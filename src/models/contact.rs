use crate::utils;
use crate::utils::AppError;
use chrono::{DateTime, NaiveDate, Utc};
use tabled::Tabled;

#[derive(Debug, PartialEq, Eq, Clone, sqlx::FromRow, Tabled)]
pub struct Contact {
    pub first_name: String,
    pub last_name: String,
    pub display_name: String,
    pub email: String,
    pub phone_number: String,
    pub birthday: NaiveDate,
    #[tabled(skip)]
    pub starred: bool,
    #[tabled(skip)]
    pub is_archived: bool,
    #[tabled(skip)]
    pub created_at: DateTime<Utc>,
    #[tabled(skip)]
    pub updated_at: DateTime<Utc>,
    #[tabled(skip)]
    pub last_seen_at: Option<DateTime<Utc>>,
    #[tabled(skip)]
    pub frequency: Option<String>,
    #[tabled(skip)]
    pub last_reminder_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Default)]
pub struct ContactBuilder {
    first_name: Option<String>,
    last_name: Option<String>,
    email: Option<String>,
    phone_number: Option<String>,
    birthday: Option<String>,
}

impl ContactBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn first_name(mut self, first_name: &str) -> Self {
        self.first_name = Some(first_name.to_owned());
        self
    }

    pub fn last_name(mut self, last_name: &str) -> Self {
        self.last_name = Some(last_name.to_owned());
        self
    }

    pub fn email(mut self, email: &str) -> Self {
        self.email = Some(email.to_owned());
        self
    }

    pub fn phone_number(mut self, phone_number: &str) -> Self {
        self.phone_number = Some(phone_number.to_owned());
        self
    }

    pub fn birthday(mut self, birthday: &str) -> Self {
        self.birthday = Some(birthday.to_owned());
        self
    }

    /// # Errors
    ///
    /// This errors if there is an invalid email, phone number, or birthday, or if required fields are missing
    ///
    /// # Panics
    /// This will panic if `NaiveDate` fails
    pub fn build(self) -> Result<Contact, AppError> {
        let first_name = self.first_name.unwrap_or_default();
        let last_name = self.last_name.unwrap_or_default();
        let email = self.email.unwrap_or_default();
        let phone_number = self.phone_number.unwrap_or_default();
        let birthday = self.birthday.unwrap_or_default();

        Contact::new(&first_name, &last_name, &email, &phone_number, &birthday)
    }
}

impl Contact {
    #[must_use]
    pub fn builder() -> ContactBuilder {
        ContactBuilder::new()
    }

    /// # Errors
    ///
    /// This errors if there is an invalid email, phone number, or birthday
    ///
    /// # Panics
    /// This will panic if `NaiveDate` fails
    pub fn new(
        first_name: &str,
        last_name: &str,
        email: &str,
        phone_number: &str,
        birthday: &str,
    ) -> Result<Self, AppError> {
        let display_name = format!("{first_name} {last_name}");

        if utils::is_not_valid_email(email) && !email.is_empty() {
            return Err(AppError::InvalidEmail(email.to_owned()));
        }

        if utils::is_not_valid_phone_number(phone_number) && !phone_number.is_empty() {
            return Err(AppError::InvalidPhoneNumber(phone_number.to_owned()));
        }

        let birthday = if birthday.trim().is_empty() {
            NaiveDate::from_ymd_opt(0, 1, 1).unwrap()
        } else {
            let Ok(parsed_date) = NaiveDate::parse_from_str(birthday, "%Y-%m-%d") else {
                return Err(AppError::InvalidBirthday(birthday.to_string()));
            };
            parsed_date
        };

        let now = Utc::now();

        Ok(Self {
            first_name: first_name.to_owned(),
            last_name: last_name.to_owned(),
            display_name,
            email: email.to_owned(),
            phone_number: phone_number.to_owned(),
            birthday,
            starred: false,
            is_archived: false,
            created_at: now,
            updated_at: now,
            last_seen_at: None,
            frequency: None,
            last_reminder_at: None,
        })
    }
}

#[derive(Debug, PartialEq, Eq, Clone, sqlx::FromRow, Tabled)]
pub struct Indexed {
    pub id: i64,
    #[sqlx(flatten)]
    #[tabled(inline)]
    pub contact: Contact,
}

#[derive(Debug, PartialEq, Eq, Clone, serde::Deserialize)]
pub struct Optional {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub display_name: Option<String>,
    pub email: Option<String>,
    pub phone_number: Option<String>,
    pub birthday: Option<NaiveDate>,
    pub starred: Option<bool>,
    pub is_archived: Option<bool>,
    pub last_seen_at: Option<DateTime<Utc>>,
    pub frequency: Option<String>,
    pub last_reminder_at: Option<DateTime<Utc>>,
}

impl Optional {
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.first_name.is_none()
            && self.last_name.is_none()
            && self.display_name.is_none()
            && self.email.is_none()
            && self.phone_number.is_none()
            && self.birthday.is_none()
            && self.starred.is_none()
            && self.is_archived.is_none()
            && self.last_seen_at.is_none()
            && self.frequency.is_none()
            && self.last_reminder_at.is_none()
    }

    #[must_use]
    pub const fn template() -> Self {
        Self {
            first_name: None,
            last_name: None,
            display_name: None,
            email: None,
            phone_number: None,
            birthday: None,
            starred: None,
            is_archived: None,
            last_seen_at: None,
            frequency: None,
            last_reminder_at: None,
        }
    }
}

#[derive(Debug)]
pub struct Construct {
    pub id: i64,
    pub optional_contact: Optional,
}

#[derive(Debug, Default)]
pub struct ConstructBuilder {
    id: Option<i64>,
    first_name: Option<String>,
    last_name: Option<String>,
    email: Option<String>,
    phone_number: Option<String>,
    display_name: Option<String>,
    birthday: Option<NaiveDate>,
    starred: Option<bool>,
    is_archived: Option<bool>,
    last_seen_at: Option<DateTime<Utc>>,
    frequency: Option<String>,
    last_reminder_at: Option<DateTime<Utc>>,
}

impl ConstructBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub const fn id(mut self, id: i64) -> Self {
        self.id = Some(id);
        self
    }

    pub fn first_name(mut self, first_name: String) -> Self {
        self.first_name = Some(first_name);
        self
    }

    pub fn last_name(mut self, last_name: String) -> Self {
        self.last_name = Some(last_name);
        self
    }

    pub fn email(mut self, email: String) -> Self {
        self.email = Some(email);
        self
    }

    pub fn phone_number(mut self, phone_number: String) -> Self {
        self.phone_number = Some(phone_number);
        self
    }

    pub fn display_name(mut self, display_name: String) -> Self {
        self.display_name = Some(display_name);
        self
    }

    pub const fn birthday(mut self, birthday: NaiveDate) -> Self {
        self.birthday = Some(birthday);
        self
    }

    pub const fn starred(mut self, starred: bool) -> Self {
        self.starred = Some(starred);
        self
    }

    pub const fn archived(mut self, is_archived: bool) -> Self {
        self.is_archived = Some(is_archived);
        self
    }

    pub const fn last_seen_at(mut self, last_seen_at: DateTime<Utc>) -> Self {
        self.last_seen_at = Some(last_seen_at);
        self
    }


    pub fn frequency(mut self, frequency: String) -> Self {
        self.frequency = Some(frequency);
        self
    }

    pub const fn last_reminder_at(mut self, last_reminder_at: DateTime<Utc>) -> Self {
        self.last_reminder_at = Some(last_reminder_at);
        self
    }

    /// # Errors
    ///
    /// This errors if there is an invalid email or phone number, missing id, or all fields are empty
    pub fn build(self) -> Result<Construct, AppError> {
        let id = self.id.ok_or(AppError::EmptyUpdate)?;

        let maybe_email = self.email.as_deref().unwrap_or("");
        if utils::is_not_valid_email(maybe_email) && self.email.is_some() {
            return Err(AppError::InvalidEmail(self.email.unwrap_or_default()));
        }

        let maybe_phone = self.phone_number.as_deref().unwrap_or("");
        if utils::is_not_valid_phone_number(maybe_phone) && self.phone_number.is_some() {
            return Err(AppError::InvalidPhoneNumber(
                self.phone_number.unwrap_or_default(),
            ));
        }

        let optional_contact = Optional {
            first_name: self.first_name,
            last_name: self.last_name,
            display_name: self.display_name,
            email: self.email,
            phone_number: self.phone_number,
            birthday: self.birthday,
            starred: self.starred,
            is_archived: self.is_archived,
            last_seen_at: self.last_seen_at,
            frequency: self.frequency,
            last_reminder_at: self.last_reminder_at,
        };

        if optional_contact.is_empty() {
            return Err(AppError::EmptyUpdate);
        }

        Ok(Construct {
            id,
            optional_contact,
        })
    }
}

impl Construct {
    #[must_use]
    pub fn builder() -> ConstructBuilder {
        ConstructBuilder::new()
    }

    /// # Errors
    ///
    /// This errors if there is an invalid email or phone number
    /// 
    /// Creates a new contact with sensible defaults:
    /// - `starred`: false
    /// - `is_archived`: false
    /// - `last_seen_at`: None
    /// - `next_reminder_at`: None
    /// - `frequency`: None
    /// - `last_reminder_at`: None
    pub fn new(
        id: i64,
        first_name: Option<String>,
        last_name: Option<String>,
        email: Option<String>,
        phone_number: Option<String>,
        display_name: Option<String>,
        birthday: Option<NaiveDate>,
    ) -> Result<Self, AppError> {
        let mut builder = ConstructBuilder::new().id(id);

        if let Some(first_name) = first_name {
            builder = builder.first_name(first_name);
        }
        if let Some(last_name) = last_name {
            builder = builder.last_name(last_name);
        }
        if let Some(email) = email {
            builder = builder.email(email);
        }
        if let Some(phone_number) = phone_number {
            builder = builder.phone_number(phone_number);
        }
        if let Some(display_name) = display_name {
            builder = builder.display_name(display_name);
        }
        if let Some(birthday) = birthday {
            builder = builder.birthday(birthday);
        }

        builder.build()
    }

    #[allow(dead_code)]
    const fn is_empty(&self) -> bool {
        self.optional_contact.first_name.is_none()
            && self.optional_contact.last_name.is_none()
            && self.optional_contact.display_name.is_none()
            && self.optional_contact.email.is_none()
            && self.optional_contact.phone_number.is_none()
            && self.optional_contact.birthday.is_none()
            && self.optional_contact.starred.is_none()
            && self.optional_contact.is_archived.is_none()
            && self.optional_contact.last_seen_at.is_none()
            && self.optional_contact.frequency.is_none()
            && self.optional_contact.last_reminder_at.is_none()
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::AppError;

    use super::{Construct, Contact};

    #[test]
    fn test_display_name() {
        let person = Contact::new(
            "Jason",
            "Ribble",
            "john@example.com",
            "123-456-7890",
            "1970-01-01",
        );
        let display_name = "Jason Ribble".to_string();

        assert_eq!(person.unwrap().display_name, display_name)
    }

    #[test]
    fn test_contact_builder() {
        let person = Contact::builder()
            .first_name("Alice")
            .last_name("Lovelace")
            .email("ada@lovelace.com")
            .phone_number("123-321-1233")
            .birthday("1970-01-01")
            .build();

        let contact = person.unwrap();
        assert_eq!(contact.first_name, "Alice");
        assert_eq!(contact.last_name, "Lovelace");
        assert_eq!(contact.display_name, "Alice Lovelace");
        assert_eq!(contact.email, "ada@lovelace.com");
        assert_eq!(contact.phone_number, "123-321-1233");
    }

    #[test]
    fn test_contact_update_construct() {
        let edits = Construct::new(
            1,
            None,
            None,
            None,
            Some("123-233-1221".to_string()),
            Some("Nickname".to_string()),
            None,
        )
        .unwrap();

        assert_eq!(edits.id, 1);
        assert_eq!(
            edits.optional_contact.display_name,
            Some("Nickname".to_string())
        );
        assert_eq!(
            edits.optional_contact.phone_number,
            Some("123-233-1221".to_string())
        );
        assert_eq!(edits.optional_contact.first_name, None);
        assert_eq!(edits.optional_contact.last_name, None);
        assert_eq!(edits.optional_contact.email, None);
    }

    #[test]
    fn test_contact_update_construct_2() {
        let edits = Construct::new(
            2,
            Some("Mary".to_string()),
            Some("Smith".to_string()),
            Some("new@email.com".to_string()),
            None,
            None,
            None,
        )
        .unwrap();

        assert_eq!(edits.id, 2);
        assert_eq!(edits.optional_contact.first_name, Some("Mary".to_string()));
        assert_eq!(edits.optional_contact.last_name, Some("Smith".to_string()));
        assert_eq!(
            edits.optional_contact.email,
            Some("new@email.com".to_string())
        );
        assert_eq!(edits.optional_contact.phone_number, None);
        assert_eq!(edits.optional_contact.display_name, None);
    }

    #[test]
    fn test_construct_builder() {
        let edits = Construct::builder()
            .id(3)
            .first_name("John".to_string())
            .email("john@example.com".to_string())
            .phone_number("555-012-3456".to_string())
            .build()
            .unwrap();

        assert_eq!(edits.id, 3);
        assert_eq!(edits.optional_contact.first_name, Some("John".to_string()));
        assert_eq!(
            edits.optional_contact.email,
            Some("john@example.com".to_string())
        );
        assert_eq!(
            edits.optional_contact.phone_number,
            Some("555-012-3456".to_string())
        );
        assert_eq!(edits.optional_contact.last_name, None);
        assert_eq!(edits.optional_contact.display_name, None);
    }

    #[test]
    fn test_is_empty() {
        let result = Construct::new(1, None, None, None, None, None, None);
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_email_construct() {
        let result = Construct::new(
            1,
            None,
            None,
            Some("invalid@example".to_string()),
            None,
            None,
            None,
        );
        assert!(result.is_err());
        assert!(matches!(result, Err(AppError::InvalidEmail(email)) if email == "invalid@example"));
    }

    #[test]
    fn test_invalid_construct_phone_number() {
        let result = Construct::new(
            1,
            None,
            None,
            None,
            Some("123-123-12345".to_string()),
            None,
            None,
        );

        println!("{result:?}");
        assert!(result.is_err());

        assert!(
            matches!(result, Err(AppError::InvalidPhoneNumber(phone_number)) if phone_number == "123-123-12345")
        );
    }

    #[test]
    fn should_accept_a_birthdate() {
        let result = Contact::new(
            "Alice",
            "Lovelace",
            "ada@lovelace.com",
            "123-321-1233",
            "1970-01-01",
        )
        .unwrap();

        let expect_birthday = chrono::NaiveDate::from_ymd_opt(1970, 1, 1).unwrap();

        assert_eq!(result.birthday, expect_birthday);
    }

    #[test]
    fn should_return_error_when_creating_contact_with_invalid_birthday() {
        let invalid_birthday = "1970-13-32"; // Invalid date
        let contact_result = Contact::new(
            "Satoshi",
            "Nakamoto",
            "satoshi@bitcoin.org",
            "123-321-1234",
            invalid_birthday,
        );

        assert!(
            matches!(contact_result, Err(AppError::InvalidBirthday(birthday)) if birthday == invalid_birthday)
        );
    }
}
