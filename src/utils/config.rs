use std::path::PathBuf;
use dirs;

#[must_use]
pub fn get_config_dir() -> PathBuf { 
    dirs::config_dir().unwrap_or_else(|| std::env::current_dir().unwrap_or_default()).join("nbd")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_get_proper_config_dir() {

        let linux_default_path = PathBuf::from("/home/user/.config/nbd/");

        assert_eq!(linux_default_path, get_config_dir());
    }

}
