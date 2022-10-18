//! # Dirs

// Ext
use std::path::{Path, PathBuf};

/// Get donmaze configuration directory path.
/// Returns None, if it's not possible to get it
pub fn init_config_dir() -> anyhow::Result<Option<PathBuf>> {
    // Get file
    #[cfg(not(test))]
    lazy_static! {
        static ref CONF_DIR: Option<PathBuf> = dirs::config_dir();
    }
    #[cfg(test)]
    lazy_static! {
        static ref CONF_DIR: Option<PathBuf> = Some(std::env::temp_dir());
    }
    if CONF_DIR.is_some() {
        // Get path of bookmarks
        let mut p: PathBuf = CONF_DIR.as_ref().unwrap().clone();
        // Append donmaze dir
        p.push("donmaze/");
        // If directory doesn't exist, create it
        match p.exists() {
            true => Ok(Some(p)),
            false => match std::fs::create_dir(p.as_path()) {
                Ok(_) => Ok(Some(p)),
                Err(err) => anyhow::bail!(err),
            },
        }
    } else {
        Ok(None)
    }
}

/// Returns the path for the log file
pub fn get_log_path(config_dir: &Path) -> PathBuf {
    let mut log_file: PathBuf = PathBuf::from(config_dir);
    log_file.push("donmaze.log");
    log_file
}

/// Get paths for theme provider
/// Returns: path of theme.toml
pub fn get_saves_path(config_dir: &Path) -> PathBuf {
    // Prepare paths
    let mut saves_path: PathBuf = PathBuf::from(config_dir);
    saves_path.push("saves/");
    saves_path
}

#[cfg(test)]
mod tests {

    use super::*;

    use pretty_assertions::assert_eq;
    use serial_test::serial;
    use std::fs::{File, OpenOptions};
    use std::io::Write;

    #[test]
    #[serial]
    fn should_get_config_dir() {
        // Create and get conf_dir
        let conf_dir: PathBuf = init_config_dir().ok().unwrap().unwrap();
        // Remove dir
        assert!(std::fs::remove_dir_all(conf_dir.as_path()).is_ok());
    }

    #[test]
    #[serial]
    fn should_not_get_config_dir() {
        let mut conf_dir: PathBuf = std::env::temp_dir();
        conf_dir.push("donmaze");
        // Create file
        let mut f: File = OpenOptions::new()
            .create(true)
            .write(true)
            .open(conf_dir.as_path())
            .ok()
            .unwrap();
        // Write
        assert!(writeln!(f, "Hello world!").is_ok());
        // Drop file
        drop(f);
        // Get config dir (will fail)
        assert!(init_config_dir().is_err());
        // Remove file
        assert!(std::fs::remove_file(conf_dir.as_path()).is_ok());
    }

    #[test]
    #[serial]
    fn should_get_log_path() {
        assert_eq!(
            get_log_path(Path::new("/home/omar/.config/donmaze/")),
            PathBuf::from("/home/omar/.config/donmaze/donmaze.log"),
        );
    }

    #[test]
    #[serial]
    fn test_system_environment_get_save_games_path() {
        assert_eq!(
            get_saves_path(Path::new("/home/omar/.config/donmaze/")),
            PathBuf::from("/home/omar/.config/donmaze/saves/"),
        );
    }
}
