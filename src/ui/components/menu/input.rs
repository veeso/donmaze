//! # Input
//!
//! Input components for menu

use super::{MenuMsg, Msg};

use tui_realm_stdlib::Input;
use tuirealm::props::{Alignment, BorderType, Borders, Color, Style};
use tuirealm::{
    command::{Cmd, Direction, Position},
    NoUserEvent,
};
use tuirealm::{
    event::{Key, KeyEvent, KeyModifiers},
    Component, Event, MockComponent,
};

#[derive(MockComponent)]
pub struct Seed {
    component: Input,
}

impl Default for Seed {
    fn default() -> Self {
        Self {
            component: Input::default()
                .foreground(Color::LightMagenta)
                .input_len(32)
                .placeholder("seed", Style::default().fg(Color::DarkGray))
                .title("Seed", Alignment::Center)
                .invalid_style(Style::default().fg(Color::Red))
                .borders(
                    Borders::default()
                        .color(Color::LightMagenta)
                        .modifiers(BorderType::Double),
                ),
        }
    }
}

impl Component<Msg, NoUserEvent> for Seed {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        match ev {
            Event::Keyboard(KeyEvent {
                code: Key::Char(ch),
                modifiers: KeyModifiers::NONE,
            }) => {
                self.perform(Cmd::Type(ch));
                Some(Msg::None)
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
                code: Key::Backspace,
                ..
            }) => {
                self.perform(Cmd::Delete);
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent {
                code: Key::Delete, ..
            }) => {
                self.perform(Cmd::Cancel);
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent { code: Key::End, .. }) => {
                self.perform(Cmd::GoTo(Position::End));
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent {
                code: Key::Home, ..
            }) => {
                self.perform(Cmd::GoTo(Position::Begin));
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent { code: Key::Up, .. }) => {
                Some(Msg::Menu(MenuMsg::ActiveNewGame))
            }
            Event::Keyboard(KeyEvent {
                code: Key::Down, ..
            }) => Some(Msg::Menu(MenuMsg::ActiveLoadGame)),
            _ => None,
        }
    }
}
