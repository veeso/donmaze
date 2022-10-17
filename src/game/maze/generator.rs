//! # Generator
//!
//! Random maze generator

use crate::game::entity::{Daemon, Enemy, Item, Potion};
use crate::utils::random;

use petgraph::graph::UnGraph;
use rand::prelude::*;
use rand::thread_rng;
use rand_pcg::Pcg64;
use rand_seeder::Seeder;
use std::collections::HashMap;

use super::room::Room;
use super::Maze;

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
        let rooms_amount = self.rand.gen_range(96..129);
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
        let mut rooms: HashMap<u32, Room> = HashMap::default();
        nodes.add_node(0);
        rooms.insert(0, Room::default());
        for i in 1..rooms_amount {
            debug!("generating room {}...", i);
            todo!("generate room");
            rooms.insert(i as u32, Room::default());
        }
        // place items in maze
        todo!();
        // place enemies in maze
        todo!();
        // place exit
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
        let daemons_to_place = self.rand.gen_range(10..18);
        debug!("generating {} daemons...", daemons_to_place);
        for _ in 0..daemons_to_place {
            enemies.push(Enemy::Daemon(self.generate_daemon()));
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

    /// generate items to place in the maze; the amount is variable, exception made for some items which are always there
    fn items_to_place(&mut self) -> Vec<Item> {
        debug!("generating items to place...");
        let mut items = vec![Item::Map, Item::MazeKey, Item::Sonar, Item::AlchemyBook];
        let potions_amount = self.rand.gen_range(8..17);
        let armors_amount = self.rand.gen_range(4..9);
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
