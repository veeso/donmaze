//! # Game files utilities
//!
//! Utilities to save and load game files

use serde_json::Error as JsonError;
use std::fs::OpenOptions;
use std::path::{Path, PathBuf};
use thiserror::Error;

use crate::game::Session;

pub type SavedGameResult<T> = Result<T, SavedGameError>;

/// Game file error
#[derive(Debug, Error)]
pub enum SavedGameError {
    #[error("io error: {0}")]
    Io(std::io::Error),
    #[error("serialization error: {0}")]
    Json(JsonError),
}

impl From<std::io::Error> for SavedGameError {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<JsonError> for SavedGameError {
    fn from(e: JsonError) -> Self {
        Self::Json(e)
    }
}

pub struct SavedGameFiles;

impl SavedGameFiles {
    /// Save `game` at `games_dir/name`
    pub fn save_game(name: &str, games_dir: &Path, game: &Session) -> SavedGameResult<()> {
        debug!("saving game {}", name);
        let mut path = games_dir.to_path_buf();
        path.push(name);
        debug!("opening save file {}", path.display());
        let file = OpenOptions::new()
            .create(true)
            .truncate(true)
            .write(true)
            .open(&path)?;
        debug!("serializing JSON to file");
        serde_json::to_writer(&file, game)?;
        info!("game saved");
        Ok(())
    }

    /// Load game at path
    pub fn load_game(path: &Path) -> SavedGameResult<Session> {
        debug!("loading game at {}", path.display());
        let file = OpenOptions::new().read(true).open(path)?;
        debug!("game file opened");
        let session = serde_json::from_reader(file)?;
        info!("save loaded");
        Ok(session)
    }

    /// Returns the list of available saved games
    pub fn saved_games(games_dir: &Path) -> SavedGameResult<Vec<PathBuf>> {
        debug!("scanning content of {}", games_dir.display());
        let files = std::fs::read_dir(games_dir)?
            .flatten()
            .into_iter()
            .filter(|x| x.path().is_file())
            .map(|x| x.path())
            .collect();
        Ok(files)
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use crate::utils::dirs;

    use pretty_assertions::assert_eq;

    #[test]
    fn should_save_game() {
        let session = Session::new(None);
        let cfg_dir = dirs::init_config_dir().unwrap().unwrap();
        let games_dir = dirs::get_saves_path(&cfg_dir).unwrap();
        assert!(SavedGameFiles::save_game("mygame", &games_dir, &session).is_ok());
    }

    #[test]
    fn should_load_game() {
        let session = Session::new(None);
        let cfg_dir = dirs::init_config_dir().unwrap().unwrap();
        let games_dir = dirs::get_saves_path(&cfg_dir).unwrap();
        SavedGameFiles::save_game("mygame", &games_dir, &session).expect("failed to save game");
        let mut path = cfg_dir.clone();
        path.push("mygame");
        assert_eq!(
            SavedGameFiles::load_game(&path).unwrap().maze_seed(),
            session.maze_seed()
        );
    }

    #[test]
    fn should_get_games_list() {
        let session = Session::new(None);
        let cfg_dir = dirs::init_config_dir().unwrap().unwrap();
        let games_dir = dirs::get_saves_path(&cfg_dir).unwrap();
        SavedGameFiles::save_game("mygame", &games_dir, &session).expect("failed to save game");
        SavedGameFiles::save_game("mygame2", &games_dir, &session).expect("failed to save game");
        assert_eq!(SavedGameFiles::saved_games(&games_dir).unwrap().len(), 2);
    }
}
