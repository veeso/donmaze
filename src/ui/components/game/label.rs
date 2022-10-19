//! # labels

use super::Msg;

use tui_realm_stdlib::Label;
use tuirealm::{
    props::{Alignment, Color},
    Component, Event, MockComponent, NoUserEvent,
};

#[derive(MockComponent)]
pub struct EnemyName {
    component: Label,
}

impl EnemyName {
    pub fn new(name: &str) -> Self {
        Self {
            component: Label::default()
                .alignment(Alignment::Right)
                .foreground(Color::Red)
                .text(name),
        }
    }
}

impl Component<Msg, NoUserEvent> for EnemyName {
    fn on(&mut self, _ev: Event<NoUserEvent>) -> Option<Msg> {
        None
    }
}
