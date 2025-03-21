fn read_csv(filename: &str) -> anyhow::Result<Vec<&str>> {
    Err(anyhow::anyhow!("something went wrong"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shoud_return_error_when_invalid_extension() {
        let invalid_call = read_csv("notacsv.txt");

        // Assert
        assert!(invalid_call.is_err());
    }

    #[test]
    fn should_return() {}
}
