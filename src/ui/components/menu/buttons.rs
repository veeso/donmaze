//! # Buttons

use super::super::Button;
use super::{MenuMsg, Msg};

use tuirealm::props::{Alignment, BorderType, Borders, Color, TextSpan};
use tuirealm::{
    event::{Key, KeyEvent},
    Component, Event, MockComponent, NoUserEvent,
};

#[derive(MockComponent)]
pub struct NewGame {
    component: Button,
}

impl Default for NewGame {
    fn default() -> Self {
        Self {
            component: Button::default()
                .alignment(Alignment::Center)
                .foreground(Color::LightRed)
                .borders(
                    Borders::default()
                        .color(Color::LightRed)
                        .modifiers(BorderType::Double),
                )
                .text(&[TextSpan::from("New game")])
                .wrap(true),
        }
    }
}

impl Component<Msg, NoUserEvent> for NewGame {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        match ev {
            Event::Keyboard(KeyEvent {
                code: Key::Enter, ..
            }) => Some(Msg::Menu(MenuMsg::NewGame)),
            Event::Keyboard(KeyEvent { code: Key::Up, .. }) => Some(Msg::Menu(MenuMsg::ActiveExit)),
            Event::Keyboard(KeyEvent {
                code: Key::Right, ..
            }) => Some(Msg::Menu(MenuMsg::ActiveSeed)),
            Event::Keyboard(KeyEvent {
                code: Key::Down, ..
            }) => Some(Msg::Menu(MenuMsg::ActiveLoadGame)),
            _ => None,
        }
    }
}

#[derive(MockComponent)]
pub struct LoadGame {
    component: Button,
}

impl Default for LoadGame {
    fn default() -> Self {
        Self {
            component: Button::default()
                .alignment(Alignment::Center)
                .foreground(Color::Blue)
                .borders(
                    Borders::default()
                        .color(Color::Blue)
                        .modifiers(BorderType::Double),
                )
                .text(&[TextSpan::from("Load game")])
                .wrap(true),
        }
    }
}

impl Component<Msg, NoUserEvent> for LoadGame {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        match ev {
            Event::Keyboard(KeyEvent {
                code: Key::Enter, ..
            }) => Some(Msg::Menu(MenuMsg::LoadGame)),
            Event::Keyboard(KeyEvent { code: Key::Up, .. }) => {
                Some(Msg::Menu(MenuMsg::ActiveNewGame))
            }
            Event::Keyboard(KeyEvent {
                code: Key::Down, ..
            }) => Some(Msg::Menu(MenuMsg::ActiveExit)),
            _ => None,
        }
    }
}

#[derive(MockComponent)]
pub struct Exit {
    component: Button,
}

impl Default for Exit {
    fn default() -> Self {
        Self {
            component: Button::default()
                .alignment(Alignment::Center)
                .foreground(Color::Red)
                .borders(
                    Borders::default()
                        .color(Color::Red)
                        .modifiers(BorderType::Double),
                )
                .text(&[TextSpan::from("Quit")])
                .wrap(true),
        }
    }
}

impl Component<Msg, NoUserEvent> for Exit {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        match ev {
            Event::Keyboard(KeyEvent {
                code: Key::Enter, ..
            }) => Some(Msg::Menu(MenuMsg::Quit)),
            Event::Keyboard(KeyEvent { code: Key::Up, .. }) => {
                Some(Msg::Menu(MenuMsg::ActiveLoadGame))
            }
            Event::Keyboard(KeyEvent {
                code: Key::Down, ..
            }) => Some(Msg::Menu(MenuMsg::ActiveNewGame)),
            _ => None,
        }
    }
}
