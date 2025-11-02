mod csv;
mod date;
mod errors;
mod validation;

pub use csv::process_csv_to_contacts;
pub use date::default_date;
pub use errors::AppError;
pub use validation::{is_not_valid_email, is_not_valid_phone_number};
