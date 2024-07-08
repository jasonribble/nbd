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
    use super::Contact;

    #[test]
    fn test_display_name() {
        let person = Contact::new("Jason", "Ribble", "john@example.com", "123-456-7890");
        let display_name = "Jason Ribble".to_string();

        assert_eq!(person.unwrap().display_name, display_name)
    }
}
