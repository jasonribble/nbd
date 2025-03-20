fn parse_csv(filename: &str) -> anyhow::Result<Vec<&str>> {
    Err(anyhow::anyhow!("something went wrong"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shoud_return_error_when_invalid_extension() {
        let invalid_call = parse_csv("notacsv.txt");

        // Assert
        assert!(invalid_call.is_err());
    }
}
