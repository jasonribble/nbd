use crate::utils;
use csv::Reader;
use std::path::Path;

use crate::models::OptionalContact;

/// # Errors
///
/// This function will return an error if
/// - Does not end with .csv
/// - Is an empty CSV
/// - Fails to open the file
/// - Is an invalid CSV
pub fn process_csv_to_contacts(filename: &str) -> anyhow::Result<Vec<OptionalContact>> {
    let path = Path::new(filename);

    validate_csv_extension(path)?;
    validate_csv_file(path)?;
    validate_csv_format(path)?;

    csv_to_contacts(path)
}

fn validate_csv_extension(path: &Path) -> anyhow::Result<()> {
    match path.extension().and_then(|ext| ext.to_str()) {
        Some("csv") => Ok(()),
        _ => Err(anyhow::anyhow!("File must have .csv extension")),
    }
}

fn validate_csv_file(path: &Path) -> anyhow::Result<()> {
    let metadata = std::fs::metadata(path)
        .map_err(|_| anyhow::anyhow!("Failed to open file: {}", path.display()))?;

    if metadata.len() == 0 {
        return Err(anyhow::anyhow!("CSV file is empty"));
    }
    Ok(())
}

fn validate_csv_format(path: &Path) -> anyhow::Result<()> {
    let reader = Reader::from_path(path)?;
    if !reader.into_records().all(|result| result.is_ok()) {
        return Err(anyhow::anyhow!("Invalid CSV format"));
    }
    Ok(())
}

fn csv_to_contacts(path: &Path) -> anyhow::Result<Vec<OptionalContact>> {
    let mut reader = Reader::from_path(path)?;

    let contacts: Result<Vec<OptionalContact>, csv::Error> = reader.deserialize().collect();
    let contacts = contacts?;

    let mut valid_contacts: Vec<OptionalContact> = Vec::new();

    for contact in &contacts {
        if let Some(phone_number) = &contact.phone_number {
            if utils::is_not_valid_phone_number(phone_number) {
                return Err(anyhow::anyhow!("Invalid Phone Number"));
            }
        }

        if let Some(email) = &contact.email {
            if utils::is_not_valid_email(email) {
                return Err(anyhow::anyhow!("Invalid Email"));
            }
        }

        valid_contacts.push(contact.clone());
    }
    Ok(valid_contacts)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn should_return_error_when_invalid_extension() {
        let invalid_call = process_csv_to_contacts("not_a_csv.txt");

        assert!(invalid_call.is_err());
    }

    #[test]
    fn should_accept_valid_csv_file() {
        let mut temp_csv = NamedTempFile::with_suffix(".csv").expect("Temp CSV");
        writeln!(temp_csv, "first_name\nAlice").expect("Write mock csv");

        let temp_csv = temp_csv.path().to_str().expect("The path to the CSV");
        let result = process_csv_to_contacts(temp_csv);

        assert!(result.is_ok());
    }

    #[allow(clippy::unwrap_used)]
    #[test]
    fn should_return_error_when_file_not_found() {
        let non_existent_file_path = "non_existent.csv";

        let result = process_csv_to_contacts(non_existent_file_path);

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Failed to open file: non_existent.csv"
        );
    }

    #[allow(clippy::unwrap_used)]
    #[test]
    fn should_error_if_file_is_empty() {
        let temp_csv = NamedTempFile::with_suffix(".csv").unwrap();

        let temp_csv = temp_csv.path().to_str().unwrap();
        let result = process_csv_to_contacts(temp_csv);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "CSV file is empty");
    }

    #[allow(clippy::unwrap_used)]
    #[test]
    fn should_error_if_invalid_csv_format() {
        let mut temp_csv = NamedTempFile::with_suffix(".csv").unwrap();

        let malformed_csv = "first_name,phone_number,email\nAlice,1234567890";
        write!(temp_csv, "{malformed_csv}").unwrap();

        let temp_csv = temp_csv.path().to_str().unwrap();
        let result = process_csv_to_contacts(temp_csv);

        match result {
            Ok(_) => panic!("Expected invalid CSV, but was valid"),
            Err(e) => assert_eq!(e.to_string(), "Invalid CSV format"),
        }
    }

    #[allow(clippy::unwrap_used)]
    #[test]
    fn should_error_when_given_an_invalid_phone_number() {
        let mut temp_csv = NamedTempFile::with_suffix(".csv").unwrap();

        let malformed_csv = "first_name,phone_number\nAlice,not_a_phone_number";
        write!(temp_csv, "{malformed_csv}").unwrap();

        let temp_csv = temp_csv.path().to_str().unwrap();
        let result = process_csv_to_contacts(temp_csv);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "Invalid Phone Number");
    }

    #[allow(clippy::unwrap_used)]
    #[test]
    fn should_error_when_given_an_invalid_email() {
        let mut temp_csv = NamedTempFile::with_suffix(".csv").unwrap();

        let malformed_email_csv = "first_name,email\nAlice,invalid@email";
        write!(temp_csv, "{malformed_email_csv}").unwrap();

        let temp_csv_path = temp_csv.path().to_str().unwrap();

        let contacts = process_csv_to_contacts(temp_csv_path);

        assert!(contacts.is_err());
        assert_eq!(contacts.unwrap_err().to_string(), "Invalid Email");
    }

    #[allow(clippy::unwrap_used)]
    #[test]
    fn should_read_csv_with_multiple_rows() -> anyhow::Result<()> {
        let mut temp_csv = NamedTempFile::with_suffix(".csv")?;
        let three_contacts =
            "first_name,phone_number\nAlice,1234567890\nBob,0989878721\nCharlie,1989878721";

        writeln!(temp_csv, "{three_contacts}")?;

        let temp_csv = temp_csv.path().to_str().unwrap();
        let result = process_csv_to_contacts(temp_csv)?;

        assert_eq!(result.len(), 3);

        Ok(())
    }

    #[test]
    fn should_return_contact_when_given_csv() -> anyhow::Result<()> {
        let mut temp_csv = NamedTempFile::with_suffix(".csv")?;
        let alice_firstname_and_phone = "first_name,phone_number\nAlice,1234567890";

        writeln!(temp_csv, "{alice_firstname_and_phone}")?;

        let temp_csv = temp_csv.path();
        let contacts = csv_to_contacts(temp_csv);

        let alice = &contacts.expect("At least one contact")[0];
        let expected_contact = OptionalContact {
            first_name: Some("Alice".to_string()),
            phone_number: Some("1234567890".to_string()),
            ..OptionalContact::template()
        };

        assert_eq!(alice, &expected_contact);

        Ok(())
    }

    #[test]
    fn should_return_multiple_contacts_when_given_csv() -> anyhow::Result<()> {
        let mut temp_csv = NamedTempFile::with_suffix(".csv")?;
        let three_contacts =
            "first_name,phone_number\nAlice,1234567890\nBob,0989878721\nCharlie,1989878721";

        writeln!(temp_csv, "{three_contacts}")?;

        let temp_csv = temp_csv.path();
        let contacts = csv_to_contacts(temp_csv)?;

        let expected_contacts = vec![
            OptionalContact {
                first_name: Some("Alice".to_string()),
                phone_number: Some("1234567890".to_string()),
                ..OptionalContact::template()
            },
            OptionalContact {
                first_name: Some("Bob".to_string()),
                phone_number: Some("0989878721".to_string()),
                ..OptionalContact::template()
            },
            OptionalContact {
                first_name: Some("Charlie".to_string()),
                phone_number: Some("1989878721".to_string()),
                ..OptionalContact::template()
            },
        ];

        for (index, expected_contact) in expected_contacts.iter().enumerate() {
            assert_eq!(&contacts[index], expected_contact);
        }

        Ok(())
    }

    #[test]
    #[ignore = "deferring feature"]
    fn should_accept_csv_that_has_iso8601_birthday() -> anyhow::Result<()> {
        let mut temp_csv = NamedTempFile::with_suffix(".csv")?;
        let first_name_iso8601_birthday = "first_name,birthday\nAlice,1987-07-11T00:00:00Z";

        writeln!(temp_csv, "{first_name_iso8601_birthday}")?;

        let temp_csv = temp_csv.path().to_str().expect("Path to csv");
        let result = process_csv_to_contacts(temp_csv);

        println!("{result:?}");

        assert!(result.is_ok());

        Ok(())
    }
}
