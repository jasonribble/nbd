use std::path::Path;

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
    use super::*;

    #[test]
    fn shoud_return_error_when_invalid_extension() {
        let invalid_call = read_csv("notacsv.txt");

        assert!(invalid_call.is_err());
    }

    #[test]
    fn should_accept_csv_file() {
        let file_path = "data.csv";

        let result = read_csv(file_path);

        assert!(result.is_ok());
    }
}
