//! # Ui
//!
//! Ui related things

use std::time::Duration;
use tuirealm::tui::layout::{Constraint, Direction, Layout};
use tuirealm::tui::widgets::Clear;
use tuirealm::{
    props::Shape, terminal::TerminalBridge, Application, Attribute, EventListenerCfg, NoUserEvent,
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
        let ui = Self {
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

    /// Get seed from view if mounted
    pub fn get_seed(&self) -> UiResult<Option<String>> {
        let value = self
            .application
            .query(&Id::Menu(MenuId::Seed), Attribute::Value)?
            .unwrap();
        Ok(match value.unwrap_string() {
            s if s.is_empty() => None,
            s => Some(s),
        })
    }

    /// Display ui to terminal
    pub fn view(&mut self) -> UiResult<()> {
        match self.view {
            View::Game => self.view_game(),
            View::GameOver => self.view_game_over(),
            View::LoadGame => self.view_load_game(),
            View::Menu => self.view_menu(),
            View::Victory => self.view_victory(),
            View::None => Ok(()),
        }
    }

    fn view_game(&mut self) -> UiResult<()> {
        todo!()
    }

    fn view_game_over(&mut self) -> UiResult<()> {
        todo!()
    }

    fn view_load_game(&mut self) -> UiResult<()> {
        todo!();
    }

    fn view_menu(&mut self) -> UiResult<()> {
        self.terminal.raw_mut().draw(|f| {
            // Prepare chunks
            let body = Layout::default()
                .direction(Direction::Vertical)
                .horizontal_margin(30)
                .constraints(
                    [
                        Constraint::Length(7), // Title
                        Constraint::Length(3), // new game
                        Constraint::Length(3), // seed
                        Constraint::Length(3), // load game
                        Constraint::Length(3), // quit
                        Constraint::Length(1), // footer
                    ]
                    .as_ref(),
                )
                .split(f.size());
            self.application.view(&Id::Menu(MenuId::Title), f, body[0]);
            self.application
                .view(&Id::Menu(MenuId::NewGame), f, body[1]);
            self.application.view(&Id::Menu(MenuId::Seed), f, body[2]);
            self.application
                .view(&Id::Menu(MenuId::LoadGame), f, body[3]);
            self.application.view(&Id::Menu(MenuId::Exit), f, body[4]);
        })?;
        Ok(())
    }

    fn view_victory(&mut self) -> UiResult<()> {
        todo!()
    }

    // -- view loaders

    pub fn load_game(&mut self) -> UiResult<()> {
        todo!()
    }

    pub fn load_game_loader(&mut self, games: Vec<String>) -> UiResult<()> {
        todo!()
    }

    /// Load menu view
    pub fn load_menu(&mut self) -> UiResult<()> {
        let (width, height) = self.sizes()?;
        self.application.umount_all();
        self.application.mount(
            Id::Menu(MenuId::Title),
            Box::new(menu::Title::default()),
            vec![],
        )?;
        self.application.mount(
            Id::Menu(MenuId::NewGame),
            Box::new(menu::NewGame::default()),
            vec![],
        )?;
        self.application.mount(
            Id::Menu(MenuId::Seed),
            Box::new(menu::Seed::default()),
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
