use companion_connect::validation::{is_valid_email, is_valid_phone_number};
use std::fmt::Display;

#[derive(Debug)]
pub struct Contact {
    pub _first_name: String,
    pub _last_name: String,
    pub display_name: String,
    pub email: Email,
    pub phone_number: PhoneNumber,
}

impl Contact {
    pub fn new(first_name: String, last_name: String, email: String, phone_number: String) -> Self {
        let display_name = format!("{first_name} {last_name}");
        let phone_number = PhoneNumber::new(phone_number).unwrap();
        let email = Email::new(email).unwrap();

        Self {
            _first_name: first_name,
            _last_name: last_name,
            display_name,
            email,
            phone_number,
        }
    }
}

#[derive(Debug)]
pub struct PhoneNumber(String);

impl PhoneNumber {
    fn new(phone_number: String) -> Result<Self, String> {
        if is_valid_phone_number(&phone_number) {
            Ok(Self(phone_number))
        } else {
            Err("Invalid phone number format".to_string())
        }
    }
}

impl Display for PhoneNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
pub struct Email(String);

impl Email {
    fn new(email: String) -> Result<Self, String> {
        if is_valid_email(&email) {
            Ok(Self(email))
        } else {
            Err("Invalid email format".to_string())
        }
    }
}

impl Display for Email {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_name() {
        let person = Contact::new(
            String::from("Jason"),
            String::from("Ribble"),
            String::from("john@example.com"),
            String::from("123-456-7890"),
        );
        let display_name = "Jason Ribble".to_string();
        assert_eq!(person.display_name, display_name)
    }
}
