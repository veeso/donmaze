//! # Title

use super::Msg;

use tui_realm_stdlib::Canvas;
use tuirealm::props::Shape;
use tuirealm::tui::symbols::Marker;
use tuirealm::NoUserEvent;
use tuirealm::{Component, Event, MockComponent};

#[derive(MockComponent)]
pub struct Title {
    component: Canvas,
}

impl Title {
    pub fn new(shapes: &[Shape], width: f64, height: f64) -> Self {
        Self {
            component: Canvas::default()
                .data(shapes)
                .marker(Marker::Block)
                .x_bounds((-width / 2.0, width / 2.0))
                .y_bounds((0.0, height)),
        }
    }
}

impl Component<Msg, NoUserEvent> for Title {
    fn on(&mut self, _ev: Event<NoUserEvent>) -> Option<Msg> {
        None
    }
}
