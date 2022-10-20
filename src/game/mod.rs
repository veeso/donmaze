///! # Game
///
/// Main game core engine and logics
pub mod entity;
mod error;
pub mod inventory;
mod maze;
mod options;
mod runtime;
pub mod session;

pub use error::Error as GameError;
pub use options::Options;
pub use runtime::Runtime;
pub use session::Session;

pub type GameResult<T> = Result<T, GameError>;
/// Health points
pub type Hp = u8;
