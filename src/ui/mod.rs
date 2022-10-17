//! # Ui
//!
//! Ui related things

use std::time::Duration;
use tuirealm::{terminal::TerminalBridge, Application, EventListenerCfg, NoUserEvent};

mod components;
mod error;

pub use error::UiError;

/// UI module result
pub type UiResult<T> = Result<T, UiError>;

/// Application ID
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum Id {
    Game(components::game::GameId),
}

/// Application MSG
#[derive(PartialEq, Eq)]
pub enum Msg {
    Game(components::game::GameMsg),
}

/// Donmaze UI
pub struct Ui {
    application: Application<Id, Msg, NoUserEvent>,
    terminal: TerminalBridge,
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
        };
        todo!("setup activity");
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
}
