#[cfg(test)]

mod tests {
    use assert_cmd::Command;

    fn create_command() -> Command {
        Command::cargo_bin(get_cli_name()).unwrap()
    }

    fn get_cli_name() -> String {
        let package_name = env!("CARGO_PKG_NAME");
        let cli_name = format!("{}-cli", package_name);
        cli_name.to_string()
    }

    #[test]
    fn test_cli_name() {
        let crate_name = get_cli_name();
        assert_eq!(crate_name, "nbd-cli");
    }

    #[test]
    fn test_cli_help() {
        let mut cmd = create_command();
        cmd.arg("--help");

        let stdout = format!("Usage: {} <COMMAND>", get_cli_name());

        cmd.assert()
            .success()
            .stdout(predicates::str::contains(stdout));
    }

    #[test]
    fn test_cli_create_contact() {
        let mut cmd = create_command();
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
            .stdout(predicates::str::contains("Successfully saved contact"));
    }

    #[test]
    fn test_cli_delete_contact() {
        let mut cmd = create_command();

        let create_output = cmd
            .arg("create")
            .arg("--first-name")
            .arg("First")
            .arg("--last-name")
            .arg("Last")
            .arg("--email")
            .arg("test@test.com")
            .arg("--phone-number")
            .arg("123-321-1233");

        let output = create_output.output().unwrap();

        let stdout = String::from_utf8(output.stdout).unwrap().trim().to_string();
        let contact_id = stdout.chars().last().unwrap().to_string();

        cmd = create_command();

        cmd.arg("delete").arg(contact_id);

        cmd.assert()
            .success()
            .stdout(predicates::str::contains("Successfully deleted contact"));
    }

    #[test]
    fn test_cli_invalid_email() {
        let mut cmd = create_command();
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
    fn test_cli_invalid_phone() {
        let mut cmd = create_command();
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
    fn test_cli_invalid_args() {
        let mut cmd = create_command();
        cmd.arg("First").arg("Last").arg("32321123");

        let stderr = format!("Usage: {} <COMMAND>", get_cli_name());

        cmd.assert()
            .failure()
            .stderr(predicates::str::contains(stderr));
    }

    #[test]
    fn test_import() {
        let mut cmd = create_command();
        cmd.arg("import")
            .arg("example.csv");

        cmd.assert()
            .success()
            .stdout(predicates::str::contains("Successfully imported"));
    }
}
