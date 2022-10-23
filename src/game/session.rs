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
    #[cfg(not(test))]
    maze: Maze,
    #[cfg(test)]
    pub maze: Maze,
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

    /// Get last room
    pub fn get_last_room(&self) -> Option<u32> {
        self.last_room
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

    /// Return player's room
    pub fn player_room(&self) -> u32 {
        self.maze.player
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

    /// Returns whether current room is exit
    pub fn is_exit(&self) -> bool {
        self.maze.is_exit()
    }

    /// Returns whether player should be able to use items
    pub fn can_use_items(&self) -> bool {
        self.player.state() != PlayerState::Asleep
    }

    /// Get adjacent rooms ids for user
    pub fn adjacent_rooms(&self) -> Vec<u32> {
        self.maze
            .adjacent_rooms(self.maze.player)
            .into_iter()
            .map(|(id, _)| id)
            .collect()
    }

    /// Play next turn
    pub fn play_turn(&mut self, action: Action) -> Effect {
        self.stats.turn += 1;
        self.stats.last_turn = Local::now();
        debug!("playing turn {}...", self.stats.turn);
        let mut effect = Effect::default();
        ActionReplay::new(self).play(action, &mut effect);
        // Check whether player has won; otherwise play cpu turn
        if action == Action::Explore(ExploreAction::LeaveMaze) {
            self.won = true;
        } else {
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
                Action::Fight(FightAction::Fight),
                Action::Fight(FightAction::Escape),
            ],
        }
    }

    /// Returns available exploring actions
    /// Does not include actions related to victory or loss
    fn available_exploring_actions(&self) -> Vec<Action> {
        let mut actions = Vec::with_capacity(6);
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
        if self.is_previous_room_set() {
            actions.push(Action::Explore(ExploreAction::GoToPreviousRoom));
        }

        actions
    }

    #[cfg(test)]
    pub fn set_last_room(&mut self, room: u32) {
        self.last_room = Some(room);
    }

    #[cfg(test)]
    pub fn mock() -> Self {
        Self::mock_with_maze(Maze::mocked())
    }

    #[cfg(test)]
    pub fn mock_with_maze(maze: Maze) -> Self {
        Self {
            maze,
            last_room: None,
            player: Player::default(),
            stats: Stats::default(),
            version: Version::V010,
            won: false,
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;

    use pretty_assertions::assert_eq;

    #[test]
    fn should_tell_whether_version_is_compatible() {
        let session = Session::mock();
        assert!(session.is_version_compatible());
    }

    #[test]
    fn should_tell_whether_last_room_is_set() {
        let mut session = Session::mock();
        assert_eq!(session.is_previous_room_set(), false);
        session.last_room = Some(4);
        assert_eq!(session.is_previous_room_set(), true);
        assert_eq!(session.get_last_room(), Some(4));
    }

    #[test]
    fn should_tell_maze_seed() {
        let session = Session::mock();
        assert_eq!(session.maze_seed(), "test");
    }

    #[test]
    fn should_return_player_inventory() {
        let session = Session::mock();
        assert_eq!(session.player_inventory().items().count(), 0);
    }

    #[test]
    fn should_return_game_stats() {
        let session = Session::mock();
        assert_eq!(session.stats().damage_inflicted, 0);
    }

    #[test]
    fn should_return_adjacent_rooms() {
        let session = Session::mock();
        assert_eq!(session.adjacent_rooms(), vec![2, 1]);
    }

    #[test]
    fn should_return_player_room() {
        let mut session = Session::mock();
        session.maze.player = 2;
        assert_eq!(session.player_room(), 2);
    }

    #[test]
    fn should_return_fighting_enemy() {
        let mut session = Session::mock();
        assert!(session.get_fighting_enemy().is_none());
        session.maze.player = 2;
        assert!(session.get_fighting_enemy().is_some());
    }

    #[test]
    fn should_return_item_in_the_room() {
        let mut session = Session::mock();
        assert!(session.get_item_in_the_room().is_none());
        session.maze.player = 1;
        assert!(session.get_item_in_the_room().is_some());
    }

    #[test]
    fn should_return_available_actions() {
        let mut session = Session::mock();
        assert_eq!(
            session.available_actions(),
            vec![
                Action::Explore(ExploreAction::ChangeRoom(2)),
                Action::Explore(ExploreAction::ChangeRoom(1)),
            ]
        );
        // go to room 1
        session.last_room = Some(0);
        session.maze.player = 1;
        assert_eq!(
            session.available_actions(),
            vec![
                Action::Explore(ExploreAction::CollectItem),
                Action::Explore(ExploreAction::ChangeRoom(9)),
                Action::Explore(ExploreAction::ChangeRoom(3)),
                Action::Explore(ExploreAction::GoToPreviousRoom),
            ]
        );
        // set asleep
        session.player.start_sleeping(1);
        assert_eq!(session.available_actions(), vec![Action::Sleep]);
        // start fight
        session.maze.player = 2;
        session.player.decr_sleep_counter();
        session.player.start_fighting();
        assert_eq!(
            session.available_actions(),
            vec![
                Action::Fight(FightAction::Fight),
                Action::Fight(FightAction::Escape),
            ]
        );
        // win
        session.player.inventory.add(Item::MazeKey);
        session.player.start_exploring();
        session.maze.player = 7;
        assert_eq!(session.is_exit(), true);
        assert_eq!(
            session.available_actions(),
            vec![Action::Explore(ExploreAction::LeaveMaze)]
        );
    }

    #[test]
    fn should_tell_whether_player_can_use_items() {
        let mut session = Session::mock();
        assert!(session.can_use_items());
        session.player.start_sleeping(3);
        assert!(!session.can_use_items());
    }
}
