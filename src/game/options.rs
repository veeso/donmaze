//! # Game options
//!
//! This module exposes game options

use std::path::PathBuf;

pub struct Options {
    /// Enable music
    pub music: bool,
    pub saved_games_dir: PathBuf,
    /// Enable sounds
    pub sound: bool,
    /// Ui refresh (ms)
    pub ticks: u64,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            music: true,
            sound: true,
            saved_games_dir: PathBuf::default(),
            ticks: 10,
        }
    }
}

impl Options {
    pub fn music(mut self, m: bool) -> Self {
        self.music = m;
        self
    }

    pub fn saved_games_dir(mut self, dir: PathBuf) -> Self {
        self.saved_games_dir = dir;
        self
    }

    pub fn sound(mut self, s: bool) -> Self {
        self.sound = s;
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

    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn should_create_options() {
        let opts = Options::default()
            .music(true)
            .saved_games_dir(PathBuf::from("/tmp"))
            .sound(true)
            .ticks(30);
        assert_eq!(opts.music, true);
        assert_eq!(opts.sound, true);
        assert_eq!(opts.saved_games_dir.as_path(), Path::new("/tmp"));
        assert_eq!(opts.ticks, 30);
    }
}
