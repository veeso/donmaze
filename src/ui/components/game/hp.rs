//! # hp

use tui_realm_stdlib::Label;
use tuirealm::props::{Alignment, Color};
use tuirealm::{Component, Event, MockComponent, NoUserEvent};

use super::Msg;
use crate::game::Hp;

#[derive(MockComponent)]
pub struct EnemyHp {
    component: HealthPoints,
}

impl EnemyHp {
    pub fn new(hp: Hp) -> Self {
        Self {
            component: HealthPoints::new(hp, Alignment::Right),
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
            component: HealthPoints::new(hp, Alignment::Left),
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
    component: Label,
}

impl HealthPoints {
    pub fn new(hp: Hp, alignment: Alignment) -> Self {
        Self {
            component: Label::default()
                .foreground(Color::Red)
                .alignment(alignment)
                .text(Self::hp_text(hp)),
        }
    }

    fn hp_text(hp: Hp) -> String {
        if hp < 255 {
            " ♥ ".repeat(hp as usize)
        } else {
            "∞".to_string()
        }
    }
}
