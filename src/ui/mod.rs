//! # Ui
//!
//! Ui related things

use std::time::Duration;
use tuirealm::{
    props::Shape, terminal::TerminalBridge, Application, EventListenerCfg, NoUserEvent,
};

mod components;
mod error;

pub use error::UiError;

use components::{game, menu};
pub use components::{
    game::{GameId, GameMsg},
    menu::{MenuId, MenuMsg},
};

/// UI module result
pub type UiResult<T> = Result<T, UiError>;

/// Application ID
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum Id {
    Game(GameId),
    Menu(MenuId),
}

/// Application MSG
#[derive(PartialEq, Eq)]
pub enum Msg {
    Game(GameMsg),
    Menu(MenuMsg),
    None,
}

/// Current UI view
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum View {
    Game,
    GameOver,
    LoadGame,
    Menu,
    Victory,
    None,
}

/// Donmaze UI
pub struct Ui {
    application: Application<Id, Msg, NoUserEvent>,
    terminal: TerminalBridge,
    view: View,
}

impl Ui {
    /// Instantiate a new Ui
    pub fn new() -> UiResult<Self> {
        let application = Application::init(
            EventListenerCfg::default().default_input_listener(Duration::from_millis(10)),
        );
        let mut ui = Self {
            application,
            terminal: TerminalBridge::new()?,
            view: View::None,
        };
        Ok(ui)
    }

    /// Tick application events
    pub fn tick(&mut self) -> UiResult<Vec<Msg>> {
        let msg = self.application.tick(tuirealm::PollStrategy::UpTo(3))?;
        Ok(msg)
    }

    /// Init terminal
    pub fn init_terminal(&mut self) {
        let _ = self.terminal.enable_raw_mode();
        let _ = self.terminal.enter_alternate_screen();
        let _ = self.terminal.clear_screen();
    }

    /// Finalize terminal
    pub fn finalize_terminal(&mut self) {
        let _ = self.terminal.disable_raw_mode();
        let _ = self.terminal.leave_alternate_screen();
        let _ = self.terminal.clear_screen();
    }

    /// Get sizes (wxh)
    pub fn sizes(&self) -> UiResult<(f64, f64)> {
        self.terminal
            .raw()
            .size()
            .map_err(|_| UiError::FailedToGetSize)
            .map(|rect| (rect.width as f64, rect.height as f64))
    }

    /// Active focus
    pub fn active(&mut self, id: Id) {
        let _ = self.application.active(&id);
    }

    /// Load menu view
    pub fn load_menu(&mut self, title: &[Shape]) -> UiResult<()> {
        let (width, height) = self.sizes()?;
        self.application.umount_all();
        self.application.mount(
            Id::Menu(MenuId::Title),
            Box::new(menu::Title::new(title, width, 5.0)),
            vec![],
        )?;
        self.application.mount(
            Id::Menu(MenuId::NewGame),
            Box::new(menu::NewGame::default()),
            vec![],
        )?;
        self.application.mount(
            Id::Menu(MenuId::LoadGame),
            Box::new(menu::LoadGame::default()),
            vec![],
        )?;
        self.application.mount(
            Id::Menu(MenuId::Exit),
            Box::new(menu::Exit::default()),
            vec![],
        )?;
        self.application.active(&Id::Menu(MenuId::NewGame))?;
        self.view = View::Menu;
        Ok(())
    }
}
