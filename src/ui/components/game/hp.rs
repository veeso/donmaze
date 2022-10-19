//! # hp

use crate::game::Hp;
use crate::gfx::ascii_art::HEARTH;

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
                .text(&Self::draw_hp(hp)),
        }
    }

    fn draw_hp(hp: Hp) -> Vec<TextSpan> {
        // make text spans from ascii art
        let lines: Vec<String> = HEARTH.lines().map(|x| x.to_string()).collect();
        let mut spans: Vec<TextSpan> = Vec::with_capacity(hp as usize);
        for line in lines {
            let mut line_span = String::new();
            for _ in 0..hp {
                line_span.push_str(&line);
                line_span.push_str(" ");
            }
            spans.push(TextSpan::from(line_span));
        }

        spans
    }
}
