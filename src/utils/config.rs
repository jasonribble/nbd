use dirs;
use std::path::Path;
use std::path::PathBuf;

#[must_use]
pub fn get_config_dir() -> PathBuf {
    resolve_config_dir(std::env::var("NBD_CONFIG_DIR").ok(), dirs::config_dir())
}

pub fn resolve_config_dir(
    env_override: Option<String>,
    default_base_path: Option<PathBuf>,
) -> PathBuf {
    env_override.map_or_else(
        || {
            default_base_path
                .unwrap_or_else(|| std::env::current_dir().unwrap_or_default())
                .join("nbd")
        },
        PathBuf::from,
    )
}


#[must_use]
pub fn build_database_path(config_dir: &Path) -> PathBuf {
    config_dir.join("contacts.db")
}


#[must_use]
pub fn build_database_url(path: &Path) -> String {
    format!("sqlite://{}", path.to_string_lossy())
}

#[must_use]
pub fn is_already_initialized(db_path: &Path) -> bool {
    db_path.exists()
}

/// # Errors
///
///  Will return filesystem errors
pub fn ensure_config_dir(target: &PathBuf) -> anyhow::Result<(), std::io::Error> {
    std::fs::create_dir_all(target)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_use_env_var_when_set() {
        let result = resolve_config_dir(
            Some("/custom/path".to_string()),
            Some(PathBuf::from("/home/user/.config")),
        );
        assert_eq!(result, PathBuf::from("/custom/path"));
    }

    #[test]
    fn should_fallback_to_default_when_env_is_not_set() {
        let result = resolve_config_dir(None, Some(PathBuf::from("/home/user/.config/")));

        assert_eq!(result, PathBuf::from("/home/user/.config/nbd"));
    }

    #[test]
    fn should_build_database_appends_contact_db() {
        let config_dir = PathBuf::from("/home/user/.config/nbd");
        let result = build_database_path(&config_dir);

        assert_eq!(result, PathBuf::from("/home/user/.config/nbd/contacts.db"));
    }

    #[test]
    fn ensure_config_dir_creates_dir_when_missing() {
        let temp = tempfile::TempDir::new().unwrap();
        let target = temp.path().join("nbd");
        assert!(!target.exists());

        ensure_config_dir(&target).unwrap();

        assert!(target.exists());
        assert!(target.is_dir());
    }

    #[test]
    fn build_database_url_formats_sqlite_url_from_path() {
        let path = PathBuf::from("/tmp/nbd/contacts.db");
        let url = build_database_url(&path);
        assert_eq!(url, "sqlite:///tmp/nbd/contacts.db");
    }
}

