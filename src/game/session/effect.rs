//! # Effect
//!
//! The effect defines the outcome for a turn played

use crate::{
    audio::Sound,
    game::{entity::Item, Hp},
};

/// Defines the effect of a turn
#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct Effect {
    /// A list of messages to report to the ui
    pub messages: Vec<Message>,
    /// List of sounds to play
    pub sounds: Vec<Sound>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Defines a message to report to the UI
pub enum Message {
    /// Item used by the user
    ItemUsed(Item),
    /// A damage in HP inflicted by the player to the enemy
    DamageDealt(Hp),
    /// A damage in HP inflicted by the enemy to the player
    DamageSuffered(Hp),
    /// The enemy has been defeated
    EnemyDefeated,
    FallAsleep,
    PlayerDead,
    Sleeping,
    /// Wake up from sleeping
    WakeUp,
}

impl Effect {
    /// Add sound to effect
    pub(super) fn sound(&mut self, s: Sound) {
        self.sounds.push(s);
    }

    /// Add message to effect
    pub(super) fn message(&mut self, m: Message) {
        self.messages.push(m);
    }
}
