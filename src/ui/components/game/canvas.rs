//! # Canvas
//!
//! Donmaze canvas component

use tui_realm_stdlib::Canvas as TuiCanvas;
use tuirealm::props::Shape;
use tuirealm::ratatui::symbols::Marker;
use tuirealm::{Component, Event, MockComponent, NoUserEvent};

use super::Msg;

/// Main game canvas
#[derive(MockComponent)]
pub struct Canvas {
    component: TuiCanvas,
}

impl Canvas {
    pub fn new(shapes: &[Shape], width: f64, height: f64) -> Self {
        Self {
            component: TuiCanvas::default()
                .data(shapes)
                .marker(Marker::Block)
                .x_bounds((0.0, width))
                .y_bounds((0.0, height)),
        }
    }
}

impl Component<Msg, NoUserEvent> for Canvas {
    fn on(&mut self, _ev: Event<NoUserEvent>) -> Option<Msg> {
        None
    }
}
