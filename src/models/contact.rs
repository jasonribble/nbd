use crate::{errors::AppError, utils};

#[derive(Debug, PartialEq, Eq, Clone, sqlx::FromRow)]
pub struct Contact {
    pub first_name: String,
    pub last_name: String,
    pub display_name: String,
    pub email: String,
    pub phone_number: String,
}

#[derive(Debug, PartialEq, Eq, Clone, sqlx::FromRow)]
pub struct Indexed {
    pub id: i64,
    #[sqlx(flatten)]
    pub contact: Contact,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Update {
    first_name: Option<String>,
    last_name: Option<String>,
    display_name: Option<String>,
    email: Option<String>,
    phone_number: Option<String>,
}

#[derive(Debug)]
pub struct Builder {
    id: i64,
    update: Update,
    errors: Vec<AppError>,
}
impl Builder {
    pub const fn new(id: i64) -> Self {
        Self {
            id,
            update: Update {
                first_name: None,
                last_name: None,
                display_name: None,
                email: None,
                phone_number: None,
            },
            errors: Vec::new(),
        }
    }

    pub const fn is_empty(&self) -> bool {
        self.update.first_name.is_none()
            && self.update.last_name.is_none()
            && self.update.display_name.is_none()
            && self.update.email.is_none()
            && self.update.phone_number.is_none()
    }

    pub fn first_name(mut self, first_name: &str) -> Self {
        self.update.first_name = Some(first_name.to_string());
        self
    }

    pub fn last_name(mut self, last_name: &str) -> Self {
        self.update.last_name = Some(last_name.to_string());
        self
    }

    pub fn email(mut self, email: &str) -> Self {
        if utils::is_not_valid_email(email) {
            self.errors.push(AppError::InvalidEmail(email.to_string()))
        }
        self.update.email = Some(email.to_string());
        self
    }

    pub fn display_name(mut self, display_name: &str) -> Self {
        self.update.display_name = Some(display_name.to_string());
        self
    }

    pub fn phone_number(mut self, phone_number: &str) -> Self {
        if utils::is_not_valid_phone_number(phone_number) {
            self.errors
                .push(AppError::InvalidPhoneNumber(phone_number.to_string()))
        }
        self.update.phone_number = Some(phone_number.to_string());
        self
    }

    pub fn build(self) -> Result<Self, Vec<AppError>> {
        assert!(!self.is_empty(), "At least one field must be set");

        if !self.errors.is_empty() {
            return Err(self.errors);
        }

        Ok(Self {
            id: self.id,
            update: self.update,
            errors: self.errors,
        })
    }
}

impl Contact {
    pub fn new(
        first_name: &str,
        last_name: &str,
        email: &str,
        phone_number: &str,
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
        })
    }
}
#[cfg(test)]
mod tests {
    use super::{Builder, Contact};

    #[test]
    fn test_display_name() {
        let person = Contact::new("Jason", "Ribble", "john@example.com", "123-456-7890");
        let display_name = "Jason Ribble".to_string();

        assert_eq!(person.unwrap().display_name, display_name)
    }

    #[test]
    fn test_contact_update_builder() {
        let edits = Builder::new(1)
            .display_name("Nickname")
            .phone_number("123-233-1221")
            .build()
            .unwrap();

        assert_eq!(edits.id, 1);
        assert_eq!(edits.update.display_name, Some("Nickname".to_string()));
        assert_eq!(edits.update.phone_number, Some("123-233-1221".to_string()));
        assert_eq!(edits.update.first_name, None);
        assert_eq!(edits.update.last_name, None);
        assert_eq!(edits.update.email, None);
    }

    #[test]
    fn test_contact_update_builder_2() {
        let edits = Builder::new(2)
            .first_name("Mary")
            .last_name("Smith")
            .email("new@email.com")
            .build()
            .unwrap();

        assert_eq!(edits.id, 2);
        assert_eq!(edits.update.first_name, Some("Mary".to_string()));
        assert_eq!(edits.update.last_name, Some("Smith".to_string()));
        assert_eq!(edits.update.email, Some("new@email.com".to_string()));
        assert_eq!(edits.update.phone_number, None);
        assert_eq!(edits.update.display_name, None);
    }

    #[test]
    #[should_panic(expected = "At least one field must be set")]
    fn test_update_builder_must_have_one() {
        let _ = Builder::new(1).build();
    }

    #[test]
    fn test_is_empty() {
        let contact = Builder::new(1);
        assert!(contact.is_empty());
    }

    #[test]
    #[ignore]
    #[should_panic(expected = "Must be built")]
    fn test_new_update_must_have_one() {
        let _ = Builder::new(1);
    }

    #[test]
    fn test_invalid_email_builder() {
        let result = Builder::new(1).email("invalid@example").build();
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_builder_phone_number() {
        let result = Builder::new(1).phone_number("invalid number").build();
        assert!(result.is_err());
    }
}
