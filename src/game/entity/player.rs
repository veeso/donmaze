//! # Player
//!
//! Player entity

use crate::game::{inventory::Inventory, Hp};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Player {
    /// Player's inventory
    pub inventory: Inventory,
    health: Hp,
    max_health: Hp,
    state: State,
    /// Turns to sleep for
    sleep_counter: u8,
}

/// Player state
#[derive(Debug, Copy, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub enum State {
    Asleep,
    Explore,
    Fight,
}

impl Default for Player {
    fn default() -> Self {
        Player {
            health: 5,
            inventory: Inventory::default(),
            max_health: 5,
            state: State::Explore,
            sleep_counter: 0,
        }
    }
}

impl Player {
    /// Player state
    pub fn state(&self) -> State {
        self.state
    }

    /// Set player state to explore
    pub fn start_exploring(&mut self) {
        debug!("put player in exploring state");
        self.state = State::Explore;
    }

    /// Set player state to fight
    pub fn start_fighting(&mut self) {
        debug!("put player in fighting state");
        self.state = State::Fight;
    }

    /// Start sleeping for n turns and set state to asleep
    pub fn start_sleeping(&mut self, sleeping_for: u8) {
        debug!("put player in sleeping state");
        self.state = State::Asleep;
        self.sleep_counter = sleeping_for;
    }

    /// Decrease sleep counter
    /// If sleep counter goes to 0; awake
    pub fn decr_sleep_counter(&mut self) {
        self.sleep_counter = self.sleep_counter.saturating_sub(1);
        debug!("decreasing sleep counter; new value {}", self.sleep_counter);
        if self.sleep_counter == 0 {
            self.start_exploring();
        }
    }

    /// Heal player by qty
    pub fn heal(&mut self, qty: Hp) {
        self.health = self.health.saturating_add(qty);
        if self.health > self.max_health() {
            self.health = self.max_health();
        }
        debug!("heal player by {}; new HP {}", qty, self.health);
    }

    /// Decrease player's health by `qty`
    pub fn damage(&mut self, qty: Hp) {
        self.health = self.health.saturating_sub(qty);
        if self.health > self.max_health() {
            self.health = self.max_health();
        }
        debug!("damaged player by {}; new HP {}", qty, self.health);
    }

    /// Heal player to maximum health
    pub fn heal_max(&mut self) {
        self.health = self.max_health;
        debug!("restored player health; new HP {}", self.health);
    }

    /// Increase max health by `qty`.
    /// Health is automatically increased by 1
    pub fn incr_max_health(&mut self, qty: Hp) {
        self.max_health = self.max_health.saturating_add(qty);
        debug!(
            "increased max health by {}; next max HP {}",
            qty, self.max_health
        );
        self.heal(1);
    }

    /// Decrease max health and player's health by quantity
    pub fn decr_max_health(&mut self, qty: Hp) {
        if self.max_health > 1 {
            self.max_health = self.max_health.saturating_sub(qty);
        }
        debug!(
            "decreased max health by {}; next max HP {}",
            qty, self.max_health
        );
        self.damage(qty);
    }

    /// Get player's health
    pub fn health(&self) -> Hp {
        self.health
    }

    /// Get max health
    pub fn max_health(&self) -> Hp {
        self.max_health
    }

    /// Returns whether is dead
    pub fn is_dead(&self) -> bool {
        self.health == 0
    }
}
