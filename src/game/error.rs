//! # Game error
//!
//! Game error types

use thiserror::Error;

#[derive(Debug, Error, Copy, Clone, Eq, PartialEq)]
pub enum Error {}
