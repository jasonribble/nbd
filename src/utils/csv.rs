use csv::Reader;
use std::path::Path;

use crate::models::OptionalContact;

fn process_csv_to_contacts(filename: &str) -> anyhow::Result<Vec<OptionalContact>> {
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

    Ok(contacts?)
}

#[cfg(test)]
mod tests {
    use std::io::Write;
    use tempfile::NamedTempFile;

    use super::*;

    #[test]
    fn shoud_return_error_when_invalid_extension() {
        let invalid_call = process_csv_to_contacts("notacsv.txt");

        assert!(invalid_call.is_err());
    }

    #[test]
    fn should_accept_valid_csv_file() {
        let mut temp_csv = NamedTempFile::with_suffix(".csv").unwrap();
        writeln!(temp_csv, "first_name\nAlice").unwrap();

        let temp_csv = temp_csv.path().to_str().unwrap();
        let result = process_csv_to_contacts(temp_csv);

        assert!(result.is_ok());
    }

    #[test]
    fn should_return_error_when_file_not_found() {
        let non_exisistent_file_path = "non_existent.csv";

        let result = process_csv_to_contacts(non_exisistent_file_path);

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Failed to open file: non_existent.csv"
        );
    }

    #[test]
    fn should_error_if_file_is_empty() {
        let temp_csv = NamedTempFile::with_suffix(".csv").unwrap();

        let temp_csv = temp_csv.path().to_str().unwrap();
        let result = process_csv_to_contacts(temp_csv);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "CSV file is empty");
    }

    #[test]
    fn should_error_if_invalid_csv_format() {
        let mut temp_csv = NamedTempFile::with_suffix(".csv").unwrap();

        let malformed_csv = "first_name,phone_number,email\nAlice,1234567890";
        write!(temp_csv, "{}", malformed_csv).unwrap();

        let temp_csv = temp_csv.path().to_str().unwrap();
        let result = process_csv_to_contacts(temp_csv);

        match result {
            Ok(_) => panic!("Expected invalid CSV, but was valid"),
            Err(e) => assert_eq!(e.to_string(), "Invalid CSV format"),
        }
    }

    #[test]
    fn should_read_csv_with_multiple_rows() -> anyhow::Result<()> {
        let mut temp_csv = NamedTempFile::with_suffix(".csv")?;
        let three_contacts =
            "first_name,phone_number\nAlice,1234567890\nBob,0989878721\nCharlie,1989878721";

        writeln!(temp_csv, "{}", three_contacts)?;

        let temp_csv = temp_csv.path().to_str().unwrap();
        let result = process_csv_to_contacts(temp_csv)?;

        assert_eq!(result.len(), 3);

        Ok(())
    }

    fn default_contact() -> OptionalContact {
        OptionalContact {
            first_name: None,
            last_name: None,
            display_name: None,
            email: None,
            phone_number: None,
        }
    }

    #[test]
    fn should_return_contact_when_given_csv() -> anyhow::Result<()> {
        let mut temp_csv = NamedTempFile::with_suffix(".csv")?;
        let alice_firstname_and_phone = "first_name,phone_number\nAlice,1234567890";

        writeln!(temp_csv, "{}", alice_firstname_and_phone)?;

        let temp_csv = temp_csv.path();
        let contacts = csv_to_contacts(temp_csv);

        let alice = &contacts.unwrap()[0];
        let expected_contact = OptionalContact {
            first_name: Some("Alice".to_string()),
            phone_number: Some("1234567890".to_string()),
            ..default_contact()
        };

        assert_eq!(alice, &expected_contact);

        Ok(())
    }

    #[test]
    fn should_return_mutilple_contacts_when_given_csv() -> anyhow::Result<()> {
        let mut temp_csv = NamedTempFile::with_suffix(".csv")?;
        let three_contacts =
            "first_name,phone_number\nAlice,1234567890\nBob,0989878721\nCharlie,1989878721";

        writeln!(temp_csv, "{}", three_contacts)?;

        let temp_csv = temp_csv.path();
        let contacts = csv_to_contacts(temp_csv).unwrap();

        let alice = &contacts[0];
        let expected_alice_contact = OptionalContact {
            first_name: Some("Alice".to_string()),
            phone_number: Some("1234567890".to_string()),
            ..default_contact()
        };

        assert_eq!(alice, &expected_alice_contact);

        let bob = &contacts[1];
        let expected_contact = OptionalContact {
            first_name: Some("Bob".to_string()),
            phone_number: Some("0989878721".to_string()),
            ..default_contact()
        };

        assert_eq!(bob, &expected_contact);

        let charlie = &contacts[2];
        let charlie_expected_contact = OptionalContact {
            first_name: Some("Charlie".to_string()),
            phone_number: Some("1989878721".to_string()),
            ..default_contact()
        };

        assert_eq!(charlie, &charlie_expected_contact);

        Ok(())
    }
}
