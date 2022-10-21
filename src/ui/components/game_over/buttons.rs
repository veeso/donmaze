use super::super::Button;
use super::{GameOverMsg, Msg};

use tuirealm::props::{Alignment, BorderType, Borders, Color, TextSpan};
use tuirealm::{
    event::{Key, KeyEvent},
    Component, Event, MockComponent, NoUserEvent,
};

#[derive(MockComponent)]
pub struct GoToMenu {
    component: Button,
}

impl Default for GoToMenu {
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
                .text(&[TextSpan::from("Go to menu")])
                .wrap(true),
        }
    }
}

impl Component<Msg, NoUserEvent> for GoToMenu {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        match ev {
            Event::Keyboard(KeyEvent {
                code: Key::Enter, ..
            }) => Some(Msg::GameOver(GameOverMsg::GoToMenu)),
            _ => None,
        }
    }
}
