use rand::rngs::ThreadRng;

///! # Cpu
///
/// This module expose the CPU player
use super::{Effect, PlayerState, Session};
use crate::audio::Sound;
use crate::game::session::Message;
use crate::game::{entity::Enemy, Hp};
use crate::utils::random;

/// Cpu plays the CPU actions
pub struct Cpu<'a> {
    session: &'a mut Session,
}

impl<'a> Cpu<'a> {
    /// Instantiate a new Cpu
    pub fn new(session: &'a mut Session) -> Self {
        Self { session }
    }

    /// Play turn for cpu
    pub fn play(&mut self, effect: &mut Effect) {
        self.fight_player(effect);
        self.move_exploring_enemies(effect);
    }

    /// Take enemy in the same room of the player
    /// and deal damage to player
    fn fight_player(&mut self, effect: &mut Effect) {
        let enemy = match self
            .session
            .maze
            .room_mut(self.session.maze.player)
            .unwrap()
            .enemy
            .as_mut()
        {
            None => return,
            Some(e) => e,
        };
        // if player state is NOT FIGHTING; it means the player has just joined; so no damage has to be dealt
        if self.session.player.state() != PlayerState::Fight {
            self.start_player_fight(effect);
            return;
        }
        // calculate damage to deal, based on enemy type
        let (damage_to_deal, critical_hit) = Self::deal_damage(enemy);
        debug!(
            "dealt {} HP to player (critical? {})",
            damage_to_deal, critical_hit
        );
        self.session.player.damage(damage_to_deal);
        // report damage
        effect.message(Message::DamageSuffered(damage_to_deal, critical_hit));
        effect.sound(Sound::EnemyAttack);
        // check if player is dead
        if self.session.player.is_dead() {
            debug!("player is dead. Game over...");
            effect.sound(Sound::PlayerDead);
            effect.message(Message::PlayerDead);
        }
    }

    /// Calculate damage to deal based on random and enemy type
    /// The bool defines whether is critical hit
    fn deal_damage(enemy: &Enemy) -> (Hp, bool) {
        let critical_hit = random::happens(&mut random::rng(), 10);
        let base_attack: Hp = match enemy {
            Enemy::Daemon(_) => 1,
            Enemy::DonMaze => 255,
            Enemy::Shadow(_) => 2,
        };
        let damage_dealt = if critical_hit {
            base_attack.saturating_add(1)
        } else {
            base_attack
        };
        (damage_dealt, critical_hit)
    }

    /// Move enemies which ARE not in fight
    fn move_exploring_enemies(&mut self, effect: &mut Effect) {
        let rooms_with_exploring_enemies: Vec<u32> = self
            .session
            .maze
            .rooms()
            .into_iter()
            .filter(|(_, room)| room.enemy.is_some())
            .filter(|(id, _)| *id != self.session.maze.player)
            .map(|(id, _)| id)
            .collect();
        debug!(
            "there are still {} exploring enemies",
            rooms_with_exploring_enemies.len()
        );
        // iter rooms
        let mut rng = random::rng();
        for room in rooms_with_exploring_enemies.into_iter() {
            self.move_enemy_at(&mut rng, room, effect);
        }
    }

    /// Move enemy located at room u32
    fn move_enemy_at(&mut self, rng: &mut ThreadRng, room: u32, effect: &mut Effect) {
        // get adjacent rooms to enemy room
        let adjacent_rooms_without_enemies: Vec<u32> = self
            .session
            .maze
            .adjacent_rooms(room)
            .into_iter()
            .filter(|(_, room)| room.enemy.is_none())
            .map(|(id, _)| id)
            .collect();
        debug!(
            "enemy at {} has {} adjacent rooms without enemies",
            room,
            adjacent_rooms_without_enemies.len()
        );
        if adjacent_rooms_without_enemies.is_empty() {
            debug!(
                "there's no adjacent room without enemies; so the enemy at {} won't be moved",
                room
            );
        } else {
            // choose room to move the enemy to
            let new_enemy_room = *random::choice(rng, &adjacent_rooms_without_enemies);
            debug!("moved enemy at {} to {}", room, new_enemy_room);
            let enemy = self
                .session
                .maze
                .room_mut(room)
                .unwrap()
                .enemy
                .take()
                .unwrap();
            let new_room = self.session.maze.room_mut(new_enemy_room).unwrap();
            new_room.enemy = Some(enemy);
            // check if enemy has joined player
            if new_enemy_room == self.session.maze.player {
                debug!("enemy {:?} has joined the room of the player", enemy);
                self.start_player_fight(effect);
            }
        }
        todo!();
    }

    /// Start player fight
    fn start_player_fight(&mut self, effect: &mut Effect) {
        effect.message(Message::EnemyApproaching);
        effect.sound(Sound::EnemyApproaching);
        // put player into fight
        self.session.player.start_fighting();
    }
}
