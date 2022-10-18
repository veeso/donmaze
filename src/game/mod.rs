use std::path::PathBuf;

///! # Game
///
/// Main game core engine and logics
use crate::audio::{AudioEngine, Sound, Theme};
use crate::gfx::{ascii_art::MAIN_TITLE, Render};
use crate::ui::{GameId, GameMsg, Id, MenuId, MenuMsg, Msg, Ui};

mod entity;
mod error;
mod inventory;
mod maze;
mod options;
mod session;

pub use error::Error as GameError;
pub use options::Options;
use session::Session;

use tuirealm::props::Color;

pub type GameResult<T> = Result<T, GameError>;
/// Health points
pub type Hp = u8;

/// Game runtime
pub struct Runtime {
    audio: Option<AudioEngine>,
    load_game: Option<PathBuf>,
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
            load_game: None,
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
        self.play_theme(Theme::Menu);
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
        self.play_theme(Theme::None);

        Ok(())
    }

    /// Play sound
    fn play_sound(&mut self, sound: Sound) {
        if let Some(audio) = self.audio.as_mut() {
            audio.play(sound.track());
        }
    }

    /// Play theme
    fn play_theme(&mut self, theme: Theme) {
        if let Some(audio) = self.audio.as_mut() {
            audio.play_theme(theme);
        }
    }

    /// load game
    fn load_game(&mut self) -> GameResult<()> {
        // TODO: check version compatible
        todo!()
    }

    fn update(&mut self, msg: Msg) -> GameResult<()> {
        match msg {
            Msg::None => Ok(()),
            Msg::Game(msg) => self.update_game(msg),
            Msg::Menu(msg) => self.update_menu(msg),
        }
    }

    fn update_game(&mut self, msg: GameMsg) -> GameResult<()> {
        todo!()
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
                self.load_game()?;
                let saved_games = todo!();
                self.ui.load_game_loader(saved_games)?;
            }
            MenuMsg::NewGame => {
                // create a new session
                let seed = self.ui.get_seed()?;
                debug!("initializing new session with seed {:?}", seed);
                self.session = Some(Session::new(seed));
                self.ui.load_game()?;
            }
            MenuMsg::Quit => {
                self.running = false;
            }
        }
        Ok(())
    }
}
