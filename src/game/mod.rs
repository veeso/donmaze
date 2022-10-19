use std::path::{Path, PathBuf};

///! # Game
///
/// Main game core engine and logics
use crate::audio::{AudioEngine, Sound, Theme};
use crate::gfx::Render;
use crate::ui::{GameId, GameMsg, Id, LoadGameMsg, MenuId, MenuMsg, Msg, Ui};
use crate::utils::saved_games::SavedGameFiles;

mod entity;
mod error;
mod inventory;
mod maze;
mod options;
mod session;

pub use error::Error as GameError;
pub use options::Options;
pub use session::Session;

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
                todo!("mount error popup");
                return Ok(());
            }
            Err(e) => {
                todo!("mount error popup");
                return Ok(());
            }
        };
        self.start_maze(session)
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
        todo!()
    }

    fn update_load_game(&mut self, msg: LoadGameMsg) -> GameResult<()> {
        match msg {
            LoadGameMsg::GoToMenu => {
                self.ui.load_menu()?;
            }
            LoadGameMsg::GameChanged(p) => match SavedGameFiles::load_game(&p) {
                Err(e) => {
                    todo!("show popup");
                }
                Ok(session) => {
                    self.ui.set_save_metadata(
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
                let seed = self.ui.get_seed()?;
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
