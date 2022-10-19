//! # Games
//!
//! Available game files

use std::path::PathBuf;

use super::{LoadGameMsg, Msg};

use tui_realm_stdlib::List;
use tuirealm::props::{Alignment, BorderType, Borders, Color, TextSpan};
use tuirealm::{
    command::{Cmd, Direction, Position},
    event::{Key, KeyEvent},
    Component, Event, MockComponent, NoUserEvent, State, StateValue,
};

#[derive(MockComponent)]
pub struct Games {
    component: List,
    game_files: Vec<PathBuf>,
}

impl Games {
    pub fn new(saved_games: &[PathBuf]) -> Self {
        let game_files: Vec<PathBuf> = saved_games
            .into_iter()
            .filter(|x| x.file_name().is_some())
            .map(|x| x.to_path_buf())
            .collect();
        let saved_games = game_files
            .iter()
            .map(|x| {
                vec![TextSpan::from(
                    x.file_name().unwrap().to_string_lossy().to_string(),
                )]
            })
            .collect();
        Self {
            component: List::default()
                .borders(
                    Borders::default()
                        .color(Color::LightRed)
                        .modifiers(BorderType::Double),
                )
                .foreground(Color::LightRed)
                .highlighted_color(Color::LightRed)
                .highlighted_str("➤	 ")
                .rewind(true)
                .scroll(true)
                .step(8)
                .title("Saved games", Alignment::Center)
                .rows(saved_games),
            game_files,
        }
    }

    fn current_game_file(&self) -> Option<PathBuf> {
        match self.state() {
            State::One(StateValue::Usize(idx)) => Some(self.game_files[idx].clone()),
            _ => None,
        }
    }
}

impl Component<Msg, NoUserEvent> for Games {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        match ev {
            Event::Keyboard(KeyEvent {
                code: Key::Down, ..
            }) => {
                self.perform(Cmd::Move(Direction::Down));
                self.current_game_file()
                    .map(|x| Msg::LoadGame(LoadGameMsg::GameChanged(x)))
            }
            Event::Keyboard(KeyEvent { code: Key::Up, .. }) => {
                self.perform(Cmd::Move(Direction::Up));
                self.current_game_file()
                    .map(|x| Msg::LoadGame(LoadGameMsg::GameChanged(x)))
            }
            Event::Keyboard(KeyEvent {
                code: Key::PageDown,
                ..
            }) => {
                self.perform(Cmd::Scroll(Direction::Down));
                self.current_game_file()
                    .map(|x| Msg::LoadGame(LoadGameMsg::GameChanged(x)))
            }
            Event::Keyboard(KeyEvent {
                code: Key::PageUp, ..
            }) => {
                self.perform(Cmd::Scroll(Direction::Up));
                self.current_game_file()
                    .map(|x| Msg::LoadGame(LoadGameMsg::GameChanged(x)))
            }
            Event::Keyboard(KeyEvent {
                code: Key::Home, ..
            }) => {
                self.perform(Cmd::GoTo(Position::Begin));
                self.current_game_file()
                    .map(|x| Msg::LoadGame(LoadGameMsg::GameChanged(x)))
            }
            Event::Keyboard(KeyEvent { code: Key::End, .. }) => {
                self.perform(Cmd::GoTo(Position::End));
                self.current_game_file()
                    .map(|x| Msg::LoadGame(LoadGameMsg::GameChanged(x)))
            }
            Event::Keyboard(KeyEvent { code: Key::Esc, .. }) => {
                Some(Msg::LoadGame(LoadGameMsg::GoToMenu))
            }
            Event::Keyboard(KeyEvent {
                code: Key::Enter, ..
            }) => self
                .current_game_file()
                .map(|x| Msg::LoadGame(LoadGameMsg::LoadGame(x))),
            _ => return None,
        }
    }
}
