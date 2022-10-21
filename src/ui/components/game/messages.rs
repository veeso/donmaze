//! # Messages
//!
//! Available messages

use crate::game::session::{Message, Reveal};
use crate::game::Session;
use crate::utils::room_resolver::{self, Direction as MazeDirection};

use super::Msg;

use tui_realm_stdlib::List;
use tuirealm::props::{BorderType, Borders, Color, TextSpan};
use tuirealm::{Component, Event, MockComponent, NoUserEvent};

#[derive(MockComponent)]
pub struct Messages {
    component: List,
}

impl Messages {
    pub fn new(messages: &[Message], session: &Session) -> Self {
        let line = messages.len().saturating_sub(1);
        Self {
            component: List::default()
                .borders(
                    Borders::default()
                        .color(Color::Reset)
                        .modifiers(BorderType::Double),
                )
                .foreground(Color::Reset)
                .scroll(false)
                .selected_line(line)
                .rows(Self::messages(messages, session)),
        }
    }

    fn messages(messages: &[Message], session: &Session) -> Vec<Vec<TextSpan>> {
        messages
            .iter()
            .map(|x| vec![TextSpan::from(Self::message(x, session))])
            .collect()
    }

    fn message(message: &Message, session: &Session) -> String {
        let has_alchemy_book = session
            .player_inventory()
            .has(crate::game::entity::Item::AlchemyBook);
        match message {
            Message::ArmorEquipped => "armor equipped; HP increased by 1".to_string(),
            Message::DamageDealt(hp) => format!("dealt {} HP to enemy", hp),
            Message::DamageSuffered(hp, true) => {
                format!("Critical hit! The enemy dealt {} HP to you.", hp)
            }
            Message::DamageSuffered(hp, false) => format!("The enemy dealt {} HP to you.", hp),
            Message::EnemyApproaching(enemy) => format!("{} entered the room", enemy.name()),
            Message::EnemyDefeated => "Enemy defeated".to_string(),
            Message::EnemyVanished => "The enemy vanished...".to_string(),
            Message::EscapeFailed => "You failed to escape the enemy".to_string(),
            Message::EscapeSucceeded(room) => {
                format!(
                    "You escaped in the room {}",
                    Self::room_direction(*room, session)
                )
            }
            Message::FallAsleep => "You suddenly feel sleepy and you fall asleep".to_string(),
            Message::ItemCollected(item) => format!("You found a {}", item.name(has_alchemy_book)),
            Message::ItemUsed(item) => item.effect().to_string(),
            Message::LeaveMaze => "You left the maze".to_string(),
            Message::PlayerDead => "You died".to_string(),
            Message::PotionDrunk(potion) => format!("You drunk the potion: {}", potion.effect()),
            Message::Reveal(room, Reveal::Enemy(enemy)) => format!(
                "the sonar revealed a {} in the room {}",
                enemy.name(),
                Self::room_direction(*room, session)
            ),
            Message::Reveal(room, Reveal::Item(item)) => format!(
                "the sonar revealed a {} in the room {}",
                item.name(has_alchemy_book),
                Self::room_direction(*room, session)
            ),
            Message::Sleeping => "You're still sleeping like a baby...".to_string(),
            Message::WakeUp => "You finally woke up".to_string(),
        }
    }

    fn room_direction(room: u32, session: &Session) -> &'static str {
        match room_resolver::resolve_room_direction(room, session) {
            MazeDirection::Ahead => "In front of you",
            MazeDirection::Left => "On your left",
            MazeDirection::Right => "On your right",
        }
    }
}

impl Component<Msg, NoUserEvent> for Messages {
    fn on(&mut self, _ev: Event<NoUserEvent>) -> Option<Msg> {
        None
    }
}
