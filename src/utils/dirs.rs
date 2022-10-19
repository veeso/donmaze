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
        if p.exists() {
            Ok(Some(p))
        } else {
            match std::fs::create_dir_all(p.as_path()) {
                Ok(_) => Ok(Some(p)),
                Err(err) => anyhow::bail!(err),
            }
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
/// Returns: path of saves dir
/// If dir doesn't exist, it is created
pub fn get_saves_path(config_dir: &Path) -> anyhow::Result<PathBuf> {
    // Prepare paths
    let mut saves_path: PathBuf = PathBuf::from(config_dir);
    saves_path.push("saves/");
    // Create dir
    if saves_path.exists() {
        Ok(saves_path)
    } else {
        match std::fs::create_dir_all(&saves_path) {
            Ok(_) => Ok(saves_path),
            Err(err) => anyhow::bail!(err),
        }
    }
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
    fn should_get_save_paths() {
        let conf_dir: PathBuf = init_config_dir().ok().unwrap().unwrap();
        let mut expected = conf_dir.clone();
        expected.push("saves/");
        assert_eq!(get_saves_path(&conf_dir).unwrap(), expected);
        assert!(std::fs::remove_dir_all(&conf_dir).is_ok());
    }
}
