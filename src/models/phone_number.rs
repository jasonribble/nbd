use sqlx::{
    sqlite::{SqliteArgumentValue, SqliteTypeInfo},
    Encode, Type,
};
use std::{borrow::Cow, fmt::Display};

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

impl<'q> Encode<'q, sqlx::Sqlite> for &'q PhoneNumber {
    fn encode_by_ref(&self, args: &mut Vec<SqliteArgumentValue<'q>>) -> sqlx::encode::IsNull {
        args.push(SqliteArgumentValue::Text(Cow::Borrowed(self.0.as_str())));
        sqlx::encode::IsNull::No
    }
}

impl Type<sqlx::Sqlite> for PhoneNumber {
    fn type_info() -> SqliteTypeInfo {
        <&str as Type<sqlx::Sqlite>>::type_info()
    }

    fn compatible(ty: &SqliteTypeInfo) -> bool {
        <&str as Type<sqlx::Sqlite>>::compatible(ty)
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
