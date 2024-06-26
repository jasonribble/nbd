use sqlx::{
    sqlite::{SqliteArgumentValue, SqliteTypeInfo},
    Encode, Type,
};
use std::{borrow::Cow, fmt::Display};

use crate::utils;

#[derive(Debug, Eq, PartialEq)]
pub struct Email(String);

impl Email {
    pub(crate) fn new(email: String) -> Result<Self, String> {
        if utils::is_valid_email(&email) {
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

impl<'q> Encode<'q, sqlx::Sqlite> for &'q Email {
    fn encode_by_ref(&self, args: &mut Vec<SqliteArgumentValue<'q>>) -> sqlx::encode::IsNull {
        args.push(SqliteArgumentValue::Text(Cow::Borrowed(self.0.as_str())));
        sqlx::encode::IsNull::No
    }
}

impl Type<sqlx::Sqlite> for Email {
    fn type_info() -> SqliteTypeInfo {
        <&str as Type<sqlx::Sqlite>>::type_info()
    }

    fn compatible(ty: &SqliteTypeInfo) -> bool {
        <&str as Type<sqlx::Sqlite>>::compatible(ty)
    }
}

#[cfg(test)]
mod tests {
    use super::Email;

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
}
