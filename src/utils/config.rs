#[must_use]
pub const fn create_config() -> &'static str {
    "~/.config/nbd/"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_create_database_in_config_folder() {
        assert_eq!("~/.config/nbd/", create_config());
    }
}
