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
pub struct Update {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub display_name: Option<String>,
    pub email: Option<String>,
    pub phone_number: Option<String>,
}

#[derive(Debug)]
pub struct Builder {
    pub id: i64,
    pub update: Update,
    errors: Vec<AppError>,
}
impl Builder {
    pub fn new(
        id: i64,
        first_name:Option<String>,
        last_name:Option<String>,
        email:Option<String>,
        phone_number:Option<String>,
        display_name: Option<String>,
    ) -> Result<Self, AppError> {

        let maybe_email = email.as_deref().unwrap_or("");

        if utils::is_not_valid_email(maybe_email) || email != None {
            return Err(AppError::InvalidEmail(email.clone().unwrap_or_else(|| String::from(""))));
        }

        let maybe_phone = phone_number.as_deref().unwrap_or("");

        if utils::is_not_valid_phone_number(maybe_phone) || phone_number != None  {
            return Err(AppError::InvalidPhoneNumber(phone_number.clone().unwrap_or_else(|| String::from(""))));
        }

        Ok(Self {
            id,
            update: Update {
                first_name,
                last_name,
                display_name,
                email,
                phone_number,
            },
            errors: Vec::new(),
        })
    }

    pub const fn is_empty(&self) -> bool {
        self.update.first_name.is_none()
            && self.update.last_name.is_none()
            && self.update.display_name.is_none()
            && self.update.email.is_none()
            && self.update.phone_number.is_none()
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
        let edits = Builder::new(1, None, None, Some("test@test.com".to_string()), None, Some("123-233-1221".to_string())).unwrap();

        assert_eq!(edits.id, 1);
        assert_eq!(edits.update.display_name, Some("Nickname".to_string()));
        assert_eq!(edits.update.phone_number, Some("123-233-1221".to_string()));
        assert_eq!(edits.update.first_name, None);
        assert_eq!(edits.update.last_name, None);
        assert_eq!(edits.update.email, None);
    }

    #[test]
    fn test_contact_update_builder_2() {
        let edits = Builder::new(2, Some("Mary".to_string()), Some("Smith".to_string()), None, Some("new@email.com".to_string()), None).unwrap();

        assert_eq!(edits.id, 2);
        assert_eq!(edits.update.first_name, Some("Mary".to_string()));
        assert_eq!(edits.update.last_name, Some("Smith".to_string()));
        assert_eq!(edits.update.email, Some("new@email.com".to_string()));
        assert_eq!(edits.update.phone_number, None);
        assert_eq!(edits.update.display_name, None);
    }

    #[test]
    fn test_is_empty() {
        let contact = Builder::new(1, None, None, None, None, None).unwrap();
        assert!(contact.is_empty());
    }

    #[test]
    fn test_is_empty_error() {
        let result = Builder::new(1, None, None, None, None, None);
        assert!(result.is_err());
    }


    #[test]
    fn test_invalid_email_builder() {
        let result: Result<Builder, crate::errors::AppError> = Builder::new(1, None, None, None, Some("invalid@example".to_string()), None);
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_builder_phone_number() {
        let result = Builder::new(1, None, None, None, None, Some("123-123-1234".to_string()));
        assert!(result.is_err());
    }
}
