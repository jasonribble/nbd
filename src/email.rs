use std::fmt::Display;
use rusqlite::types::{FromSql, FromSqlResult, ValueRef};
use companion_connect::validation::is_valid_email;

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