//! # Action replay

use rand::Rng;

use super::{
    Action, Effect, ExploreAction, FightAction, Message, PlayerState, Reveal, Session, Sound,
};
use crate::game::entity::{Enemy, Item, Potion};
use crate::utils::random;
use crate::utils::room_resolver;

const ESCAPE_PROBABILITY: u8 = 50;

/// The action replay is used to play actions performed by the player
pub struct ActionReplay<'a> {
    session: &'a mut Session,
}

impl<'a> ActionReplay<'a> {
    pub fn new(session: &'a mut Session) -> Self {
        Self { session }
    }

    /// Play action
    pub fn play(mut self, action: Action, effect: &mut Effect) {
        debug!("playing player action: {:?}", action);
        match action {
            Action::Die => self.die(effect),
            Action::Explore(explore) => self.play_explore_action(explore, effect),
            Action::Fight(fight) => self.play_fight_action(fight, effect),
            Action::SaveGame => self.save_game(effect),
            Action::Sleep => self.sleep(effect),
            Action::UseItem(item) => self.use_item(item, effect),
        }
    }

    /// Player dies. Game over
    fn die(&mut self, effect: &mut Effect) {
        debug!("ho-ho. You're dead.");
        effect.message(Message::PlayerDead);
        effect.sound(Sound::PlayerDead);
    }

    fn save_game(&self, effect: &mut Effect) {
        debug!("game saved");
        effect.message(Message::GameSaved);
        effect.sound(Sound::GameSaved);
    }

    /// Play explore action
    fn play_explore_action(&mut self, action: ExploreAction, effect: &mut Effect) {
        assert_eq!(self.session.player.state(), PlayerState::Explore);
        debug!("playing explore action: {:?}", action);
        match action {
            ExploreAction::ChangeRoom(room) => self.change_room(room, effect),
            ExploreAction::CollectItem => self.collect_item(effect),
            ExploreAction::GoToPreviousRoom => self.go_to_previous_room(effect),
            ExploreAction::LeaveMaze => self.leave_maze(effect),
        }
    }

    /// Play fight action
    fn play_fight_action(&mut self, action: FightAction, effect: &mut Effect) {
        assert_eq!(self.session.player.state(), PlayerState::Fight);
        debug!("playing fight action: {:?}", action);
        match action {
            FightAction::Escape => self.escape(effect),
            FightAction::Fight => self.fight(effect),
        }
    }

    /// Change room to provided node
    fn change_room(&mut self, room: u32, effect: &mut Effect) {
        assert!(self.session.maze.room_adjacent(room));
        debug!(
            "going from room {} to room {}",
            self.session.maze.player, room
        );
        effect.message(Message::RoomChanged(room_resolver::resolve_room_direction(
            room,
            self.session,
        )));
        // put previous room to visited rooms
        self.session.visit_room(self.session.maze.player);
        self.session.last_room = Some(self.session.maze.player);
        self.session.maze.player = room;
        effect.sound(Sound::Steps);
    }

    /// Collect item at room and put it into the inventory
    fn collect_item(&mut self, effect: &mut Effect) {
        // get item in room
        let item = self.session.maze.take_item().unwrap();
        debug!("found item {:?} in room {}", item, self.session.maze.player);
        self.session.player.inventory.add(item);
        effect.sound(Sound::ItemCollected);
        effect.message(Message::ItemCollected(item));
    }

    /// Go to previous room
    fn go_to_previous_room(&mut self, effect: &mut Effect) {
        assert!(self.session.is_previous_room_set());
        self.change_room(self.session.last_room.unwrap(), effect);
    }

    fn leave_maze(&mut self, effect: &mut Effect) {
        assert!(self.session.player_inventory().has(Item::MazeKey));
        effect.message(Message::LeaveMaze);
        effect.sound(Sound::LeaveMaze);
        self.session.leave_maze();
    }

    /// Use item in inventory
    fn use_item(&mut self, item: Item, effect: &mut Effect) {
        assert!(self.session.player.inventory.has(item));
        assert!(item.usable(self.session.player.state()));
        self.session.stats.items_used += 1;
        debug!("using item {:?}", item);
        match item {
            Item::Armor => self.wear_armor(effect),
            Item::Potion(potion) => self.drink_potion(potion, effect),
            Item::Sonar => self.use_sonar(effect),
            Item::Talisman => self.use_talisman(effect),
            Item::AlchemyBook | Item::PaintCan | Item::MazeKey => {}
        }
        if item.consumable() {
            debug!("item {:?} is consumable; decrease quantity", item);
            self.session.player.inventory.consume(item);
        }
        effect.message(Message::ItemUsed(item));
    }

    /// Wear armor
    fn wear_armor(&mut self, effect: &mut Effect) {
        debug!("increasing max HP by 1");
        self.session.player.incr_max_health(1);
        debug!(
            "armor worned; new max HP {}",
            self.session.player.max_health()
        );
        effect.sound(Sound::ArmorEquipped);
        effect.message(Message::ArmorEquipped);
    }

    /// Drink potion and apply effects
    fn drink_potion(&mut self, potion: Potion, effect: &mut Effect) {
        debug!("drinking potion: {:?}", potion);
        match potion {
            Potion::Chamomille if self.session.player.state() == PlayerState::Explore => {
                self.session.player.start_sleeping(3);
                self.session.player.heal(1);
                effect.message(Message::FallAsleep);
            }
            Potion::Chamomille => {
                self.session.player.heal(1);
            }
            Potion::DaemonsBlood => {
                self.session.player.decr_max_health(1);
            }
            Potion::DeadlyPoison => {
                self.session.player.damage(255);
            }
            Potion::FairyInABottle => {
                self.session.player.heal_max();
            }
            Potion::Mead => {
                self.session.player.heal(2);
            }
            Potion::RedPotion => {
                self.session.player.heal(5);
            }
            Potion::SnakePoison => {
                self.session.player.damage(2);
            }
            Potion::UnicornElixir => {
                self.session.player.incr_max_health(5);
                self.session.player.heal_max();
            }
            Potion::Vinegar => {
                self.session.player.damage(1);
            }
        }
        effect.sound(Sound::DrinkPotion);
        effect.message(Message::PotionDrunk(potion));
    }

    /// Use sonar to detect enemies and items in adjacent rooms
    fn use_sonar(&mut self, effect: &mut Effect) {
        let adjacent_rooms = self.session.maze.adjacent_rooms(self.session.maze.player);
        let mut revealed = false;
        for (node, room) in adjacent_rooms.into_iter() {
            if let Some(enemy) = room.enemy {
                debug!("revealed enemy {:?} in room {}", enemy, node);
                effect.message(Message::Reveal(node, Reveal::Enemy(enemy)));
                revealed = true;
            }
            if let Some(item) = room.item {
                debug!("revealed item {:?} in room {}", item, node);
                effect.message(Message::Reveal(node, Reveal::Item(item)));
                revealed = true;
            }
        }
        if !revealed {
            debug!("the sonar didn't reveal anything");
            effect.message(Message::RevealNothing);
        }
        effect.sound(Sound::Sonar);
    }

    /// Use talisman to kill enemy or vanish donmaze
    fn use_talisman(&mut self, effect: &mut Effect) {
        // get current room enemy and remove from it
        let enemy = self.session.maze.take_enemy().unwrap();
        debug!(
            "enemy {:?} removed from room {}",
            enemy, self.session.maze.player
        );
        // move don maze to a random room
        if matches!(enemy, Enemy::DonMaze) {
            let rooms_without_enemies: Vec<u32> = self
                .session
                .maze
                .rooms()
                .into_iter()
                .filter(|(_, room)| room.enemy.is_none())
                .map(|(node, _)| node)
                .collect();
            let mut rng = random::rng();
            let new_enemy_room = *random::choice(&mut rng, &rooms_without_enemies);
            debug!(
                "moved donmaze {:?} from room {} to {}",
                enemy, self.session.maze.player, new_enemy_room
            );
            let room = self.session.maze.room_mut(new_enemy_room).unwrap();
            room.enemy = Some(enemy);
        } else {
            self.session.stats.enemies_killed += 1;
        }
        // leave fight
        self.session.player.start_exploring();
        effect.sound(Sound::EnemyScream);
        effect.message(Message::EnemyVanished);
    }

    /// Try to escape (50% chance) to the first adjacent room, but not previous room.
    /// If there's no adjacent room, but previous room; escape to previous room
    fn escape(&mut self, effect: &mut Effect) {
        let mut rng = random::rng();
        if random::happens(&mut rng, ESCAPE_PROBABILITY) {
            // find room to escape to
            let adjacent_rooms_but_not_last: Vec<u32> = self
                .session
                .maze
                .adjacent_rooms(self.session.maze.player)
                .into_iter()
                .filter(|(id, _)| Some(id) != self.session.last_room.as_ref())
                .map(|(id, _)| id)
                .collect();
            let new_room = if adjacent_rooms_but_not_last.is_empty() {
                debug!("room is dead-end; escape to previous room then");
                self.session.last_room.unwrap()
            } else {
                *random::choice(&mut rng, &adjacent_rooms_but_not_last)
            };
            debug!("escape succeeded; new room {}", new_room);
            self.session.last_room = Some(self.session.maze.player);
            self.session.maze.player = new_room;
            self.session.stats.fights_escaped += 1;
            self.session.player.start_exploring();
            effect.message(Message::EscapeSucceeded(new_room));
            effect.sound(Sound::Rush);
        } else {
            debug!("escape failed");
            effect.message(Message::EscapeFailed);
        }
    }

    /// Fight enemy
    fn fight(&mut self, effect: &mut Effect) {
        // get current room enemy and remove from it
        let enemy = self.session.maze.fighting_enemy_mut().unwrap();
        let damage_dealt = if matches!(enemy, Enemy::DonMaze) {
            debug!("you can't deal with donmaze");
            0
        } else {
            let mut rng = random::rng();
            match rng.gen_range(0..100) {
                x if x < 30 => 1,
                x if x < 60 => 2,
                x if x < 80 => 3,
                x if x < 95 => 4,
                _ => 5,
            }
        };
        debug!("player dealt {} HP to {:?}", damage_dealt, enemy);
        enemy.damage(damage_dealt);
        self.session.stats.damage_inflicted += damage_dealt as u64;
        effect.message(Message::DamageDealt(damage_dealt));
        // check if enemy has died
        if enemy.health() == 0 {
            debug!("enemy defeated");
            // remove enemy from room
            let room = self
                .session
                .maze
                .room_mut(self.session.maze.player)
                .unwrap();
            room.enemy = None;
            self.session.stats.enemies_killed += 1;
            self.session.player.start_exploring();
            effect.message(Message::EnemyDefeated);
            effect.sound(Sound::PlayerAttack);
            effect.sound(Sound::EnemyScream);
        } else {
            debug!("new enemy HP: {}", enemy.health());
            effect.sound(Sound::PlayerAttack);
        }
    }

    /// Play sleep turn. Decrease sleep counter. Wake up if 0
    fn sleep(&mut self, effect: &mut Effect) {
        assert_eq!(self.session.player.state(), PlayerState::Asleep);
        self.session.player.decr_sleep_counter();
        self.session.stats.slept_for_turns += 1;
        if self.session.player.state() == PlayerState::Asleep {
            debug!("zzzzzz you're sleeping");
            effect.message(Message::Sleeping);
            effect.sound(Sound::Sleep);
        } else {
            debug!("wake up!");
            effect.message(Message::WakeUp);
            effect.sound(Sound::WakeUp);
        }
    }
}
