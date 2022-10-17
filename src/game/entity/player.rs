//! # Player
//!
//! Player entity

use crate::game::inventory::Inventory;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Player {
    /// Player's inventory
    pub inventory: Inventory,
    health: u8,
    max_health: u8,
}

impl Default for Player {
    fn default() -> Self {
        Player {
            health: 3,
            inventory: Inventory::default(),
            max_health: 3,
        }
    }
}

impl Player {
    /// Heal player by qty
    pub fn heal(&mut self, qty: u8) {
        self.health = self.health.saturating_add(qty);
        if self.health > self.max_health() {
            self.health = self.max_health();
        }
    }

    /// Decrease player's health by `qty`
    pub fn decr_health(&mut self, qty: u8) {
        self.health = self.health.saturating_sub(qty);
        if self.health > self.max_health() {
            self.health = self.max_health();
        }
    }

    /// Heal player to maximum health
    pub fn heal_max(&mut self) {
        self.health = self.max_health;
    }

    /// Increase max health by `qty`.
    /// Health is automatically increased by 1
    pub fn incr_max_health(&mut self, qty: u8) {
        self.max_health = self.max_health.saturating_add(qty);
        self.heal(1);
    }

    /// Decrease max health and player's health by quantity
    pub fn decr_max_health(&mut self, qty: u8) {
        if self.max_health > 1 {
            self.max_health = self.max_health.saturating_sub(qty);
        }
        self.decr_health(qty);
    }

    /// Get player's health
    pub fn health(&self) -> u8 {
        self.health
    }

    /// Get max health
    pub fn max_health(&self) -> u8 {
        self.max_health
    }

    /// Returns whether is game over
    pub fn game_over(&self) -> bool {
        self.health == 0
    }
}
