use std::{fs::File, path::Path};

fn read_csv(filename: &str) -> anyhow::Result<Vec<&str>> {
    let path = Path::new(filename);
    let extension = path.extension().and_then(|ext| ext.to_str());

    if let Some(ext) = extension {
        if ext != "csv" {
            return Err(anyhow::anyhow!("File must have .csv extension"));
        }
    }

    match extension {
        Some("csv") => {
            let metadata = std::fs::metadata(path)
                .map_err(|_| anyhow::anyhow!("Failed to open file: {}", path.display()))?;

            if metadata.len() == 0 {
                return Err(anyhow::anyhow!("CSV file is empty"));
            }

            Ok(Vec::new())
        }
        _ => Err(anyhow::anyhow!("File must have .csv extension")),
    }
}

#[cfg(test)]
mod tests {
    use tempfile::NamedTempFile;

    use super::*;

    #[test]
    fn shoud_return_error_when_invalid_extension() {
        let invalid_call = read_csv("notacsv.txt");

        assert!(invalid_call.is_err());
    }

    #[test]
    fn should_accept_csv_file() {
        let temp_csv = NamedTempFile::with_suffix(".csv").unwrap();

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
}
