use csv::Reader;
use std::path::Path;

fn read_csv(filename: &str) -> anyhow::Result<Vec<String>> {
    let path = Path::new(filename);

    validate_csv_extension(path)?;
    validate_csv_file(path)?;
    validate_csv_format(path)?;

    let mut reader = Reader::from_path(filename)?;
    let mut records = Vec::new();

    for result in reader.records() {
        let record = result?;
        records.push(record[0].to_string());
    }

    Ok(records)
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

#[cfg(test)]
mod tests {
    use std::io::Write;
    use tempfile::NamedTempFile;

    use super::*;

    #[test]
    fn shoud_return_error_when_invalid_extension() {
        let invalid_call = read_csv("notacsv.txt");

        assert!(invalid_call.is_err());
    }

    #[test]
    fn should_accept_valid_csv_file() {
        let mut temp_csv = NamedTempFile::with_suffix(".csv").unwrap();
        writeln!(temp_csv, "first_name\nAlice").unwrap();

        let result = read_csv(temp_csv.path().to_str().unwrap());

        assert!(result.is_ok());
    }

    #[test]
    fn should_return_error_when_file_not_found() {
        let non_exisistent_file_path = "non_existent.csv";

        let result = read_csv(non_exisistent_file_path);

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Failed to open file: non_existent.csv"
        );
    }

    #[test]
    fn should_error_if_file_is_empty() {
        let temp_csv = NamedTempFile::with_suffix(".csv").unwrap();

        let result = read_csv(temp_csv.path().to_str().unwrap());

        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "CSV file is empty");
    }

    #[test]
    fn should_error_if_invalid_csv_format() {
        let mut temp_csv = NamedTempFile::with_suffix(".csv").unwrap();

        let malformed_csv = "name,age,city\nAlice,30";
        write!(temp_csv, "{}", malformed_csv).unwrap();

        let result = read_csv(temp_csv.path().to_str().unwrap());

        match result {
            Ok(_) => panic!("Expected invalid CSV, but was valid"),
            Err(e) => assert_eq!(e.to_string(), "Invalid CSV format"),
        }
    }

    #[test]
    fn test_read_csv_with_multiple_rows() -> anyhow::Result<()> {
        let mut temp_csv = NamedTempFile::with_suffix(".csv")?;
        let name_age_three_rows_content = "name,age\nAlice,30\nBob,25\nCharlie,35";

        writeln!(temp_csv, "{}", name_age_three_rows_content)?;
        let result = read_csv(temp_csv.path().to_str().unwrap())?;

        assert_eq!(result.len(), 3);

        Ok(())
    }
}
