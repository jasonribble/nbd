use std::fmt::Display;
use rusqlite::types::{FromSql, FromSqlResult, ToSqlOutput, ValueRef, ToSql};

use crate::validation::is_valid_phone_number;

#[derive(Debug, Eq, PartialEq)]
pub struct PhoneNumber(String);

impl PhoneNumber {
    pub(crate) fn new(phone_number: String) -> Result<Self, String> {
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

impl ToSql for PhoneNumber {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        Ok(ToSqlOutput::Borrowed(ValueRef::Text(self.0.as_bytes())))
    }
}

#[cfg(test)]
mod tests {
    use rusqlite::types::{FromSql, ToSql, ToSqlOutput, ValueRef};
    use crate::phone_number::PhoneNumber;

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
    fn test_phone_number_from_sql_valid() {
        let value = ValueRef::Text(b"1234567890");
        let result = PhoneNumber::column_result(value);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), PhoneNumber("1234567890".to_string()));
    }

    #[test]
    fn test_phone_number_display() {
        let phone = PhoneNumber("1234567890".to_string());
        assert_eq!(format!("{}", phone), "1234567890");
    }

    #[test]
    fn test_phone_number_to_sql() {
        let phone = PhoneNumber("1234567890".to_string());
        let result = phone.to_sql();
        assert!(result.is_ok());
        if let Ok(ToSqlOutput::Borrowed(ValueRef::Text(bytes))) = result {
            assert_eq!(bytes, b"1234567890");
        } else {
            panic!("Unexpected ToSqlOutput variant");
        }
    }
}
