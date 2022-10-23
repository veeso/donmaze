//! # Player
//!
//! Player entity

use crate::game::{inventory::Inventory, Hp};

const BASE_PLAYER_HEALTH: Hp = 10;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
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
            health: BASE_PLAYER_HEALTH,
            inventory: Inventory::default(),
            max_health: BASE_PLAYER_HEALTH,
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

#[cfg(test)]
mod test {

    use super::*;

    use pretty_assertions::assert_eq;

    #[test]
    fn should_init_player() {
        let player = Player::default();
        assert_eq!(player.health(), BASE_PLAYER_HEALTH);
        assert_eq!(player.inventory.items().len(), 0);
        assert_eq!(player.max_health(), BASE_PLAYER_HEALTH);
        assert_eq!(player.state(), State::Explore);
        assert_eq!(player.sleep_counter, 0);
    }

    #[test]
    fn should_heal_and_damage() {
        let mut player = Player::default();
        assert_eq!(player.health(), BASE_PLAYER_HEALTH);
        player.heal(3);
        assert_eq!(player.health(), BASE_PLAYER_HEALTH);
        player.incr_max_health(3);
        assert_eq!(player.health(), BASE_PLAYER_HEALTH + 1);
        assert_eq!(player.max_health(), BASE_PLAYER_HEALTH + 3);
        player.damage(5);
        assert_eq!(player.health(), BASE_PLAYER_HEALTH + 1 - 5);
        assert_eq!(player.max_health(), BASE_PLAYER_HEALTH + 3);
        player.heal_max();
        assert_eq!(player.health(), player.max_health());
        player.decr_max_health(4);
        assert_eq!(player.health(), BASE_PLAYER_HEALTH + 1 - 3 + 1);
        assert_eq!(player.max_health(), BASE_PLAYER_HEALTH - 1);
        player.damage(9);
        assert_eq!(player.health(), 0);
    }

    #[test]
    fn should_tell_whether_is_game_over() {
        let mut player = Player::default();
        assert_eq!(player.is_dead(), false);
        player.damage(255);
        assert_eq!(player.is_dead(), true);
    }

    #[test]
    fn should_change_status() {
        let mut player = Player::default();
        player.start_fighting();
        assert_eq!(player.state(), State::Fight);
        player.start_exploring();
        assert_eq!(player.state(), State::Explore);
        player.start_sleeping(2);
        assert_eq!(player.sleep_counter, 2);
        assert_eq!(player.state(), State::Asleep);
        player.decr_sleep_counter();
        assert_eq!(player.state(), State::Asleep);
        assert_eq!(player.sleep_counter, 1);
        player.decr_sleep_counter();
        assert_eq!(player.state(), State::Explore);
        assert_eq!(player.sleep_counter, 0);
    }

    #[test]
    fn should_serialize() {
        #[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
        struct Test {
            player: Player,
        }
        let test = Test {
            player: Player::default(),
        };
        let json = serde_json::to_string(&test).unwrap();
        let decoded: Test = serde_json::from_str(&json).unwrap();
        assert_eq!(test, decoded);
    }
}
