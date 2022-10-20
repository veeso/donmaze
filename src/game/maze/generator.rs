//! # Generator
//!
//! Random maze generator

use crate::game::entity::{Daemon, Enemy, Item, Potion, Shadow};
use crate::utils::random;

use petgraph::{graph::UnGraph, stable_graph::DefaultIx};
use rand::prelude::*;
use rand::thread_rng;
use rand_pcg::Pcg64;
use rand_seeder::Seeder;
use std::collections::HashMap;

use super::room::Room;
use super::Maze;

const MIN_ROOMS: usize = 96;
const MAX_ROOMS: usize = 128;

/// Rangom maze generator
pub struct Generator {
    rand: Pcg64,
    /// a 32 alphanumeric chars long string which is used to generate the maze
    seed: String,
}

impl Generator {
    /// Instantiate a new `Generator`
    pub fn new(seed: Option<String>) -> Self {
        let seed = seed.unwrap_or(Self::random_seed());
        let rand: Pcg64 = Seeder::from(&seed).make_rng();
        Self { rand, seed }
    }

    /// Generate a maze and all the entities in it using generator's seed
    pub fn generate(mut self) -> Maze {
        debug!("generating a maze from seed '{}'", self.seed);
        let rooms_amount = self.rand.gen_range(MIN_ROOMS..(MAX_ROOMS + 1));
        debug!("generating a maze with {} rooms", rooms_amount);
        let (nodes, rooms) = self.generate_rooms(rooms_amount);

        Maze {
            nodes,
            rooms,
            player: 0,
            seed: self.seed,
        }
    }

    /// Generate rooms from amount
    fn generate_rooms(&mut self, rooms_amount: usize) -> (UnGraph<u32, u32>, HashMap<u32, Room>) {
        let items_to_place = self.items_to_place();
        let enemy_to_place = self.enemies_to_place();
        // generate room 0
        let mut nodes: UnGraph<u32, u32> = UnGraph::default();
        let mut rooms: HashMap<DefaultIx, Room> = HashMap::default();
        nodes.add_node(0);
        rooms.insert(0, Room::default());
        for i in 1..rooms_amount {
            debug!("generating room {}...", i);
            let index = nodes.add_node(i as u32);
            rooms.insert(index.index() as u32, Room::default());
        }
        // connect rooms
        todo!("connect rooms");
        // place items in maze
        todo!();
        // place enemies in maze
        todo!();
        // place exit
        todo!("EXIT MUST BE PLACED IN A ROOM WITH LESS THAN 4 ADJACENT NODES");
        let exit = self.rand.gen_range(40..rooms_amount) as u32;
        let exit_node = rooms.get_mut(&exit).unwrap();
        exit_node.is_exit = true;
        debug!("placed exit at node {}", exit);

        (nodes, rooms)
    }

    /// generate enemies to place in the maze
    fn enemies_to_place(&mut self) -> Vec<Enemy> {
        debug!("generating enemies to place...");
        let mut enemies = vec![Enemy::DonMaze];
        let daemons_to_place = self.rand.gen_range(5..16);
        debug!("generating {} daemons...", daemons_to_place);
        for _ in 0..daemons_to_place {
            enemies.push(Enemy::Daemon(self.generate_daemon()));
        }
        let shadows_to_place = self.rand.gen_range(10..21);
        debug!("generating {} shadows...", shadows_to_place);
        for _ in 0..shadows_to_place {
            enemies.push(Enemy::Shadow(self.generate_shadow()));
        }
        debug!("shuffling enemies...");
        enemies.shuffle(&mut self.rand);
        enemies
    }

    /// Generate daemon
    fn generate_daemon(&mut self) -> Daemon {
        let hp = self.rand.gen_range(2..11);
        Daemon::new(hp)
    }

    fn generate_shadow(&mut self) -> Shadow {
        let hp = self.rand.gen_range(2..6);
        Shadow::new(hp)
    }

    /// generate items to place in the maze; the amount is variable, exception made for some items which are always there
    fn items_to_place(&mut self) -> Vec<Item> {
        debug!("generating items to place...");
        let mut items = vec![Item::MazeKey, Item::AlchemyBook];
        let potions_amount = self.rand.gen_range(12..31);
        let armors_amount = self.rand.gen_range(4..9);
        let sonars_amount = self.rand.gen_range(5..8);
        let talismans_amount = self.rand.gen_range(2..5);
        // gen potions
        debug!("generating {} potions...", potions_amount);
        for _ in 0..potions_amount {
            items.push(self.generate_potion());
        }
        debug!("generating {} armors...", armors_amount);
        for _ in 0..armors_amount {
            items.push(Item::Armor);
        }
        debug!("generating {} sonars...", sonars_amount);
        for _ in 0..sonars_amount {
            items.push(Item::Sonar);
        }
        debug!("generating {} talismans...", talismans_amount);
        for _ in 0..talismans_amount {
            items.push(Item::Talisman);
        }
        debug!("shuffling items...");
        items.shuffle(&mut self.rand);
        items
    }

    /// Generate random potion to place
    fn generate_potion(&mut self) -> Item {
        Item::Potion(match self.rand.gen_range(0..100) {
            value if value < 25 => Potion::Mead,         // 25%
            value if value < 45 => Potion::Vinegar,      // 20%
            value if value < 60 => Potion::RedPotion,    // 15%
            value if value < 70 => Potion::SnakePoison,  // 10%
            value if value < 80 => Potion::DaemonsBlood, // 10%
            value if value < 95 => Potion::Chamomille,   // 15%
            95 | 96 | 97 => Potion::UnicornElixir,       // 3%
            98 | 99 => Potion::DeadlyPoison,             // 2%
            _ => panic!("out of range"),
        })
    }

    /// Generate a random seed
    fn random_seed() -> String {
        let mut rng = thread_rng();
        random::random_alphanumeric_with_len(&mut rng, 32)
    }
}

#[cfg(test)]
mod test {

    use super::*;

    use pretty_assertions::assert_eq;

    #[test]
    fn should_generate_a_valid_maze() {
        let maze = Generator::new(None).generate();
        // should have between 96 and 128 rooms
        assert!(maze.rooms.len() >= MIN_ROOMS && maze.rooms.len() <= MAX_ROOMS);
        // should have items
        assert_item_in_maze(&maze, Item::AlchemyBook, 1, 1);
        assert_item_in_maze(&maze, Item::Armor, 4, 8);
        assert_item_in_maze(&maze, Item::MazeKey, 1, 1);
        assert_item_in_maze(&maze, Item::Sonar, 5, 7);
        assert_item_in_maze(&maze, Item::Talisman, 2, 4);
        assert_potion_in_maze(&maze, 12, 30);
        assert_item_in_maze(&maze, Item::AlchemyBook, 1, 1);
        assert_item_in_maze(&maze, Item::AlchemyBook, 1, 1);
        // should have enemies
        let mut found = 0;
        for (_, room) in maze.rooms() {
            if matches!(room.enemy, Some(Enemy::Daemon(_))) {
                found += 1;
            }
        }
        assert!(5 <= found && found <= 15);
        let mut found = 0;
        for (_, room) in maze.rooms() {
            if matches!(room.enemy, Some(Enemy::Shadow(_))) {
                found += 1;
            }
        }
        assert!(10 <= found && found <= 20);
        let mut found = 0;
        for (_, room) in maze.rooms() {
            if matches!(room.enemy, Some(Enemy::DonMaze)) {
                found += 1;
            }
        }
        assert_eq!(found, 1);
        // should have exit
        let mut found = 0;
        let mut exit = None;
        for (node, room) in maze.rooms() {
            if room.is_exit {
                found += 1;
                exit = Some(node);
            }
        }
        assert_eq!(found, 1);
        assert!(exit.is_some());
        // verify that exit room has less than 4 edges
        assert!(
            maze.adjacent_rooms(exit.unwrap()).len() < 4
                && !maze.adjacent_rooms(exit.unwrap()).is_empty()
        );
    }

    #[test]
    fn should_generate_two_equal_mazes_with_same_seed() {
        let mut rng = random::rng();
        let seed = random::random_alphanumeric_with_len(&mut rng, 32);
        let maze_a = Generator::new(Some(seed.clone())).generate();
        let maze_b = Generator::new(Some(seed.clone())).generate();
        assert_eq!(maze_a.seed(), &seed);
        assert_eq!(maze_b.seed(), &seed);
        assert_eq!(maze_a, maze_b);
    }

    fn assert_item_in_maze(maze: &Maze, item: Item, min_qty: usize, max_qty: usize) {
        let mut found = 0;
        for (_, room) in maze.rooms() {
            if Some(item) == room.item {
                found += 1;
            }
        }
        assert!(min_qty <= found && found <= max_qty);
    }

    fn assert_potion_in_maze(maze: &Maze, min_qty: usize, max_qty: usize) {
        let mut found = 0;
        for (_, room) in maze.rooms() {
            if matches!(room.item, Some(Item::Potion(_))) {
                found += 1;
            }
        }
        assert!(min_qty <= found && found <= max_qty);
    }
}
