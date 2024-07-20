#[cfg(test)]

mod tests {
    use assert_cmd::Command;

    #[test]
    fn test_help() {
        let mut cmd = Command::cargo_bin("connect").unwrap();

        cmd.arg("--help");

        cmd.assert()
            .success()
            .stdout(predicates::str::contains("Usage: connect <COMMAND>"));
    }

    #[test]
    fn test_connect_works() {
        let mut cmd = Command::cargo_bin("connect").unwrap();

        cmd.arg("create")
            .arg("--first-name")
            .arg("First")
            .arg("--last-name")
            .arg("Last")
            .arg("--email")
            .arg("test@test.com")
            .arg("--phone-number")
            .arg("123-321-1233");

        cmd.assert()
            .success()
            .stdout(predicates::str::contains("Successfully saved contact."));
    }

    #[test]
    fn test_connect_invalid_email() {
        let mut cmd = Command::cargo_bin("connect").unwrap();

        cmd.arg("create")
            .arg("--first-name")
            .arg("First")
            .arg("--last-name")
            .arg("Last")
            .arg("--email")
            .arg("test@.com")
            .arg("--phone-number")
            .arg("123-321-1233");

        cmd.assert()
            .failure()
            .stderr(predicates::str::contains("InvalidEmail"));
    }

    #[test]
    fn test_connect_invalid_phone() {
        let mut cmd = Command::cargo_bin("connect").unwrap();

        cmd.arg("create")
            .arg("--first-name")
            .arg("First")
            .arg("--last-name")
            .arg("Last")
            .arg("--email")
            .arg("test@com.com")
            .arg("--phone-number")
            .arg("123-321-123");

        cmd.assert()
            .failure()
            .stderr(predicates::str::contains("InvalidPhone"));
    }

    #[test]
    fn test_connect_invalid_args() {
        let mut cmd = Command::cargo_bin("connect").unwrap();

        cmd.arg("First").arg("Last").arg("32321123");

        cmd.assert()
            .failure()
            .stderr(predicates::str::contains("Usage: connect <COMMAND>"));
    }
}
