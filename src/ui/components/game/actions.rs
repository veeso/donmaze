//! # Actions

use super::{GameMsg, Msg};
use crate::game::session::{Action, ExploreAction, FightAction, Session};
use crate::utils::room_resolver::{self, Direction as MazeDirection};

use tui_realm_stdlib::List;
use tuirealm::command::{Cmd, Direction, Position};
use tuirealm::event::{Key, KeyEvent, KeyModifiers};
use tuirealm::props::{Alignment, BorderType, Borders, Color, TextSpan};
use tuirealm::{Component, Event, MockComponent, NoUserEvent, State, StateValue};

#[derive(MockComponent)]
pub struct AvailableActions {
    component: List,
    actions: Vec<Action>,
}

impl AvailableActions {
    pub fn new(session: &Session) -> Self {
        let actions = session.available_actions();
        let rows = actions
            .iter()
            .map(|x| vec![TextSpan::from(Self::action_name(x, session))])
            .collect();
        Self {
            component: List::default()
                .borders(
                    Borders::default()
                        .color(Color::Reset)
                        .modifiers(BorderType::Double),
                )
                .highlighted_str("➤	 ")
                .rewind(true)
                .scroll(true)
                .step(4)
                .title("What are you going to do?", Alignment::Left)
                .rows(rows),
            actions,
        }
    }

    fn action_name(action: &Action, session: &Session) -> &'static str {
        match action {
            Action::Die => "Game over",
            Action::UseItem(_) => panic!("ACCESS VIOLATION"),
            Action::Explore(ExploreAction::ChangeRoom(room)) => {
                Self::room_direction(*room, session)
            }
            Action::Explore(ExploreAction::CollectItem) => "Gather item",
            Action::Explore(ExploreAction::GoToPreviousRoom) => "Go back",
            Action::Explore(ExploreAction::LeaveMaze) => "Leave the labyrinth",
            Action::Fight(FightAction::Escape) => "Escape fight",
            Action::Fight(FightAction::Fight) => "Fight",
            Action::SaveGame => "Save game",
            Action::Sleep => "Sleep",
        }
    }

    fn room_direction(room: u32, session: &Session) -> &'static str {
        match room_resolver::resolve_room_direction(room, session) {
            MazeDirection::Ahead => "Go ahead",
            MazeDirection::Left => "Go left",
            MazeDirection::Right => "Go right",
        }
    }

    fn selected_action(&self) -> Option<Action> {
        match self.state() {
            State::One(StateValue::Usize(idx)) => Some(self.actions[idx]),
            _ => None,
        }
    }
}

impl Component<Msg, NoUserEvent> for AvailableActions {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        match ev {
            Event::Keyboard(KeyEvent {
                code: Key::Down, ..
            }) => {
                self.perform(Cmd::Move(Direction::Down));
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent { code: Key::Up, .. }) => {
                self.perform(Cmd::Move(Direction::Up));
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent {
                code: Key::PageDown,
                ..
            }) => {
                self.perform(Cmd::Scroll(Direction::Down));
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent {
                code: Key::PageUp, ..
            }) => {
                self.perform(Cmd::Scroll(Direction::Up));
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent {
                code: Key::Home, ..
            }) => {
                self.perform(Cmd::GoTo(Position::Begin));
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent { code: Key::End, .. }) => {
                self.perform(Cmd::GoTo(Position::End));
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent {
                code: Key::Enter, ..
            }) => self
                .selected_action()
                .map(|x| Msg::Game(GameMsg::ActionSelected(x))),
            Event::Keyboard(KeyEvent {
                code: Key::Char('e'),
                modifiers: KeyModifiers::NONE,
            }) => Some(Msg::Game(GameMsg::ShowInventory)),
            Event::Keyboard(KeyEvent {
                code: Key::Char('s'),
                modifiers: KeyModifiers::NONE,
            }) => Some(Msg::Game(GameMsg::ShowSaveFileName)),
            Event::Keyboard(KeyEvent { code: Key::Esc, .. }) => {
                Some(Msg::Game(GameMsg::ShowQuitPopup))
            }
            _ => None,
        }
    }
}
