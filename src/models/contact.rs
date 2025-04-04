use crate::utils;
use crate::utils::AppError;
use chrono::NaiveDate;
use tabled::Tabled;

#[derive(Debug, PartialEq, Eq, Clone, sqlx::FromRow, Tabled)]
pub struct Contact {
    pub first_name: String,
    pub last_name: String,
    pub display_name: String,
    pub email: String,
    pub phone_number: String,
    pub birthday: NaiveDate,
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
}

impl Optional {
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.first_name.is_none()
            && self.last_name.is_none()
            && self.display_name.is_none()
            && self.email.is_none()
            && self.phone_number.is_none()
    }

    #[must_use]
    pub const fn template() -> Self {
        Self {
            first_name: None,
            last_name: None,
            display_name: None,
            email: None,
            phone_number: None,
        }
    }
}

#[derive(Debug)]
pub struct Construct {
    pub id: i64,
    pub optional_contact: Optional,
}
impl Construct {
    /// # Errors
    ///
    /// This errors if there is an invalid email or phone number
    pub fn new(
        id: i64,
        first_name: Option<String>,
        last_name: Option<String>,
        email: Option<String>,
        phone_number: Option<String>,
        display_name: Option<String>,
    ) -> Result<Self, AppError> {
        let maybe_email = email.as_deref().unwrap_or("");

        if utils::is_not_valid_email(maybe_email) && Option::is_some(&email) {
            return Err(AppError::InvalidEmail(email.clone().unwrap_or_default()));
        }

        let maybe_phone = phone_number.as_deref().unwrap_or("");

        if utils::is_not_valid_phone_number(maybe_phone) && Option::is_some(&phone_number) {
            return Err(AppError::InvalidPhoneNumber(
                phone_number.clone().unwrap_or_default(),
            ));
        }

        let optional_contact = Optional {
            first_name,
            last_name,
            display_name,
            email,
            phone_number,
        };

        if optional_contact.is_empty() {
            return Err(AppError::EmptyUpdate);
        }

        Ok(Self {
            id,
            optional_contact,
        })
    }

    #[allow(dead_code)]
    const fn is_empty(&self) -> bool {
        self.optional_contact.first_name.is_none()
            && self.optional_contact.last_name.is_none()
            && self.optional_contact.display_name.is_none()
            && self.optional_contact.email.is_none()
            && self.optional_contact.phone_number.is_none()
    }
}

impl Contact {
    /// # Errors
    ///
    /// This errors if there is an invalid email or phone number
    pub fn new(
        first_name: &str,
        last_name: &str,
        email: &str,
        phone_number: &str,
        birthday: NaiveDate,
    ) -> Result<Self, AppError> {
        let display_name = format!("{first_name} {last_name}");

        if utils::is_not_valid_email(email) {
            return Err(AppError::InvalidEmail(email.to_owned()));
        }

        if utils::is_not_valid_phone_number(phone_number) {
            return Err(AppError::InvalidPhoneNumber(phone_number.to_owned()));
        }

        Ok(Self {
            first_name: first_name.to_owned(),
            last_name: last_name.to_owned(),
            display_name,
            email: email.to_owned(),
            phone_number: phone_number.to_owned(),
            birthday,
        })
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
            chrono::NaiveDate::default(),
        );
        let display_name = "Jason Ribble".to_string();

        assert_eq!(person.unwrap().display_name, display_name)
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
    fn test_is_empty() {
        let result = Construct::new(1, None, None, None, None, None);
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
        );
        assert!(result.is_err());
        assert!(matches!(result, Err(AppError::InvalidEmail(email)) if email == "invalid@example"));
    }

    #[test]
    fn test_invalid_construct_phone_number() {
        let result = Construct::new(1, None, None, None, Some("123-123-12345".to_string()), None);

        println!("{result:?}");
        assert!(result.is_err());

        assert!(
            matches!(result, Err(AppError::InvalidPhoneNumber(phone_number)) if phone_number == "123-123-12345")
        );
    }

    #[test]
    fn should_accept_a_birthdate() {
        let birthday = chrono::NaiveDate::from_ymd_opt(1990, 5, 15).unwrap();
        let result = Contact::new(
            "Alice",
            "Lovelace",
            "ada@lovelace.com",
            "123-321-1233",
            birthday,
        )
        .unwrap();

        assert_eq!(result.birthday, birthday);
    }
}
