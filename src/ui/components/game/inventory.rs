//! # Inventory
//!
//! player's inventory

use super::{GameMsg, Msg};
use crate::game::entity::Item;
use crate::game::Session;

use tuirealm::command::{Cmd, CmdResult, Direction, Position};
use tuirealm::event::{Key, KeyEvent};
use tuirealm::props::{
    Alignment, AttrValue, Attribute, BorderType, Borders, Color, Props, Style, TextModifiers,
};
use tuirealm::tui::{
    layout::{Constraint, Corner, Direction as LayoutDirection, Layout},
    text::{Span, Spans},
    widgets::{List as TuiList, ListItem, ListState, Paragraph as TuiParagraph, Wrap},
};
use tuirealm::{Component, Event, MockComponent, NoUserEvent, State};

struct ItemState {
    item: Item,
    description: String,
    quantity: u8,
    name: String,
    usable: bool,
}

pub struct Inventory {
    inventory: Vec<ItemState>, // list of items and their quantity
    props: Props,
    item: usize,
    this_item_desc: String,
}

impl Inventory {
    pub fn new(session: &Session) -> Self {
        let has_alchemy_book = session.player_inventory().has(Item::AlchemyBook);
        let mut inventory: Vec<ItemState> = session
            .player_inventory()
            .items()
            .map(|(i, q)| ItemState {
                item: *i,
                description: i.description(has_alchemy_book).to_string(),
                name: i.name(has_alchemy_book).to_string(),
                quantity: *q,
                usable: i.usable(session.player().state()),
            })
            .collect();
        // sort by name
        inventory.sort_by_key(|x| x.name.clone());
        Self {
            inventory,
            item: 0,
            props: Props::default(),
            this_item_desc: String::default(),
        }
    }

    fn incr_item(&mut self, rewind: bool) {
        // Check if index is at last element
        if self.item + 1 < self.inventory.len() {
            self.item += 1;
        } else if rewind {
            self.item = 0;
        }
    }

    fn decr_item(&mut self, rewind: bool) {
        // Check if index is bigger than 0
        if self.item > 0 {
            self.item -= 1;
        } else if rewind && self.inventory.len() > 0 {
            self.item = self.inventory.len() - 1;
        }
    }

    fn item_at_first(&mut self) {
        self.item = 0;
    }

    fn item_at_last(&mut self) {
        if self.inventory.len() > 0 {
            self.item = self.inventory.len() - 1;
        } else {
            self.item = 0;
        }
    }

    /// Calculate the max step ahead to scroll list
    fn calc_max_step_ahead(&self, max: usize) -> usize {
        let remaining: usize = match self.inventory.len() {
            0 => 0,
            len => len - 1 - self.item,
        };
        if remaining > max {
            max
        } else {
            remaining
        }
    }

    /// Calculate the max step ahead to scroll list
    fn calc_max_step_behind(&self, max: usize) -> usize {
        if self.item > max {
            max
        } else {
            self.item
        }
    }
}

impl MockComponent for Inventory {
    fn view(&mut self, frame: &mut tuirealm::Frame, area: tuirealm::tui::layout::Rect) {
        let focus = self
            .props
            .get_or(Attribute::Focus, AttrValue::Flag(false))
            .unwrap_flag();
        let list_block = tui_realm_stdlib::utils::get_block(
            Borders::default()
                .color(Color::LightRed)
                .modifiers(BorderType::Double),
            Some((String::from("Inventory"), Alignment::Center)),
            focus,
            None,
        );

        // Make list entries
        let list_items: Vec<ListItem> = self
            .inventory
            .iter()
            .map(|item| {
                let item_style = if item.usable {
                    Style::default()
                } else {
                    Style::default().add_modifier(TextModifiers::CROSSED_OUT)
                };
                let cols = Span::styled(format!("{} ({}x)", item.name, item.quantity), item_style);
                ListItem::new(Spans::from(cols))
            })
            .collect();

        let chunks = Layout::default()
            .direction(LayoutDirection::Vertical)
            .vertical_margin(1)
            .constraints([Constraint::Percentage(80), Constraint::Percentage(20)].as_ref())
            .split(area);

        // Make list
        let list = TuiList::new(list_items)
            .block(list_block)
            .style(Style::default().fg(Color::LightRed))
            .start_corner(Corner::TopLeft)
            .highlight_style(Style::default().fg(Color::LightRed))
            .highlight_symbol("➤	 ");
        let mut state: ListState = ListState::default();
        state.select(Some(self.item));
        frame.render_stateful_widget(list, chunks[0], &mut state);
        // render description
        if let Some(item) = self.inventory.get(self.item) {
            self.this_item_desc = item.description.clone();
            let desc_block = tui_realm_stdlib::utils::get_block(
                Borders::default()
                    .color(Color::LightRed)
                    .modifiers(BorderType::Double),
                Some((String::from("Description"), Alignment::Left)),
                focus,
                None,
            );
            frame.render_widget(
                TuiParagraph::new(self.this_item_desc.as_str())
                    .block(desc_block)
                    .style(Style::default())
                    .wrap(Wrap { trim: true }),
                chunks[1],
            );
        }
    }

    fn query(&self, attr: Attribute) -> Option<AttrValue> {
        self.props.get(attr)
    }

    fn attr(&mut self, attr: Attribute, value: AttrValue) {
        self.props.set(attr, value);
    }

    fn state(&self) -> State {
        State::None
    }

    fn perform(&mut self, cmd: Cmd) -> CmdResult {
        match cmd {
            Cmd::Move(Direction::Down) => {
                let prev = self.item;
                self.incr_item(true);
                if prev != self.item {
                    CmdResult::Changed(self.state())
                } else {
                    CmdResult::None
                }
            }
            Cmd::Move(Direction::Up) => {
                let prev = self.item;
                self.decr_item(true);
                if prev != self.item {
                    CmdResult::Changed(self.state())
                } else {
                    CmdResult::None
                }
            }
            Cmd::Scroll(Direction::Down) => {
                let prev = self.item;
                let step = self
                    .props
                    .get_or(Attribute::ScrollStep, AttrValue::Length(8))
                    .unwrap_length();
                let step: usize = self.calc_max_step_ahead(step);
                (0..step).for_each(|_| self.incr_item(false));
                if prev != self.item {
                    CmdResult::Changed(self.state())
                } else {
                    CmdResult::None
                }
            }
            Cmd::Scroll(Direction::Up) => {
                let prev = self.item;
                let step = self
                    .props
                    .get_or(Attribute::ScrollStep, AttrValue::Length(8))
                    .unwrap_length();
                let step: usize = self.calc_max_step_behind(step);
                (0..step).for_each(|_| self.decr_item(false));
                if prev != self.item {
                    CmdResult::Changed(self.state())
                } else {
                    CmdResult::None
                }
            }
            Cmd::GoTo(Position::Begin) => {
                let prev = self.item;
                self.item_at_first();
                if prev != self.item {
                    CmdResult::Changed(self.state())
                } else {
                    CmdResult::None
                }
            }
            Cmd::GoTo(Position::End) => {
                let prev = self.item;
                self.item_at_last();
                if prev != self.item {
                    CmdResult::Changed(self.state())
                } else {
                    CmdResult::None
                }
            }
            _ => CmdResult::None,
        }
    }
}

impl Component<Msg, NoUserEvent> for Inventory {
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
            Event::Keyboard(KeyEvent { code: Key::Esc, .. }) => {
                Some(Msg::Game(GameMsg::CloseInventory))
            }
            Event::Keyboard(KeyEvent {
                code: Key::Enter, ..
            }) => {
                // get item and return use item if it is usable
                if let Some(item) = self.inventory.get(self.item) {
                    if item.usable {
                        Some(Msg::Game(GameMsg::UseItem(item.item)))
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}
