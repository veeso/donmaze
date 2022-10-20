//! # Utils

pub mod dirs;
pub mod graphq;
pub mod random;
pub mod saved_games;
pub mod ui;

use log::LevelFilter;
use simplelog::{ConfigBuilder, WriteLogger};
use std::{fs::OpenOptions, path::Path};

/// Setup logger
pub fn setup_logger(log_level: LevelFilter, path: &Path) -> anyhow::Result<()> {
    let file = OpenOptions::new()
        .create(true)
        .append(false)
        .truncate(true)
        .write(true)
        .open(path)?;

    let config = ConfigBuilder::new().set_time_format_rfc3339().build();
    WriteLogger::init(log_level, config, file)?;
    Ok(())
}

#[cfg(test)]
mod test {

    use tempfile::NamedTempFile;

    use super::*;

    #[test]
    fn should_setup_logger() {
        let tempfile = NamedTempFile::new().unwrap();
        assert!(setup_logger(LevelFilter::Debug, tempfile.path()).is_ok());
    }
}
