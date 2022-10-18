//! # Session
//!
//! A game session. Contains all the current states for the game

use super::{
    entity::{Player, PlayerState},
    maze::Maze,
};
use crate::audio::Sound;

mod action;
mod action_replay;
mod effect;
mod version;

pub use action::{Action, ExploreAction, FightAction};
use action_replay::ActionReplay;
pub use effect::{Effect, Message};
use version::Version;

/// The session contains all the game states.
/// It must be serializable since it is used to save and load games
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Session {
    maze: Maze,
    /// The last room the player's been
    last_room: Option<u32>,
    player: Player,
    /// Turn number
    turn: usize,
    /// Game version; used to check whether this version loaded is compatible
    version: Version,
}

impl Session {
    /// Returns whether session version is compatible with game
    pub fn is_version_compatible(&self) -> bool {
        self.version.is_compatible()
    }

    /// Returns whether previous room is set
    pub fn is_previous_room_set(&self) -> bool {
        self.last_room.is_some()
    }

    /// Play next turn
    pub fn play_turn(&mut self, action: Action) -> Effect {
        self.turn += 1;
        debug!("playing turn {}...", self.turn);
        let mut effect = Effect::default();
        ActionReplay::new(self).play(action, &mut effect);
        self.play_cpu_turn(&mut effect);
        effect
    }

    fn play_cpu_turn(&mut self, effect: &mut Effect) {
        todo!()
    }
}
