use std::fmt::Display;
use rusqlite::{types::{FromSql, FromSqlResult, ToSqlOutput, ValueRef}, ToSql};

use crate::utils::is_valid_email;

#[derive(Debug, Eq, PartialEq)]
pub struct Email(String);

impl Email {
    pub (crate) fn new(email: String) -> Result<Self, String> {
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

impl ToSql for Email {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        Ok(ToSqlOutput::Borrowed(ValueRef::Text(self.0.as_bytes())))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::types::{FromSql, ToSql, ToSqlOutput, Value};


    #[test]
    fn test_email_creation_valid() {
        let email = Email::new("user@example.com".to_string());
        assert!(email.is_ok());
        assert_eq!(email.unwrap().to_string(), "user@example.com");
    }

    #[test]
    fn test_email_creation_invalid() {
        let email = Email::new("invalid_email".to_string());
        assert!(email.is_err());
        assert_eq!(email.unwrap_err(), "Invalid email format");
    }

    #[test]
    fn test_email_display() {
        let email = Email::new("user@example.com".to_string()).unwrap();
        assert_eq!(format!("{}", email), "user@example.com");
    }

    #[test]
    fn test_email_from_sql() {
        let value = Value::Text("user@example.com".to_string());
        let email = Email::column_result(ValueRef::from(&value));
        assert!(email.is_ok());
        assert_eq!(email.unwrap().to_string(), "user@example.com");
    }

    #[test]
    fn test_email_number_to_sql() {
        let email = Email("user@example.com".to_string());
        let result = email.to_sql();
        assert!(result.is_ok());
        if let Ok(ToSqlOutput::Borrowed(ValueRef::Text(bytes))) = result {
            assert_eq!(bytes, b"user@example.com");
        } else {
            panic!("Unexpected ToSqlOutput variant");
        }
    }
}