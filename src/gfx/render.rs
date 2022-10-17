//! # Render
//!
//! This module exposes the render engine

use tuirealm::props::Color;
use tuirealm::props::Shape;
use tuirealm::tui::widgets::canvas::Rectangle;

// Viewports
const X_SIZE_LARGE: f64 = 4.0;
const Y_SIZE_LARGE: f64 = 6.0;
const X_SIZE_MEDIUM: f64 = 3.0;
const Y_SIZE_MEDIUM: f64 = 4.0;
const X_SIZE_SMALL: f64 = 1.0;
const Y_SIZE_SMALL: f64 = 3.0;

/// Render engine
pub struct Render {
    x_scale: f64,
    y_scale: f64,
}

impl Render {
    /// Instantiate a new render engine
    pub fn new(width: f64, height: f64) -> Self {
        if (width, height) >= (160.0, 36.0) {
            Self {
                x_scale: X_SIZE_LARGE,
                y_scale: Y_SIZE_LARGE,
            }
        } else if (width, height) >= (96.0, 32.0) {
            Self {
                x_scale: X_SIZE_MEDIUM,
                y_scale: Y_SIZE_MEDIUM,
            }
        } else {
            Self {
                x_scale: X_SIZE_SMALL,
                y_scale: Y_SIZE_SMALL,
            }
        }
    }

    /// Render shape from star-shape
    pub fn star_shape(&self, mut x: f64, mut y: f64, data: &str, color: Color) -> Vec<Shape> {
        let newline_x = x;
        let mut shapes = Vec::new();
        for line in data.lines() {
            // reset x
            x = newline_x;
            // iter line chars
            for symbol in line.chars() {
                if symbol == '*' {
                    shapes.push(Shape::Rectangle(Rectangle {
                        x,
                        y,
                        width: self.x_scale,
                        height: self.y_scale,
                        color,
                    }));
                }
                // incr x
                x += self.x_scale;
            }
            // incr y
            y -= self.y_scale;
        }
        shapes
    }

    /// Stack shapes into a stack where each vector is divided by a new layer
    pub fn stack(&self, layers: Vec<Vec<Shape>>) -> Vec<Shape> {
        let mut stack = Vec::new();
        for layer in layers.into_iter() {
            stack.extend(layer);
            stack.push(Shape::Layer);
        }
        stack
    }

    /// Get canvas Y-origin
    pub fn origin_y(&self, canvas_height: f64) -> f64 {
        (canvas_height * self.y_scale) - (4.0 * self.y_scale)
    }
}
