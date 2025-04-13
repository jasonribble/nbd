#[cfg(test)]
mod tests {
    use assert_cmd::Command;
    use nbd::db::ContactRepo;
    use nbd::{db::Connection, models::Contact};
    use sqlx::SqlitePool;

    fn create_command() -> Command {
        Command::cargo_bin(get_cli_name()).unwrap()
    }

    fn get_cli_name() -> String {
        let package_name = env!("CARGO_PKG_NAME");
        let cli_name = format!("{}-cli", package_name);
        cli_name.to_string()
    }

    async fn clean_database() -> Result<(), sqlx::Error> {
        let pool = SqlitePool::connect("sqlite:contacts.db").await?;

        // Execute each query directly on the pool instead of using a transaction
        sqlx::query!("PRAGMA foreign_keys = OFF")
            .execute(&pool)
            .await?;

        sqlx::query!("DELETE FROM contacts_metadata")
            .execute(&pool)
            .await?;

        sqlx::query!("DELETE FROM contacts").execute(&pool).await?;

        sqlx::query!("DELETE FROM SQLITE_SEQUENCE WHERE name = 'contacts'")
            .execute(&pool)
            .await?;

        sqlx::query!("DELETE FROM SQLITE_SEQUENCE WHERE name = 'contacts_metadata'")
            .execute(&pool)
            .await?;

        sqlx::query!("PRAGMA foreign_keys = ON")
            .execute(&pool)
            .await?;

        Ok(())
    }

    #[test]
    fn should_have_name_nbd_cli() {
        let crate_name = get_cli_name();
        assert_eq!(crate_name, "nbd-cli");
    }

    #[test]
    fn should_display_cli_help() {
        let mut cmd = create_command();
        cmd.arg("--help");

        let expected_output = vec![
            "Usage: nbd-cli <COMMAND>",
            "",
            "Commands:",
            "  create  Create a contact",
            "  edit    Edit a contact by ID",
            "  show    Get all contacts",
            "  get     Get a contact",
            "  delete  Delete a contact",
            "  import  Import contact via CSV",
            "  help    Print this message or the help of the given subcommand(s)",
            "",
            "Options:",
            "  -h, --help     Print help",
            "  -V, --version  Print version",
        ];

        cmd.assert()
            .success()
            .stdout(predicates::str::contains(expected_output.join("\n")));
    }

    #[tokio::test]
    async fn should_be_able_to_create_full_contact() {
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

        clean_database().await.unwrap();
    }

    #[tokio::test]
    async fn should_delete_a_contact_when_one_is_present() -> anyhow::Result<()> {
        let pool = SqlitePool::connect("sqlite:contacts.db").await?;
        let data_repo = Connection::new(pool);

        let birthday = chrono::NaiveDate::from_ymd_opt(1832, 1, 27).unwrap();

        let example_contact = Contact::new(
            "Lewis",
            "Carroll",
            "lewis@wonderland.com",
            "777-777-7777",
            birthday,
        )
        .unwrap();

        data_repo.save_contact(example_contact).await.unwrap();

        let mut cmd = create_command();
        let contact_id = "1";
        cmd.arg("delete").arg(contact_id);

        cmd.assert()
            .success()
            .stdout(predicates::str::contains("Successfully deleted contact"));

        clean_database().await?;

        Ok(())
    }

    #[test]
    fn should_error_when_providing_invalid_email() {
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
    fn should_error_when_providing_invalid_phone_number() {
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
    fn should_error_when_invalid_args() {
        let mut cmd = create_command();
        cmd.arg("First").arg("Last").arg("32321123");

        let stderr = format!("Usage: {} <COMMAND>", get_cli_name());

        cmd.assert()
            .failure()
            .stderr(predicates::str::contains(stderr));
    }

    #[tokio::test]
    async fn should_import_one_contact_when_importing_alice_csv() -> anyhow::Result<()> {
        let mut cmd = create_command();
        cmd.arg("import").arg("tests/fixtures/alice.csv");

        cmd.assert()
            .success()
            .stdout(predicates::str::contains("Successfully imported 1 contact"));

        let pool = SqlitePool::connect("sqlite:contacts.db").await?;
        let data_repo = Connection::new(pool);

        let contacts = data_repo.get_all_contacts().await?;

        assert_eq!(contacts.len(), 1);

        Ok(())
    }

    #[test]
    fn should_import_example_csv_with_three_rows() {
        let mut cmd = create_command();
        cmd.arg("import").arg("tests/fixtures/example.csv");

        cmd.assert()
            .success()
            .stdout(predicates::str::contains("Successfully imported"));
    }

    #[test]
    fn should_fail_when_given_incorrect_file() {
        let mut cmd = create_command();
        cmd.arg("import").arg("tests/fixtures/example.txt");

        cmd.assert()
            .success()
            .stdout(predicates::str::contains("File must have .csv extension"));
    }

    #[test]
    fn should_fail_when_given_blank_csv() {
        let mut cmd = create_command();
        cmd.arg("import").arg("tests/fixtures/blank.csv");

        cmd.assert()
            .success()
            .stdout(predicates::str::contains("CSV file is empty"));
    }

    #[tokio::test]
    #[ignore = "TODO: fix acceptance tests"]
    async fn should_say_no_contacts_when_contacts_are_empty() -> anyhow::Result<()> {
        clean_database().await?;

        let mut cmd = create_command();
        cmd.arg("show");

        cmd.assert()
            .success()
            .stdout(predicates::str::contains("No contacts yet!"));
        Ok(())
    }

    #[tokio::test]
    #[ignore = "TODO refactor acceptance tests"]
    async fn should_show_one_contact_when_one_contact_available() -> anyhow::Result<()> {
        clean_database().await?;

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

        let mut cmd = create_command();

        cmd.arg("show");

        let expected = "1 | First Last   | First      | Last      |    123456789 | test@test.com |";

        cmd.assert()
            .success()
            .stdout(predicates::str::contains(expected));
        Ok(())
    }

    #[tokio::test]
    #[ignore = "TODO refactor acceptance tests"]
    async fn should_show_two_contact_when_two_contact_available() -> anyhow::Result<()> {
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

        let mut cmd = create_command();

        cmd.arg("show");

        let expected =
            "+----+------------+-----------+--------------+---------------+--------------+
    | id | first_name | last_name | display_name | email         | phone_number |
    +----+------------+-----------+--------------+---------------+--------------+
    | 1 | First      | Last      | First Last   | test@test.com | 123-321-1233 |
    +----+------------+-----------+--------------+---------------+--------------+
    | 2 | First      | Last      | First Last   | test@test.com | 123-321-1233 |
    +----+------------+-----------+--------------+---------------+--------------+";

        cmd.assert()
            .success()
            .stdout(predicates::str::contains(expected));
        Ok(())
    }

    #[tokio::test]
    async fn should_accept_a_firstname_and_birthday() {
        let mut cmd = create_command();
        cmd.arg("create")
            .arg("--first-name")
            .arg("Molly")
            .arg("--birthday")
            .arg("1970-01-01");

        cmd.assert()
            .success()
            .stdout(predicates::str::contains("Successfully saved contact"));

        clean_database().await.unwrap();
    }
}
