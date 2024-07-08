use crate::{errors::AppError, utils};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Contact {
    pub first_name: String,
    pub last_name: String,
    pub display_name: String,
    pub email: String,
    pub phone_number: String,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ContactWithId {
    pub id: i64,
    pub contact: Contact,
}

impl Contact {
    pub fn new(
        first_name: String,
        last_name: String,
        email: String,
        phone_number: String,
    ) -> Result<Self, AppError> {
        let display_name = format!("{first_name} {last_name}");

        if utils::is_not_valid_email(&email) {
            return Err(AppError::InvalidEmail(email));
        }

        if utils::is_not_valid_phone_number(&phone_number) {
            return Err(AppError::InvalidPhoneNumber(phone_number));
        }

        Ok(Self {
            first_name,
            last_name,
            display_name,
            email,
            phone_number,
        })
    }
}
#[cfg(test)]
mod tests {
    use super::Contact;

    #[test]
    fn test_display_name() {
        let person = Contact::new(
            String::from("Jason"),
            String::from("Ribble"),
            String::from("john@example.com"),
            String::from("123-456-7890"),
        );
        let display_name = "Jason Ribble".to_string();

        assert_eq!(person.unwrap().display_name, display_name)
    }
}
