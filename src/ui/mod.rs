//! # Ui
//!
//! Ui related things

use std::path::PathBuf;
use std::time::Duration;

use tuirealm::props::Shape;
use tuirealm::ratatui::layout::{Constraint, Direction, Layout};
use tuirealm::ratatui::widgets::Clear;
use tuirealm::terminal::{CrosstermTerminalAdapter, TerminalBridge};
use tuirealm::{Application, EventListenerCfg, NoUserEvent, State, StateValue};

use crate::game::session::Message;
use crate::game::{Hp, Session};
use crate::utils::ui::draw_area_in;

mod components;
mod error;

pub use components::game::{GameId, GameMsg};
pub use components::game_over::{GameOverId, GameOverMsg};
pub use components::load_game::{LoadGameId, LoadGameMsg};
pub use components::menu::{MenuId, MenuMsg};
pub use components::victory::{VictoryId, VictoryMsg};
use components::{game, game_over, load_game, menu, victory};
pub use error::UiError;

/// UI module result
pub type UiResult<T> = Result<T, UiError>;

/// Application ID
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum Id {
    Game(GameId),
    GameOver(GameOverId),
    LoadGame(LoadGameId),
    Menu(MenuId),
    Victory(VictoryId),
}

/// Application MSG
#[derive(PartialEq, Eq)]
pub enum Msg {
    Game(GameMsg),
    GameOver(GameOverMsg),
    LoadGame(LoadGameMsg),
    Menu(MenuMsg),
    Victory(VictoryMsg),
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
    terminal: TerminalBridge<CrosstermTerminalAdapter>,
    view: View,
}

impl Ui {
    /// Instantiate a new Ui
    pub fn new() -> UiResult<Self> {
        let terminal = TerminalBridge::init_crossterm()?;

        let application = Application::init(
            EventListenerCfg::default().crossterm_input_listener(Duration::from_millis(10), 100),
        );
        let ui = Self {
            application,
            terminal,
            view: View::None,
        };
        Ok(ui)
    }

    /// Tick application events
    pub fn tick(&mut self) -> UiResult<Vec<Msg>> {
        let msg = self.application.tick(tuirealm::PollStrategy::UpTo(3))?;
        Ok(msg)
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

    // @! view

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
        self.terminal.raw_mut().draw(|f| {
            let body = Layout::default()
                .direction(Direction::Vertical)
                .constraints(
                    [
                        Constraint::Length(1), // Enemy data
                        Constraint::Min(20),   // Canvas
                        Constraint::Length(6), // player's stats
                    ]
                    .as_ref(),
                )
                .split(f.area());
            // enemy chunks
            let enemy_data_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(40), Constraint::Percentage(60)].as_ref())
                .split(body[0]);
            self.application
                .view(&Id::Game(GameId::EnemyName), f, enemy_data_chunks[0]);
            self.application
                .view(&Id::Game(GameId::EnemyHp), f, enemy_data_chunks[1]);
            // canvas
            self.application.view(&Id::Game(GameId::Canvas), f, body[1]);
            // player's states
            let player_states = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(
                    [
                        Constraint::Length(40),
                        Constraint::Min(30),
                        Constraint::Length(40),
                    ]
                    .as_ref(),
                )
                .split(body[2]);
            self.application
                .view(&Id::Game(GameId::AvailableActions), f, player_states[0]);
            self.application
                .view(&Id::Game(GameId::Messages), f, player_states[1]);
            self.application
                .view(&Id::Game(GameId::PlayerHp), f, player_states[2]);
            // popups
            if self.application.mounted(&Id::Game(GameId::ErrorPopup)) {
                let popup = draw_area_in(f.area(), 50, 20);
                f.render_widget(Clear, popup);
                // make popup
                self.application
                    .view(&Id::Game(GameId::ErrorPopup), f, popup);
            } else if self.application.mounted(&Id::Game(GameId::Inventory)) {
                let popup = draw_area_in(f.area(), 70, 80);
                f.render_widget(Clear, popup);
                // make popup
                self.application
                    .view(&Id::Game(GameId::Inventory), f, popup);
            } else if self.application.mounted(&Id::Game(GameId::QuitPopup)) {
                let popup = draw_area_in(f.area(), 50, 10);
                f.render_widget(Clear, popup);
                // make popup
                self.application
                    .view(&Id::Game(GameId::QuitPopup), f, popup);
            } else if self
                .application
                .mounted(&Id::Game(GameId::SaveFileNamePopup))
            {
                let popup = draw_area_in(f.area(), 50, 10);
                f.render_widget(Clear, popup);
                // make popup
                self.application
                    .view(&Id::Game(GameId::SaveFileNamePopup), f, popup);
            } else if self.application.mounted(&Id::Game(GameId::GameOverPopup)) {
                let popup = draw_area_in(f.area(), 50, 10);
                f.render_widget(Clear, popup);
                // make popup
                self.application
                    .view(&Id::Game(GameId::GameOverPopup), f, popup);
            }
        })?;

        Ok(())
    }

    fn view_game_over(&mut self) -> UiResult<()> {
        self.terminal.raw_mut().draw(|f| {
            // Prepare chunks
            let body = Layout::default()
                .direction(Direction::Vertical)
                .horizontal_margin(30)
                .constraints(
                    [
                        Constraint::Length(7),  // Title
                        Constraint::Length(10), // stats
                        Constraint::Length(3),  // menu
                        Constraint::Length(1),  // footer
                    ]
                    .as_ref(),
                )
                .split(f.area());
            self.application
                .view(&Id::GameOver(GameOverId::Title), f, body[0]);
            self.application
                .view(&Id::GameOver(GameOverId::Stats), f, body[1]);
            self.application
                .view(&Id::GameOver(GameOverId::GoToMenu), f, body[2]);
        })?;
        Ok(())
    }

    fn view_load_game(&mut self) -> UiResult<()> {
        self.terminal.raw_mut().draw(|f| {
            // Prepare chunks
            let body = Layout::default()
                .direction(Direction::Horizontal)
                .horizontal_margin(30)
                .constraints(
                    [
                        Constraint::Percentage(60), // List
                        Constraint::Percentage(40), // metadata
                    ]
                    .as_ref(),
                )
                .split(f.area());
            let metadata_chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(10), // metadata
                ])
                .split(body[1]);
            self.application
                .view(&Id::LoadGame(LoadGameId::Games), f, body[0]);
            self.application
                .view(&Id::LoadGame(LoadGameId::Metadata), f, metadata_chunks[0]);
            // popups
            if self
                .application
                .mounted(&Id::LoadGame(LoadGameId::ErrorPopup))
            {
                let popup = draw_area_in(f.area(), 50, 20);
                f.render_widget(Clear, popup);
                // make popup
                self.application
                    .view(&Id::LoadGame(LoadGameId::ErrorPopup), f, popup);
            }
        })?;
        Ok(())
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
                        Constraint::Length(3), // new game + seed
                        Constraint::Length(3), // load game
                        Constraint::Length(3), // quit
                        Constraint::Length(1), // footer
                    ]
                    .as_ref(),
                )
                .split(f.area());
            let new_game_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
                .split(body[1]);
            self.application.view(&Id::Menu(MenuId::Title), f, body[0]);
            self.application
                .view(&Id::Menu(MenuId::NewGame), f, new_game_chunks[0]);
            self.application
                .view(&Id::Menu(MenuId::Seed), f, new_game_chunks[1]);
            self.application
                .view(&Id::Menu(MenuId::LoadGame), f, body[2]);
            self.application.view(&Id::Menu(MenuId::Exit), f, body[3]);
        })?;
        Ok(())
    }

    fn view_victory(&mut self) -> UiResult<()> {
        self.terminal.raw_mut().draw(|f| {
            // Prepare chunks
            let body = Layout::default()
                .direction(Direction::Vertical)
                .horizontal_margin(30)
                .constraints(
                    [
                        Constraint::Length(7),  // Title
                        Constraint::Length(15), // stats
                        Constraint::Length(3),  // menu
                        Constraint::Length(1),  // footer
                    ]
                    .as_ref(),
                )
                .split(f.area());
            self.application
                .view(&Id::Victory(VictoryId::Title), f, body[0]);
            self.application
                .view(&Id::Victory(VictoryId::Stats), f, body[1]);
            self.application
                .view(&Id::Victory(VictoryId::GoToMenu), f, body[2]);
        })?;
        Ok(())
    }

    // -- @! view loaders

    pub fn load_game(&mut self, session: &Session) -> UiResult<()> {
        self.application.umount_all();
        self.application.mount(
            Id::Game(GameId::AvailableActions),
            Box::new(game::AvailableActions::new(session)),
            vec![],
        )?;
        self.application.mount(
            Id::Game(GameId::Canvas),
            Box::new(game::Canvas::new(&[], 0.0, 0.0)),
            vec![],
        )?;
        self.application.mount(
            Id::Game(GameId::Messages),
            Box::new(game::Messages::new(&[], session)),
            vec![],
        )?;
        self.application.mount(
            Id::Game(GameId::PlayerHp),
            Box::new(game::PlayerHp::new(session.player().health())),
            vec![],
        )?;
        self.application
            .active(&Id::Game(GameId::AvailableActions))?;
        self.view = View::Game;

        Ok(())
    }

    /// load game loader
    pub fn load_game_loader(&mut self, games: &[PathBuf], game0: Option<&Session>) -> UiResult<()> {
        self.application.umount_all();
        self.application.mount(
            Id::LoadGame(LoadGameId::Games),
            Box::new(load_game::Games::new(games)),
            vec![],
        )?;
        if let Some(session) = game0 {
            self.application.mount(
                Id::LoadGame(LoadGameId::Metadata),
                Box::new(load_game::Metadata::new(session)),
                vec![],
            )?;
        }
        self.application.active(&Id::LoadGame(LoadGameId::Games))?;
        self.view = View::LoadGame;
        Ok(())
    }

    pub fn load_game_over(&mut self, session: &Session) -> UiResult<()> {
        self.application.umount_all();
        self.application.umount_all();
        let (width, _) = self.sizes()?;
        let width = width as u16 - 60;
        self.application.mount(
            Id::GameOver(GameOverId::Title),
            Box::new(game_over::Title::new(width)),
            vec![],
        )?;
        self.application.mount(
            Id::GameOver(GameOverId::Stats),
            Box::new(game_over::Stats::new(session)),
            vec![],
        )?;
        self.application.mount(
            Id::GameOver(GameOverId::GoToMenu),
            Box::new(game_over::GoToMenu::default()),
            vec![],
        )?;

        self.application
            .active(&Id::GameOver(GameOverId::GoToMenu))?;
        self.view = View::GameOver;

        Ok(())
    }

    /// Load menu view
    pub fn load_menu(&mut self) -> UiResult<()> {
        self.application.umount_all();
        let (width, _) = self.sizes()?;
        let width = width as u16 - 60;
        self.application.mount(
            Id::Menu(MenuId::Title),
            Box::new(menu::Title::new(width)),
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

    pub fn load_victory(&mut self, session: &Session) -> UiResult<()> {
        self.application.umount_all();
        let (width, _) = self.sizes()?;
        let width = width as u16 - 60;
        self.application.mount(
            Id::Victory(VictoryId::Title),
            Box::new(victory::Title::new(width)),
            vec![],
        )?;
        self.application.mount(
            Id::Victory(VictoryId::Stats),
            Box::new(victory::Stats::new(session)),
            vec![],
        )?;
        self.application.mount(
            Id::Victory(VictoryId::GoToMenu),
            Box::new(victory::GoToMenu::default()),
            vec![],
        )?;

        self.application.active(&Id::Victory(VictoryId::GoToMenu))?;
        self.view = View::Victory;

        Ok(())
    }

    // @! component update

    /// Get seed from view if mounted
    pub fn get_menu_seed(&self) -> UiResult<Option<String>> {
        let value = match self.application.state(&Id::Menu(MenuId::Seed))? {
            State::One(StateValue::String(value)) => value,
            _ => String::new(),
        };

        Ok(match value {
            s if s.is_empty() => None,
            s => Some(s),
        })
    }

    /// Close menu error
    pub fn close_load_game_error(&mut self) -> UiResult<()> {
        self.application
            .umount(&Id::LoadGame(LoadGameId::ErrorPopup))?;
        Ok(())
    }

    /// Show menu error
    pub fn show_load_game_error<S: AsRef<str>>(&mut self, text: S) -> UiResult<()> {
        self.application.remount(
            Id::LoadGame(LoadGameId::ErrorPopup),
            Box::new(load_game::ErrorPopup::new(text)),
            vec![],
        )?;
        self.application
            .active(&Id::LoadGame(LoadGameId::ErrorPopup))?;
        Ok(())
    }

    /// Set save file metadata
    pub fn set_load_game_save_metadata(&mut self, session: &Session) -> UiResult<()> {
        self.application.remount(
            Id::LoadGame(LoadGameId::Metadata),
            Box::new(load_game::Metadata::new(session)),
            vec![],
        )?;
        Ok(())
    }

    /// Show game error
    pub fn show_game_error_popup<S: AsRef<str>>(&mut self, text: S) -> UiResult<()> {
        self.application.remount(
            Id::Game(GameId::ErrorPopup),
            Box::new(game::ErrorPopup::new(text)),
            vec![],
        )?;
        self.application.active(&Id::Game(GameId::ErrorPopup))?;
        Ok(())
    }

    /// Close game error
    pub fn close_game_error_popup(&mut self) -> UiResult<()> {
        self.application.umount(&Id::Game(GameId::ErrorPopup))?;
        Ok(())
    }

    /// Show game over
    pub fn show_game_gameover_popup(&mut self) -> UiResult<()> {
        self.application.remount(
            Id::Game(GameId::GameOverPopup),
            Box::new(game::GameOverPopup::default()),
            vec![],
        )?;
        self.application.active(&Id::Game(GameId::GameOverPopup))?;
        Ok(())
    }

    pub fn show_game_inventory(&mut self, session: &Session) -> UiResult<()> {
        self.application.remount(
            Id::Game(GameId::Inventory),
            Box::new(game::Inventory::new(session)),
            vec![],
        )?;
        self.application.active(&Id::Game(GameId::Inventory))?;
        Ok(())
    }

    /// Close game inventory
    pub fn close_game_inventory(&mut self) -> UiResult<()> {
        self.application.umount(&Id::Game(GameId::Inventory))?;
        Ok(())
    }

    pub fn show_game_quit_popup(&mut self) -> UiResult<()> {
        self.application.remount(
            Id::Game(GameId::QuitPopup),
            Box::new(game::QuitPopup::new()),
            vec![],
        )?;
        self.application.active(&Id::Game(GameId::QuitPopup))?;
        Ok(())
    }

    /// Close game quit
    pub fn close_game_quit_popup(&mut self) -> UiResult<()> {
        self.application.umount(&Id::Game(GameId::QuitPopup))?;
        Ok(())
    }

    pub fn show_game_save_file_name(&mut self) -> UiResult<()> {
        self.application.remount(
            Id::Game(GameId::SaveFileNamePopup),
            Box::new(game::SaveFileNamePopup::new()),
            vec![],
        )?;
        self.application
            .active(&Id::Game(GameId::SaveFileNamePopup))?;
        Ok(())
    }

    /// Close game save game
    pub fn close_game_save_file_name(&mut self) -> UiResult<()> {
        self.application
            .umount(&Id::Game(GameId::SaveFileNamePopup))?;
        Ok(())
    }

    /// Update messages in view
    pub fn update_game_messages(
        &mut self,
        messages: &[Message],
        session: &Session,
    ) -> UiResult<()> {
        self.application.remount(
            Id::Game(GameId::Messages),
            Box::new(game::Messages::new(messages, session)),
            vec![],
        )?;
        Ok(())
    }

    /// Update actions
    pub fn update_game_actions(&mut self, session: &Session) -> UiResult<()> {
        self.application.remount(
            Id::Game(GameId::AvailableActions),
            Box::new(game::AvailableActions::new(session)),
            vec![],
        )?;
        self.application
            .active(&Id::Game(GameId::AvailableActions))?;
        Ok(())
    }

    /// Update actions
    pub fn update_game_canvas(&mut self, shapes: &[Shape]) -> UiResult<()> {
        let (width, height) = self.sizes()?;
        let height = height - 6.0 - 2.0 - 2.0;
        self.application.remount(
            Id::Game(GameId::Canvas),
            Box::new(game::Canvas::new(shapes, width - 2.0, height)),
            vec![],
        )?;
        Ok(())
    }

    /// Update player health component
    pub fn update_game_player_health(&mut self, hp: Hp) -> UiResult<()> {
        self.application.remount(
            Id::Game(GameId::PlayerHp),
            Box::new(game::PlayerHp::new(hp)),
            vec![],
        )?;
        Ok(())
    }

    pub fn update_game_enemy_data(&mut self, hp: Hp, name: &str) -> UiResult<()> {
        self.application.remount(
            Id::Game(GameId::EnemyHp),
            Box::new(game::EnemyHp::new(hp)),
            vec![],
        )?;
        self.application.remount(
            Id::Game(GameId::EnemyName),
            Box::new(game::EnemyName::new(name)),
            vec![],
        )?;
        Ok(())
    }

    pub fn hide_game_enemy_data(&mut self) -> UiResult<()> {
        if self.application.mounted(&Id::Game(GameId::EnemyHp)) {
            self.application.umount(&Id::Game(GameId::EnemyName))?;
            self.application.umount(&Id::Game(GameId::EnemyHp))?;
        }
        Ok(())
    }
}
