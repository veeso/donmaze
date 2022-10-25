//! # Texts

use crate::game::Session;

use super::Msg;

use tui_realm_stdlib::Paragraph;
use tuirealm::props::{Alignment, BorderSides, Borders, Color, TextSpan};
use tuirealm::NoUserEvent;
use tuirealm::{Component, Event, MockComponent};

#[derive(MockComponent)]
pub struct Title {
    component: Paragraph,
}

impl Title {
    pub fn new(area_width: u16) -> Self {
        let margin_left = " ".repeat((area_width as usize / 2) - 21);
        Self {
            component: Paragraph::default()
                .borders(Borders::default().sides(BorderSides::NONE))
                .foreground(Color::Red)
                .alignment(Alignment::Left)
                .text(&[
                    TextSpan::from(format!(
                        "{}█   █  █   ████  █████   ███    ███  █   █",
                        margin_left
                    )),
                    TextSpan::from(format!(
                        "{}█   █     █        █    █   █  █      █ █ ",
                        margin_left
                    )),
                    TextSpan::from(format!(
                        "{}█   █  █  █        █    █   █  █       █  ",
                        margin_left
                    )),
                    TextSpan::from(format!(
                        "{}█ █   █  █        █    █   █  █       █  ",
                        margin_left
                    )),
                    TextSpan::from(format!(
                        "{}█    █   ████    █     ███   █       █  ",
                        margin_left
                    )),
                ]),
        }
    }
}

impl Component<Msg, NoUserEvent> for Title {
    fn on(&mut self, _ev: Event<NoUserEvent>) -> Option<Msg> {
        None
    }
}

#[derive(MockComponent)]
pub struct Stats {
    component: Paragraph,
}

impl Stats {
    pub fn new(session: &Session) -> Self {
        Self {
            component: Paragraph::default()
                .borders(Borders::default().sides(BorderSides::NONE))
                .foreground(Color::Reset)
                .alignment(Alignment::Center)
                .text(&[
                    TextSpan::from("You escaped the maze!"),
                    TextSpan::from(""),
                    TextSpan::from(format!(
                        "Damage inflicted: {} HP",
                        session.stats().damage_inflicted
                    )),
                    TextSpan::from(format!(
                        "Damage suffered: {} HP",
                        session.stats().damage_suffered
                    )),
                    TextSpan::from(format!(
                        "Enemies killed: {}",
                        session.stats().enemies_killed
                    )),
                    TextSpan::from(format!(
                        "Fights escaped: {}",
                        session.stats().fights_escaped
                    )),
                    TextSpan::from(format!("Items used: {}", session.stats().items_used)),
                    TextSpan::from(format!("Rooms explored: {}", session.visited_rooms())),
                    TextSpan::from(format!(
                        "You've been asleep for {} turns",
                        session.stats().slept_for_turns
                    )),
                    TextSpan::from(format!("You played for {} turns", session.stats().turn)),
                    TextSpan::from(""),
                    TextSpan::from("Thank you for playing!"),
                ]),
        }
    }
}

impl Component<Msg, NoUserEvent> for Stats {
    fn on(&mut self, _ev: Event<NoUserEvent>) -> Option<Msg> {
        None
    }
}
