use sqlx::{
    postgres::{PgArgumentBuffer, PgTypeInfo, PgValueRef},
    Encode, Type,
};
use std::fmt::Display;

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

impl<'q> Encode<'q, sqlx::Postgres> for &'q Email {
    fn encode_by_ref(&self, buf: &mut PgArgumentBuffer) -> sqlx::encode::IsNull {
        <&str as Encode<sqlx::Postgres>>::encode(self.0.as_str(), buf)
    }
}

impl Type<sqlx::Postgres> for Email {
    fn type_info() -> PgTypeInfo {
        <&str as Type<sqlx::Postgres>>::type_info()
    }

    fn compatible(ty: &PgTypeInfo) -> bool {
        <&str as Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl<'r> sqlx::Decode<'r, sqlx::Postgres> for Email {
    fn decode(value: PgValueRef<'r>) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let s = <&str as sqlx::Decode<sqlx::Postgres>>::decode(value)?;
        Ok(Email(s.to_string()))
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
