//! # Generator
//!
//! Random maze generator

use crate::game::entity::{Daemon, Enemy, Item, Potion, Shadow};
use crate::utils::random;

use petgraph::stable_graph::NodeIndex;
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
        let seed = seed.unwrap_or_else(Self::random_seed);
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
        let enemies_to_place = self.enemies_to_place();
        // generate room 0
        let mut nodes: UnGraph<u32, u32> = UnGraph::default();
        let mut rooms: HashMap<DefaultIx, Room> = HashMap::default();
        let room_0 = nodes.add_node(0);
        rooms.insert(0, Room::default());
        let mut rooms_to_connect: Vec<NodeIndex> = Vec::with_capacity(rooms_amount);
        for i in 1..rooms_amount {
            debug!("generating room {}...", i);
            let index = nodes.add_node(i as u32);
            rooms.insert(index.index() as u32, Room::default());
            rooms_to_connect.push(index);
        }
        // Generate room iterator; NOTE: rev() because rooms are popped from tail
        let mut rooms_to_connect: Vec<NodeIndex> = rooms_to_connect.into_iter().rev().collect();
        // connect rooms
        let nodes = self.connect_rooms(nodes, room_0, &mut rooms_to_connect, false);
        // assert rooms to connect is empty
        assert!(rooms_to_connect.is_empty());
        // place items in maze
        self.place_items_in_maze(&mut rooms, items_to_place);
        // place enemies in maze
        self.place_enemies_in_maze(&mut rooms, enemies_to_place);
        // place exit
        self.place_maze_exit(&nodes, &mut rooms);

        (nodes, rooms)
    }

    /// Recursive function to connect rooms until rooms_to_connect is empty
    fn connect_rooms(
        &mut self,
        mut nodes: UnGraph<u32, u32>,
        previous_room: NodeIndex,
        rooms_to_connect: &mut Vec<NodeIndex>,
        can_be_dead_end: bool,
    ) -> UnGraph<u32, u32> {
        // @! base case
        if rooms_to_connect.is_empty() {
            debug!("rooms to connect is empty; returning nodes");
            return nodes;
        }
        // @! recursive case
        let current_room = rooms_to_connect.pop().unwrap();
        debug!("connecting edges for {}...", current_room.index());
        let edges_for_room = self.edges_for_room(can_be_dead_end);
        debug!(
            "room {} will have {} edges",
            current_room.index(),
            edges_for_room + 1
        );
        // connect previous room
        nodes.add_edge(previous_room, current_room, 0);
        debug!(
            "connected previous room {} to {}",
            previous_room.index(),
            current_room.index()
        );
        let mut rooms_to_connect_chunks =
            self.make_rooms_to_connect_chunks(rooms_to_connect, edges_for_room);
        debug!(
            "rooms to connect was {:?}; chunks are {:?}",
            rooms_to_connect, rooms_to_connect_chunks
        );
        // iter over new edges
        for (i, item) in rooms_to_connect_chunks
            .iter_mut()
            .enumerate()
            .take(edges_for_room)
        {
            nodes = self.connect_rooms(nodes, current_room, item, i != edges_for_room - 1);
        }
        nodes
    }

    /// Randomize the amount of edges per room
    ///
    /// 25% -> 3 + 1
    /// 45% -> 2 + 1
    /// 20 % -> 1 + 1
    /// 10 % -> dead-end (if can be dead end; 1 otherwise)
    fn edges_for_room(&mut self, can_be_dead_end: bool) -> usize {
        match self.rand.gen_range(0..100) {
            x if x < 25 => 3,
            x if x < 70 => 2,
            x if x < 90 => 1,
            _ if can_be_dead_end => 0, // dead end
            _ => 1,
        }
    }

    /// Starting from rooms to connect, returns `chunks` vector randomly distribuited to create different branches for the maze
    fn make_rooms_to_connect_chunks(
        &mut self,
        rooms_to_connect: &mut Vec<NodeIndex>,
        chunks_count: usize,
    ) -> Vec<Vec<NodeIndex>> {
        let mut chunks = Vec::with_capacity(chunks_count);
        let total_rooms = rooms_to_connect.len();
        for i in 0..chunks_count {
            if i == chunks_count - 1 {
                // push remaining rooms
                let mut new_chunk = Vec::with_capacity(rooms_to_connect.len());
                while let Some(room) = rooms_to_connect.pop() {
                    new_chunk.push(room);
                }
                chunks.push(new_chunk);
            } else {
                // chunk
                let one_tenth = ((chunks_count * 10) / 100) as isize;
                let variadic = if one_tenth == 0 {
                    0
                } else {
                    self.rand.gen_range(-one_tenth..one_tenth)
                };
                let chunks_rooms = if variadic < 0 {
                    (total_rooms / chunks_count).saturating_sub(variadic.unsigned_abs())
                } else {
                    (total_rooms / chunks_count).saturating_add(variadic as usize)
                };
                let mut new_chunk = Vec::with_capacity(chunks_rooms);
                for _ in 0..chunks_rooms {
                    if let Some(r) = rooms_to_connect.pop() {
                        new_chunk.push(r);
                    }
                }
                chunks.push(new_chunk);
            }
        }

        chunks
    }

    /// place items in maze randomly
    fn place_items_in_maze(
        &mut self,
        rooms: &mut HashMap<DefaultIx, Room>,
        mut items_to_place: Vec<Item>,
    ) {
        // keep placing items, until all items have been placed
        while let Some(item) = items_to_place.pop() {
            // get rooms which are still without any item and NOT room 0
            let mut rooms_without_items: Vec<u32> = rooms
                .iter()
                .filter(|(node, room)| room.item.is_none() && **node != 0u32)
                .map(|(node, _)| *node)
                .collect();
            rooms_without_items.sort(); // NOTE: sorting is necessary, since otherwise rooms are randomly sorted based on hashmap order
                                        // choose the room where the item should be placed
            let room = rooms_without_items[self.rand.gen_range(0..rooms_without_items.len())];
            let mut room_data = rooms.get_mut(&room).unwrap();
            room_data.item = Some(item);
            debug!("placed item {:?} in room {}", item, room);
        }
    }

    /// place enemies in maze randomly
    fn place_enemies_in_maze(
        &mut self,
        rooms: &mut HashMap<DefaultIx, Room>,
        mut enemies_to_place: Vec<Enemy>,
    ) {
        // keep placing enemies, until all enemies have been placed
        while let Some(enemy) = enemies_to_place.pop() {
            // get rooms which are still without any ENEMY and NOT room 0
            let mut rooms_without_enemies: Vec<u32> = rooms
                .iter()
                .filter(|(node, room)| room.enemy.is_none() && **node != 0u32)
                .map(|(node, _)| *node)
                .collect();
            rooms_without_enemies.sort(); // NOTE: sorting is necessary, since otherwise rooms are randomly sorted based on hashmap order
                                          // choose the room where the enemy should be placed
            let room = rooms_without_enemies[self.rand.gen_range(0..rooms_without_enemies.len())];
            let mut room_data = rooms.get_mut(&room).unwrap();
            room_data.enemy = Some(enemy);
            debug!("placed enemy {:?} in room {}", enemy, room);
        }
    }

    /// Place maze exit.
    /// Exit can be placed in ANY room WITH LESS THAN 4 EDGES and with ID > 40
    fn place_maze_exit(
        &mut self,
        nodes: &UnGraph<DefaultIx, DefaultIx>,
        rooms: &mut HashMap<DefaultIx, Room>,
    ) {
        let mut compatible_rooms: Vec<u32> = Vec::with_capacity(rooms.len());
        for i in 40..rooms.len() as u32 {
            if nodes.edges(i.into()).count() < 4 {
                debug!("room {} is suitable for being chosen as exit", i);
                compatible_rooms.push(i);
            }
        }
        compatible_rooms.sort(); // NOTE: sorting is necessary, since otherwise rooms are randomly sorted based on hashmap order
        let room = compatible_rooms[self.rand.gen_range(0..compatible_rooms.len())];
        debug!("chosen room {} for exit", room);
        let mut room_data = rooms.get_mut(&room).unwrap();
        room_data.is_exit = true;
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
        let hp = self.rand.gen_range(2..8);
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
        assert!((5..=15).contains(&found));
        let mut found = 0;
        for (_, room) in maze.rooms() {
            if matches!(room.enemy, Some(Enemy::Shadow(_))) {
                found += 1;
            }
        }
        assert!((10..=20).contains(&found));
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
        assert!(exit.unwrap() >= 40);
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
