//! # Action replay

use crate::game::entity::{Item, Potion};

use super::{Action, Effect, ExploreAction, FightAction, Message, PlayerState, Session, Sound};

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
            Action::Sleep => self.sleep(effect),
        }
    }

    /// Player dies. Game over
    fn die(&mut self, effect: &mut Effect) {
        debug!("ho-ho. You're dead.");
        effect.message(Message::PlayerDead);
        effect.sound(Sound::PlayerDead);
    }

    /// Play explore action
    fn play_explore_action(&mut self, action: ExploreAction, effect: &mut Effect) {
        assert_eq!(self.session.player.state(), PlayerState::Explore);
        debug!("playing explore action: {:?}", action);
        match action {
            ExploreAction::ChangeRoom(room) => self.change_room(room, effect),
            ExploreAction::GoToPreviousRoom => self.go_to_previous_room(effect),
            ExploreAction::UseItem(item) => self.use_item(item, effect),
        }
    }

    fn change_room(&mut self, room: u32, effect: &mut Effect) {
        assert!(self.session.maze.room_adjacent(room));
        debug!(
            "going from room {} to room {}",
            self.session.maze.player, room
        );
        self.session.last_room = Some(self.session.maze.player);
        self.session.maze.player = room;
        effect.sound(Sound::Steps);
    }

    fn go_to_previous_room(&mut self, effect: &mut Effect) {
        assert!(self.session.is_previous_room_set());
        self.change_room(self.session.last_room.unwrap(), effect);
    }

    fn use_item(&mut self, item: Item, effect: &mut Effect) {
        assert!(self.session.player.inventory.has(item));
        assert!(item.usable(self.session.player.state()));
        debug!("using item {:?}", item);
        match item {
            Item::Armor => self.wear_armor(effect),
            Item::Potion(potion) => self.drink_potion(potion, effect),
            Item::Sonar => self.use_sonar(effect),
            Item::Talisman => self.use_talisman(effect),
            Item::AlchemyBook | Item::MazeKey => {}
        }
        if item.consumable() {
            debug!("item {:?} is consumable; decrease quantity", item);
            self.session.player.inventory.consume(item);
        }
        effect.message(Message::ItemUsed(item));
    }

    fn wear_armor(&mut self, effect: &mut Effect) {
        debug!("increasing max HP by 1");
        self.session.player.incr_max_health(1);
        debug!(
            "armor worned; new max HP {}",
            self.session.player.max_health()
        );
    }

    fn drink_potion(&mut self, potion: Potion, effect: &mut Effect) {
        debug!("drinking potion: {:?}", potion);
        match potion {
            Potion::Chamomille if self.session.player.state() == PlayerState::Explore => {
                self.session.player.start_sleeping(3);
                self.session.player.heal(1);
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
            Potion::Mead => {
                self.session.player.heal(1);
            }
            Potion::RedPotion => {
                self.session.player.heal(3);
            }
            Potion::SnakePoison => {
                self.session.player.damage(2);
            }
            Potion::UnicornElixir => {
                self.session.player.incr_max_health(2);
            }
            Potion::Vinegar => {
                self.session.player.damage(1);
            }
        }
        effect.sound(Sound::DrinkPotion);
    }

    fn use_sonar(&mut self, effect: &mut Effect) {
        todo!()
    }

    fn use_talisman(&mut self, effect: &mut Effect) {
        todo!()
    }

    /// Play fight action
    fn play_fight_action(&mut self, action: FightAction, effect: &mut Effect) {
        assert_eq!(self.session.player.state(), PlayerState::Fight);
        debug!("playing fight action: {:?}", action);
        todo!()
    }

    /// Play sleep turn. Decrease sleep counter. Wake up if 0
    fn sleep(&mut self, effect: &mut Effect) {
        assert_eq!(self.session.player.state(), PlayerState::Asleep);
        self.session.player.decr_sleep_counter();
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
