//! # Title

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
        let margin_left = " ".repeat(area_width as usize);
        Self {
            component: Paragraph::default()
                .borders(Borders::default().sides(BorderSides::NONE))
                .foreground(Color::Red)
                .alignment(Alignment::Left)
                .text(&[
                    TextSpan::from(format!(
                        "{}███    █████  █▅   █     █    █  ███   █████  █████",
                        margin_left
                    )),
                    TextSpan::from(format!(
                        "{}█  █   █   █  █ █  █     ██  ██ █   █     █   █    ",
                        margin_left
                    )),
                    TextSpan::from(format!(
                        "{}█   █  █   █  █  █ █     █ ██ █ █████    █    █████",
                        margin_left
                    )),
                    TextSpan::from(format!(
                        "{}█  █   █   █  █   ██     █    █ █   █   █     █    ",
                        margin_left
                    )),
                    TextSpan::from(format!(
                        "{}███    █████  █    █     █    █ █   █  █████  █████",
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
