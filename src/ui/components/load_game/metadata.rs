//! # Metadata

use super::Msg;
use crate::game::Session;

use tui_realm_stdlib::Paragraph;
use tuirealm::props::{Alignment, BorderSides, Borders, TextSpan};
use tuirealm::{Component, Event, MockComponent, NoUserEvent};

#[derive(MockComponent)]
pub struct Metadata {
    component: Paragraph,
}

impl Metadata {
    pub fn new(session: &Session) -> Self {
        Self {
            component: Paragraph::default()
                .alignment(Alignment::Center)
                .borders(Borders::default().sides(BorderSides::NONE))
                .text(&[
                    TextSpan::from(format!("seed: {}", session.maze_seed())),
                    TextSpan::from(format!(
                        "last turn: {}",
                        session.stats().last_turn.to_rfc2822()
                    )),
                    TextSpan::from(format!("turn: {}", session.stats().turn)),
                    TextSpan::from("♥".repeat(session.player().health() as usize)),
                ])
                .wrap(true),
        }
    }
}

impl Component<Msg, NoUserEvent> for Metadata {
    fn on(&mut self, _ev: Event<NoUserEvent>) -> Option<Msg> {
        None
    }
}
