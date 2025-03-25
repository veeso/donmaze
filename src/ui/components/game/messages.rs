//! # Messages
//!
//! Available messages

use tui_realm_stdlib::List;
use tuirealm::props::{BorderType, Borders, Color, TextSpan};
use tuirealm::{Component, Event, MockComponent, NoUserEvent};

use super::Msg;
use crate::game::session::{Message, Reveal};
use crate::game::Session;
use crate::utils::room_resolver::{self, Direction as MazeDirection};

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
            Message::ArmorEquipped => "Armor equipped; HP increased by 1".to_string(),
            Message::DamageDealt(hp) => format!("Dealt {} HP to enemy", hp),
            Message::DamageSuffered(hp, true) => {
                format!("Critical hit! The enemy dealt {} HP to you.", hp)
            }
            Message::DamageSuffered(hp, false) => format!("The enemy dealt {} HP to you.", hp),
            Message::EnemyApproaching(enemy) => format!("{} entered the room", enemy.name()),
            Message::EnemyDefeated => "Enemy defeated".to_string(),
            Message::EnemyMissed => "Enemy missed".to_string(),
            Message::EnemyVanished => "The enemy vanished...".to_string(),
            Message::EscapeFailed => "You failed to escape the enemy".to_string(),
            Message::EscapeSucceeded(room) => {
                format!(
                    "You escaped in the room {}",
                    Self::room_direction(*room, session)
                )
            }
            Message::FallAsleep => "You suddenly feel sleepy and you fall asleep".to_string(),
            Message::GameSaved => "Game has been saved".to_string(),
            Message::ItemCollected(item) => format!("You found a {}", item.name(has_alchemy_book)),
            Message::ItemUsed(item) => format!(
                "You used {}: {}",
                item.name(has_alchemy_book),
                item.effect()
            ),
            Message::LeaveMaze => "You left the maze".to_string(),
            Message::PlayerDead => "You died".to_string(),
            Message::PotionDrunk(potion) => {
                format!("You drunk the {}: {}", potion.name(), potion.effect())
            }
            Message::Reveal(room, Reveal::Enemy(enemy)) => format!(
                "The sonar revealed a {} in the room {}",
                enemy.name(),
                Self::room_direction(*room, session)
            ),
            Message::Reveal(room, Reveal::Item(item)) => format!(
                "The sonar revealed a {} in the room {}",
                item.name(has_alchemy_book),
                Self::room_direction(*room, session)
            ),
            Message::RevealNothing => "The sonar didn't reveal anything.".to_string(),
            Message::RoomChanged(MazeDirection::Ahead) => {
                "You entered the room in front of you".to_string()
            }
            Message::RoomChanged(MazeDirection::Left) => {
                "You entered the room on your left".to_string()
            }
            Message::RoomChanged(MazeDirection::Right) => {
                "You entered the room on your right".to_string()
            }
            Message::Sleeping => "You're still sleeping like a baby...".to_string(),
            Message::WakeUp => "You finally woke up".to_string(),
        }
    }

    fn room_direction(room: u32, session: &Session) -> &'static str {
        match room_resolver::resolve_room_direction(room, session) {
            MazeDirection::Ahead => "in front of you",
            MazeDirection::Left => "on your left",
            MazeDirection::Right => "on your right",
        }
    }
}

impl Component<Msg, NoUserEvent> for Messages {
    fn on(&mut self, _ev: Event<NoUserEvent>) -> Option<Msg> {
        None
    }
}
