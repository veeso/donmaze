//! # Game options
//!
//! This module exposes game options

use std::path::PathBuf;

pub struct Options {
    saved_games_dir: PathBuf,
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
