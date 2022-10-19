//! # Popup

use super::{GameMsg, Msg};

use tui_realm_stdlib::{Input, Paragraph, Radio};
use tuirealm::command::{Cmd, CmdResult, Direction, Position};
use tuirealm::event::{Key, KeyEvent};
use tuirealm::props::{Alignment, BorderType, Borders, Color, Style, TextSpan};
use tuirealm::{Component, Event, MockComponent, NoUserEvent, State, StateValue};

// -- error popup

#[derive(MockComponent)]
pub struct ErrorPopup {
    component: Paragraph,
}

impl ErrorPopup {
    pub fn new<S: AsRef<str>>(text: S) -> Self {
        Self {
            component: Paragraph::default()
                .alignment(Alignment::Center)
                .borders(
                    Borders::default()
                        .color(Color::Red)
                        .modifiers(BorderType::Double),
                )
                .foreground(Color::Red)
                .text(&[TextSpan::from(text.as_ref())])
                .wrap(true),
        }
    }
}

impl Component<Msg, NoUserEvent> for ErrorPopup {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        match ev {
            Event::Keyboard(KeyEvent {
                code: Key::Esc | Key::Enter,
                ..
            }) => Some(Msg::Game(GameMsg::CloseErrorPopup)),
            _ => None,
        }
    }
}

#[derive(MockComponent)]
pub struct GameOverPopup {
    component: Paragraph,
}

impl Default for GameOverPopup {
    fn default() -> Self {
        Self {
            component: Paragraph::default()
                .alignment(Alignment::Center)
                .borders(
                    Borders::default()
                        .color(Color::Red)
                        .modifiers(BorderType::Double),
                )
                .foreground(Color::Red)
                .text(&[TextSpan::from("You're dead. Game over!")])
                .wrap(true),
        }
    }
}

impl Component<Msg, NoUserEvent> for GameOverPopup {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        match ev {
            Event::Keyboard(KeyEvent {
                code: Key::Esc | Key::Enter,
                ..
            }) => Some(Msg::Game(GameMsg::GameOver)),
            _ => None,
        }
    }
}

// -- quit popup

#[derive(MockComponent)]
pub struct QuitPopup {
    component: Radio,
}

impl QuitPopup {
    pub fn new() -> Self {
        Self {
            component: Radio::default()
                .borders(
                    Borders::default()
                        .color(Color::Magenta)
                        .modifiers(BorderType::Double),
                )
                .foreground(Color::Magenta)
                .title("Quit donmaze?", Alignment::Center)
                .rewind(true)
                .choices(&["Quit and save", "Quit without saving", "No"]),
        }
    }
}

impl Component<Msg, NoUserEvent> for QuitPopup {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        match ev {
            Event::Keyboard(KeyEvent { code: Key::Esc, .. }) => {
                Some(Msg::Game(GameMsg::CloseQuitPopup))
            }
            Event::Keyboard(KeyEvent {
                code: Key::Left, ..
            }) => {
                self.perform(Cmd::Move(Direction::Left));
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent {
                code: Key::Right, ..
            }) => {
                self.perform(Cmd::Move(Direction::Right));
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent {
                code: Key::Enter, ..
            }) => match self.perform(Cmd::Submit) {
                CmdResult::Submit(State::One(StateValue::Usize(0))) => {
                    Some(Msg::Game(GameMsg::Quit(true)))
                }
                CmdResult::Submit(State::One(StateValue::Usize(1))) => {
                    Some(Msg::Game(GameMsg::Quit(false)))
                }
                _ => Some(Msg::Game(GameMsg::CloseQuitPopup)),
            },
            _ => None,
        }
    }
}

#[derive(MockComponent)]
pub struct SaveFileNamePopup {
    component: Input,
}

impl SaveFileNamePopup {
    pub fn new() -> Self {
        Self {
            component: Input::default()
                .borders(
                    Borders::default()
                        .color(Color::LightRed)
                        .modifiers(BorderType::Double),
                )
                .foreground(Color::LightRed)
                .placeholder("save name", Style::default().fg(Color::Rgb(128, 128, 128)))
                .title("Save game as…", Alignment::Center),
        }
    }
}

impl Component<Msg, NoUserEvent> for SaveFileNamePopup {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        match ev {
            Event::Keyboard(KeyEvent {
                code: Key::Left, ..
            }) => {
                self.perform(Cmd::Move(Direction::Left));
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent {
                code: Key::Right, ..
            }) => {
                self.perform(Cmd::Move(Direction::Right));
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent {
                code: Key::Home, ..
            }) => {
                self.perform(Cmd::GoTo(Position::Begin));
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent { code: Key::End, .. }) => {
                self.perform(Cmd::GoTo(Position::End));
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent {
                code: Key::Delete, ..
            }) => {
                self.perform(Cmd::Cancel);
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent {
                code: Key::Backspace,
                ..
            }) => {
                self.perform(Cmd::Delete);
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent {
                code: Key::Char(ch),
                ..
            }) => {
                self.perform(Cmd::Type(ch));
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent {
                code: Key::Enter, ..
            }) => match self.state() {
                State::One(StateValue::String(i)) => Some(Msg::Game(GameMsg::SaveGame(i))),
                _ => Some(Msg::None),
            },
            Event::Keyboard(KeyEvent { code: Key::Esc, .. }) => {
                Some(Msg::Game(GameMsg::CloseSaveFileName))
            }
            _ => None,
        }
    }
}
