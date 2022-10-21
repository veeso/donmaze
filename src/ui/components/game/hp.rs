//! # hp

use crate::game::Hp;

use super::Msg;

use tui_realm_stdlib::Paragraph;
use tuirealm::props::{Alignment, BorderSides, Borders, Color, TextSpan};
use tuirealm::NoUserEvent;
use tuirealm::{Component, Event, MockComponent};

#[derive(MockComponent)]
pub struct EnemyHp {
    component: HealthPoints,
}

impl EnemyHp {
    pub fn new(hp: Hp) -> Self {
        Self {
            component: HealthPoints::new(hp, BorderSides::LEFT, Alignment::Right),
        }
    }
}

impl Component<Msg, NoUserEvent> for EnemyHp {
    fn on(&mut self, _ev: Event<NoUserEvent>) -> Option<Msg> {
        None
    }
}

#[derive(MockComponent)]
pub struct PlayerHp {
    component: HealthPoints,
}

impl PlayerHp {
    pub fn new(hp: Hp) -> Self {
        Self {
            component: HealthPoints::new(hp, BorderSides::RIGHT, Alignment::Left),
        }
    }
}

impl Component<Msg, NoUserEvent> for PlayerHp {
    fn on(&mut self, _ev: Event<NoUserEvent>) -> Option<Msg> {
        None
    }
}

#[derive(MockComponent)]
struct HealthPoints {
    component: Paragraph,
}

impl HealthPoints {
    pub fn new(hp: Hp, sides: BorderSides, alignment: Alignment) -> Self {
        Self {
            component: Paragraph::default()
                .borders(Borders::default().sides(sides))
                .foreground(Color::Red)
                .alignment(alignment)
                .text(&[Self::hp_text(hp)]),
        }
    }

    fn hp_text(hp: Hp) -> TextSpan {
        if hp < 255 {
            TextSpan::from(" ♥ ".repeat(hp as usize))
        } else {
            TextSpan::from("∞")
        }
    }
}
