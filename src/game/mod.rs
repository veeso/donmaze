use std::path::{Path, PathBuf};

///! # Game
///
/// Main game core engine and logics
use crate::audio::{AudioEngine, Sound, Theme};
use crate::gfx::Render;
use crate::ui::{GameId, GameMsg, Id, LoadGameMsg, MenuId, MenuMsg, Msg, Ui};
use crate::utils::saved_games::SavedGameFiles;

pub mod entity;
mod error;
pub mod inventory;
mod maze;
mod options;
pub mod session;

use entity::Item;
pub use error::Error as GameError;
pub use options::Options;
pub use session::Session;

use self::session::Action;

pub type GameResult<T> = Result<T, GameError>;
/// Health points
pub type Hp = u8;

/// Game runtime
pub struct Runtime {
    audio: Option<AudioEngine>,
    saved_games_dir: PathBuf,
    render: Render,
    running: bool,
    session: Option<Session>,
    ui: Ui,
}

impl Runtime {
    /// Setup game runtime
    pub fn setup(options: Options) -> GameResult<Self> {
        debug!("setting up game runtime");
        let audio = if options.muted {
            None
        } else {
            debug!("configuring audio engine");
            AudioEngine::new(Theme::None).map(|x| Some(x))?
        };
        info!("audio OK");
        debug!("initializing Ui");
        let mut ui = Ui::new()?;
        info!("Ui OK");
        let (width, height) = ui.sizes()?;
        let render = Render::new(width, height);
        info!("Render engine configured to work on {}x{}", width, height);
        // loading menu
        debug!("loading menu");
        ui.load_menu()?;
        info!("menu loaded");
        Ok(Self {
            audio,
            saved_games_dir: options.saved_games_dir,
            render,
            running: true,
            session: None,
            ui,
        })
    }

    /// Run game
    pub fn run(mut self) -> GameResult<()> {
        debug!("initializing terminal...");
        self.ui.init_terminal();
        debug!("playing Menu theme...");
        self.play_theme(Theme::Menu)?;
        while self.running {
            // Run view
            let mut redraw = true;
            let messages = self.ui.tick()?;
            for msg in messages.into_iter() {
                self.update(msg)?;
                redraw = true;
            }
            if redraw {
                self.ui.view()?;
            }
        }
        // Finalize terminal and stop sound
        debug!("finalizing terminal...");
        self.ui.finalize_terminal();
        debug!("stopping theme...");
        self.play_theme(Theme::None)?;

        Ok(())
    }

    /// Play sound
    fn play_sound(&mut self, sound: Sound) {
        if let Some(audio) = self.audio.as_mut() {
            audio.play(sound.track());
        }
    }

    /// Play theme
    fn play_theme(&mut self, theme: Theme) -> GameResult<()> {
        if let Some(audio) = self.audio.as_mut() {
            audio.play_theme(theme)?;
        }
        Ok(())
    }

    /// Start gameplay in the maze
    fn start_maze(&mut self, session: Session) -> GameResult<()> {
        self.session = Some(session);
        self.play_theme(Theme::Maze)?;
        self.ui.load_game()?;

        Ok(())
    }

    /// load saved game
    fn load_game(&mut self, game_file: &Path) -> GameResult<()> {
        debug!("loading game {}", game_file.display());
        let session = match SavedGameFiles::load_game(game_file) {
            Ok(s) if s.is_version_compatible() => s,
            Ok(s) => {
                self.ui.show_load_game_error("incompatible game version")?;
                return Ok(());
            }
            Err(e) => {
                self.ui
                    .show_load_game_error(format!("failed to load game: {}", e))?;
                return Ok(());
            }
        };
        self.start_maze(session)
    }

    /// Save game as name
    fn save_game(&mut self, name: &str) -> GameResult<()> {
        if let Some(session) = self.session.as_ref() {
            debug!("saving game as {}", name);
            SavedGameFiles::save_game(name, &self.saved_games_dir, &session)?;
        }
        Ok(())
    }

    /// Play action in game
    fn play_action(&mut self, action: Action) -> GameResult<()> {
        debug!("playing action {:?}", action);
        let effect = self.session.as_mut().unwrap().play_turn(action);
        // play sounds
        for sound in effect.sounds {
            self.play_sound(sound);
        }
        // show messages
        self.ui
            .update_messages(&effect.messages, self.session.as_ref().unwrap())?;
        todo!("reload possible actions");
        todo!("update canvas");
        todo!("update health");
        if self.session.as_ref().unwrap().game_over() {
            info!("player is dead; show game over");
            self.ui.show_game_gameover_popup()?;
        }
        if self.session.as_ref().unwrap().has_won() {
            info!("player has won; show victory");
            self.ui.load_victory()?;
        }
        Ok(())
    }

    fn update(&mut self, msg: Msg) -> GameResult<()> {
        match msg {
            Msg::None => Ok(()),
            Msg::Game(msg) => self.update_game(msg),
            Msg::LoadGame(msg) => self.update_load_game(msg),
            Msg::Menu(msg) => self.update_menu(msg),
        }
    }

    fn update_game(&mut self, msg: GameMsg) -> GameResult<()> {
        match msg {
            GameMsg::ActionSelected(action) => {
                self.play_action(action)?;
            }
            GameMsg::CloseErrorPopup => {
                self.ui.close_game_error_popup()?;
            }
            GameMsg::CloseInventory => {
                self.ui.close_game_inventory()?;
            }
            GameMsg::CloseQuitPopup => {
                self.ui.close_game_quit_popup()?;
            }
            GameMsg::CloseSaveFileName => {
                self.ui.close_game_save_file_name()?;
            }
            GameMsg::GameOver => {
                info!("game over; destroy session and show game over");
                self.session = None;
                self.ui.load_game_over()?;
            }
            GameMsg::Quit(save) => {
                if save {
                    if let Err(err) = self.save_game("autosave") {
                        error!("failed to save game: {}", err);
                        self.ui
                            .show_game_error_popup(format!("failed to save game: {}", err))?;
                        return Ok(());
                    }
                }
                self.session = None;
                self.ui.load_menu()?;
            }
            GameMsg::SaveGame(name) => {
                self.ui.close_game_save_file_name()?;
                if let Err(err) = self.save_game(&name) {
                    error!("failed to save game: {}", err);
                    self.ui
                        .show_game_error_popup(format!("failed to save game: {}", err))?;
                }
            }
            GameMsg::ShowInventory => {
                if let Some(session) = self.session.as_ref() {
                    self.ui.show_game_inventory(session.player_inventory())?;
                }
            }
            GameMsg::ShowQuitPopup => {
                self.ui.show_game_quit_popup()?;
            }
            GameMsg::ShowSaveFileName => {
                self.ui.show_game_save_file_name()?;
            }
        }
        Ok(())
    }

    fn update_load_game(&mut self, msg: LoadGameMsg) -> GameResult<()> {
        match msg {
            LoadGameMsg::CloseErrorPopup => {
                self.ui.close_load_game_error()?;
            }
            LoadGameMsg::GoToMenu => {
                self.ui.load_menu()?;
            }
            LoadGameMsg::GameChanged(p) => match SavedGameFiles::load_game(&p) {
                Err(e) => {
                    self.ui
                        .show_load_game_error(format!("failed to load game: {}", e))?;
                }
                Ok(session) => {
                    self.ui.set_load_game_save_metadata(
                        session.last_turn,
                        session.maze_seed().to_string(),
                        session.turn,
                    )?;
                }
            },
            LoadGameMsg::LoadGame(game_file) => {
                self.load_game(&game_file)?;
            }
        }
        Ok(())
    }

    fn update_menu(&mut self, msg: MenuMsg) -> GameResult<()> {
        match msg {
            MenuMsg::ActiveExit => {
                self.ui.active(Id::Menu(MenuId::Exit));
            }
            MenuMsg::ActiveLoadGame => {
                self.ui.active(Id::Menu(MenuId::LoadGame));
            }
            MenuMsg::ActiveNewGame => {
                self.ui.active(Id::Menu(MenuId::NewGame));
            }
            MenuMsg::ActiveSeed => {
                self.ui.active(Id::Menu(MenuId::Seed));
            }
            MenuMsg::LoadGame => {
                let saved_games = SavedGameFiles::saved_games(&self.saved_games_dir)?;
                let game_0 = match saved_games.get(0) {
                    None => None,
                    Some(p) => SavedGameFiles::load_game(p).ok().map(|session| {
                        (
                            session.last_turn,
                            session.maze_seed().to_string(),
                            session.turn,
                        )
                    }),
                };
                self.ui.load_game_loader(&saved_games, game_0)?;
            }
            MenuMsg::NewGame => {
                // create a new session
                let seed = self.ui.get_menu_seed()?;
                debug!("initializing new session with seed {:?}", seed);
                self.start_maze(Session::new(seed))?;
            }
            MenuMsg::Quit => {
                self.running = false;
            }
        }
        Ok(())
    }
}
