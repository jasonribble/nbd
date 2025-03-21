use std::{fs::File, path::Path};

fn read_csv(filename: &str) -> anyhow::Result<Vec<&str>> {
    let path = Path::new(filename);
    let extension = path.extension().and_then(|ext| ext.to_str());

    match extension {
        Some("csv") => Ok(Vec::new()),
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
}
