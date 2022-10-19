//! # Metadata

use super::Msg;

use chrono::{DateTime, Local};
use tui_realm_stdlib::Label;
use tuirealm::{Component, Event, MockComponent, NoUserEvent};

#[derive(MockComponent)]
pub struct LastTurn {
    component: Label,
}

impl LastTurn {
    pub fn new(turn: DateTime<Local>) -> Self {
        Self {
            component: Label::default().text(format!("last turn: {}", turn.to_rfc3339())),
        }
    }
}

impl Component<Msg, NoUserEvent> for LastTurn {
    fn on(&mut self, _ev: Event<NoUserEvent>) -> Option<Msg> {
        None
    }
}

#[derive(MockComponent)]
pub struct Seed {
    component: Label,
}

impl Seed {
    pub fn new(seed: String) -> Self {
        Self {
            component: Label::default().text(format!("seed: {}", seed)),
        }
    }
}

impl Component<Msg, NoUserEvent> for Seed {
    fn on(&mut self, _ev: Event<NoUserEvent>) -> Option<Msg> {
        None
    }
}

#[derive(MockComponent)]
pub struct Turn {
    component: Label,
}

impl Turn {
    pub fn new(turn: usize) -> Self {
        Self {
            component: Label::default().text(format!("turn: {}", turn)),
        }
    }
}

impl Component<Msg, NoUserEvent> for Turn {
    fn on(&mut self, _ev: Event<NoUserEvent>) -> Option<Msg> {
        None
    }
}
