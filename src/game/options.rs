//! # Game options
//!
//! This module exposes game options

use std::path::PathBuf;

pub struct Options {
    pub saved_games_dir: PathBuf,
    /// Is audio muted
    pub muted: bool,
    /// Ui refresh (ms)
    pub ticks: u64,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            muted: false,
            saved_games_dir: PathBuf::default(),
            ticks: 10,
        }
    }
}

impl Options {
    pub fn muted(mut self, m: bool) -> Self {
        self.muted = m;
        self
    }

    pub fn saved_games_dir(mut self, dir: PathBuf) -> Self {
        self.saved_games_dir = dir;
        self
    }

    pub fn ticks(mut self, ticks: u64) -> Self {
        self.ticks = ticks;
        self
    }
}

#[cfg(test)]
mod test {

    use std::path::Path;

    use super::*;

    use pretty_assertions::assert_eq;

    #[test]
    fn should_create_options() {
        let opts = Options::default()
            .muted(true)
            .saved_games_dir(PathBuf::from("/tmp"))
            .ticks(30);
        assert_eq!(opts.muted, true);
        assert_eq!(opts.saved_games_dir.as_path(), Path::new("/tmp"));
        assert_eq!(opts.ticks, 30);
    }
}
