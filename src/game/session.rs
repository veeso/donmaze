//! # Session
//!
//! A game session. Contains all the current states for the game

use super::{
    entity::{Enemy, Item, Player, PlayerState},
    inventory::Inventory,
    maze::Maze,
};
use crate::audio::Sound;

use chrono::Local;

mod action;
mod action_replay;
mod cpu;
mod effect;
mod stats;
mod version;

pub use action::{Action, ExploreAction, FightAction};
use action_replay::ActionReplay;
use cpu::Cpu;
pub use effect::{Effect, Message, Reveal};
pub use stats::Stats;
use version::Version;

/// The session contains all the game states.
/// It must be serializable since it is used to save and load games
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Session {
    maze: Maze,
    /// The last room the player's been
    last_room: Option<u32>,
    player: Player,
    /// Game stats
    stats: Stats,
    /// Game version; used to check whether this version loaded is compatible
    version: Version,
    /// has the player won
    won: bool,
}

impl Session {
    /// Create a new session
    pub fn new(seed: Option<String>) -> Self {
        Self {
            maze: Maze::generate(seed),
            last_room: None,
            player: Player::default(),
            stats: Stats::default(),
            version: Version::V010,
            won: false,
        }
    }

    /// Returns whether session version is compatible with game
    pub fn is_version_compatible(&self) -> bool {
        self.version.is_compatible()
    }

    /// Returns whether previous room is set
    pub fn is_previous_room_set(&self) -> bool {
        self.last_room.is_some()
    }

    /// Get maze seed
    pub fn maze_seed(&self) -> &str {
        self.maze.seed()
    }

    /// Get player inventory
    pub fn player_inventory(&self) -> &Inventory {
        &self.player.inventory
    }

    /// Leave maze
    fn leave_maze(&mut self) {
        self.won = true;
    }

    /// Report player has won
    pub fn has_won(&self) -> bool {
        self.won
    }

    /// Returns whether is game over
    pub fn game_over(&self) -> bool {
        self.player.is_dead()
    }

    /// Get read only player
    pub fn player(&self) -> &Player {
        &self.player
    }

    /// Get game stats
    pub fn stats(&self) -> &Stats {
        &self.stats
    }

    /// Get fighting enemy
    pub fn get_fighting_enemy(&self) -> Option<&Enemy> {
        self.maze.fighting_enemy()
    }

    pub fn get_item_in_the_room(&self) -> Option<&Item> {
        self.maze.item_in_room()
    }

    /// Play next turn
    pub fn play_turn(&mut self, action: Action) -> Effect {
        self.stats.turn += 1;
        self.stats.last_turn = Local::now();
        debug!("playing turn {}...", self.stats.turn);
        let mut effect = Effect::default();
        ActionReplay::new(self).play(action, &mut effect);
        // Check whether player has won
        if action == Action::Explore(ExploreAction::LeaveMaze) {
            self.won = true;
        }
        // if player has won, doon't play turn for cpu
        if !self.has_won() {
            Cpu::new(self).play(&mut effect);
        }
        effect
    }

    /// Returns player's available actions for the current turn
    pub fn available_actions(&self) -> Vec<Action> {
        // game over
        if self.game_over() {
            return vec![Action::Die];
        }
        // win if in exit with maze key and not fighting or sleeping
        if self.maze.is_exit()
            && self.player.state() == PlayerState::Explore
            && self.player_inventory().has(Item::MazeKey)
        {
            return vec![Action::Explore(ExploreAction::LeaveMaze)];
        }
        // normal cases
        match self.player.state() {
            PlayerState::Asleep => vec![Action::Sleep],
            PlayerState::Explore => self.available_exploring_actions(),
            PlayerState::Fight => vec![
                Action::Fight(FightAction::Escape),
                Action::Fight(FightAction::Fight),
            ],
        }
    }

    /// Returns available exploring actions
    /// Does not include actions related to victory or loss
    fn available_exploring_actions(&self) -> Vec<Action> {
        let mut actions = Vec::with_capacity(6);
        if self.is_previous_room_set() {
            actions.push(Action::Explore(ExploreAction::GoToPreviousRoom));
        }
        if self.maze.has_item() {
            actions.push(Action::Explore(ExploreAction::CollectItem));
        }
        // push adjacent rooms, except last room
        for (node, _) in self
            .maze
            .adjacent_rooms(self.maze.player)
            .iter()
            .filter(|(node, _)| Some(*node) != self.last_room)
        {
            actions.push(Action::Explore(ExploreAction::ChangeRoom(*node)));
        }

        actions
    }

    #[cfg(test)]
    pub fn mock() -> Self {
        Self {
            maze: Maze::mocked(),
            last_room: None,
            player: Player::default(),
            stats: Stats::default(),
            version: Version::V010,
            won: false,
        }
    }
}
