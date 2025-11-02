use chrono::NaiveDate;

/// # Panics
/// Panics if a strange date is put in with `NaiveDate::from_ymd_opt`
#[must_use]
pub const fn default_date() -> NaiveDate {
    NaiveDate::from_ymd_opt(1, 1, 1).expect("Expected default date")
}
