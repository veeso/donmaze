//! # Effect
//!
//! The effect defines the outcome for a turn played

use crate::{
    audio::Sound,
    game::{
        entity::{Enemy, Item, Potion},
        Hp,
    },
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
    ArmorEquipped,
    /// Item collected by the player
    ItemCollected(Item),
    /// Item used by the player
    ItemUsed(Item),
    /// A damage in HP inflicted by the player to the enemy
    DamageDealt(Hp),
    /// A damage in HP inflicted by the enemy to the player; bool (is critical?)
    DamageSuffered(Hp, bool),
    /// An enemy has approached the player in his room
    EnemyApproaching(Enemy),
    /// The enemy has been defeated
    EnemyDefeated,
    /// Enemy died or moved due to talisman
    EnemyVanished,
    /// Escape try failed
    EscapeFailed,
    /// Escape try succeeded
    EscapeSucceeded(u32),
    /// Player falls asleep
    FallAsleep,
    /// Leave maze and win
    LeaveMaze,
    /// pleayer is dead
    PlayerDead,
    /// A potion has been drunk
    PotionDrunk(Potion),
    /// Sonar reveal
    Reveal(u32, Reveal),
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

#[derive(Debug, Clone, PartialEq, Eq)]
/// A sonar reveal
pub enum Reveal {
    Item(Item),
    Enemy(Enemy),
}

#[cfg(test)]
mod test {

    use super::*;

    use pretty_assertions::assert_eq;

    #[test]
    fn should_make_effect() {
        let mut effect = Effect::default();
        effect.message(Message::ArmorEquipped);
        effect.sound(Sound::DrinkPotion);
        assert_eq!(effect.messages, vec![Message::ArmorEquipped]);
        assert_eq!(effect.sounds, vec![Sound::DrinkPotion]);
    }
}
