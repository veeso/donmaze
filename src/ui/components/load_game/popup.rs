//! # Popup
//!
//! Menu popup

use super::{LoadGameMsg, Msg};

use tui_realm_stdlib::Paragraph;
use tuirealm::event::{Key, KeyEvent};
use tuirealm::props::{Alignment, BorderType, Borders, Color, TextSpan};
use tuirealm::{Component, Event, MockComponent, NoUserEvent};

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
            }) => Some(Msg::LoadGame(LoadGameMsg::CloseErrorPopup)),
            _ => None,
        }
    }
}
