use sqlx::{
    postgres::{PgArgumentBuffer, PgTypeInfo, PgValueRef},
    Encode, Type,
};
use std::fmt::Display;

use crate::utils;

#[derive(Debug, Eq, PartialEq)]
pub struct PhoneNumber(String);

impl PhoneNumber {
    pub(crate) fn new(phone_number: String) -> Result<Self, String> {
        if utils::is_valid_phone_number(&phone_number) {
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

impl<'q> Encode<'q, sqlx::Postgres> for &'q PhoneNumber {
    fn encode_by_ref(&self, buf: &mut PgArgumentBuffer) -> sqlx::encode::IsNull {
        <&str as Encode<sqlx::Postgres>>::encode(self.0.as_str(), buf)
    }
}

impl Type<sqlx::Postgres> for PhoneNumber {
    fn type_info() -> PgTypeInfo {
        <&str as Type<sqlx::Postgres>>::type_info()
    }

    fn compatible(ty: &PgTypeInfo) -> bool {
        <&str as Type<sqlx::Postgres>>::compatible(ty)
    }
}

impl<'r> sqlx::Decode<'r, sqlx::Postgres> for PhoneNumber {
    fn decode(
        value: PgValueRef<'r>,
    ) -> Result<Self, Box<dyn std::error::Error + 'static + Send + Sync>> {
        let s = <&str as sqlx::Decode<sqlx::Postgres>>::decode(value)?;
        Ok(Self(s.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::PhoneNumber;

    #[test]
    fn test_phone_number_new_valid() {
        let valid_number = "1234567890".to_string();
        assert!(PhoneNumber::new(valid_number).is_ok());
    }

    #[test]
    fn test_phone_number_new_invalid() {
        let invalid_number = "123".to_string();
        assert!(PhoneNumber::new(invalid_number).is_err());
    }

    #[test]
    fn test_phone_number_display() {
        let phone = PhoneNumber("1234567890".to_string());
        assert_eq!(format!("{}", phone), "1234567890");
    }
}
