use companion_connect::validation::{is_valid_email, is_valid_phone_number};
use std::fmt::Display;
use rusqlite::types::{FromSql, FromSqlResult, ValueRef};

#[derive(Debug)]
pub struct Contact {
    pub first_name: String,
    pub last_name: String,
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
            first_name,
            last_name,
            display_name,
            email,
            phone_number,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
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

impl FromSql for PhoneNumber {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        value.as_str().map(|s| Self(s.to_string()))
    }
}

impl Display for PhoneNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Eq, PartialEq)]
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

impl FromSql for Email {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        value.as_str().map(|s| Self(s.to_string()))
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
