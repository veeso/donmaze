//! # Buttons

use super::{MenuMsg, Msg};

use tuirealm::props::{
    Alignment, AttrValue, Attribute, BorderType, Borders, Color, PropPayload, PropValue, Props,
    Style, TextModifiers, TextSpan,
};
use tuirealm::tui::{
    layout::Rect,
    text::{Span, Spans},
    widgets::{Paragraph as TuiParagraph, Wrap},
};
use tuirealm::{
    command::{Cmd, CmdResult},
    event::{Key, KeyEvent},
    Component, Event, Frame, MockComponent, NoUserEvent, State,
};

#[derive(MockComponent)]
pub struct NewGame {
    component: Button,
}

impl Default for NewGame {
    fn default() -> Self {
        Self {
            component: Button::default()
                .alignment(Alignment::Center)
                .foreground(Color::LightRed)
                .borders(
                    Borders::default()
                        .color(Color::LightRed)
                        .modifiers(BorderType::Double),
                )
                .text(&[TextSpan::from("New game")])
                .wrap(true),
        }
    }
}

impl Component<Msg, NoUserEvent> for NewGame {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        match ev {
            Event::Keyboard(KeyEvent {
                code: Key::Enter, ..
            }) => Some(Msg::Menu(MenuMsg::NewGame)),
            Event::Keyboard(KeyEvent { code: Key::Up, .. }) => Some(Msg::Menu(MenuMsg::ActiveExit)),
            Event::Keyboard(KeyEvent {
                code: Key::Right, ..
            }) => Some(Msg::Menu(MenuMsg::ActiveSeed)),
            Event::Keyboard(KeyEvent {
                code: Key::Down, ..
            }) => Some(Msg::Menu(MenuMsg::ActiveLoadGame)),
            _ => None,
        }
    }
}

#[derive(MockComponent)]
pub struct LoadGame {
    component: Button,
}

impl Default for LoadGame {
    fn default() -> Self {
        Self {
            component: Button::default()
                .alignment(Alignment::Center)
                .foreground(Color::Blue)
                .borders(
                    Borders::default()
                        .color(Color::Blue)
                        .modifiers(BorderType::Double),
                )
                .text(&[TextSpan::from("Load game")])
                .wrap(true),
        }
    }
}

impl Component<Msg, NoUserEvent> for LoadGame {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        match ev {
            Event::Keyboard(KeyEvent {
                code: Key::Enter, ..
            }) => Some(Msg::Menu(MenuMsg::LoadGame)),
            Event::Keyboard(KeyEvent { code: Key::Up, .. }) => {
                Some(Msg::Menu(MenuMsg::ActiveNewGame))
            }
            Event::Keyboard(KeyEvent {
                code: Key::Down, ..
            }) => Some(Msg::Menu(MenuMsg::ActiveExit)),
            _ => None,
        }
    }
}

#[derive(MockComponent)]
pub struct Exit {
    component: Button,
}

impl Default for Exit {
    fn default() -> Self {
        Self {
            component: Button::default()
                .alignment(Alignment::Center)
                .foreground(Color::Red)
                .borders(
                    Borders::default()
                        .color(Color::Red)
                        .modifiers(BorderType::Double),
                )
                .text(&[TextSpan::from("Quit")])
                .wrap(true),
        }
    }
}

impl Component<Msg, NoUserEvent> for Exit {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        match ev {
            Event::Keyboard(KeyEvent {
                code: Key::Enter, ..
            }) => Some(Msg::Menu(MenuMsg::Quit)),
            Event::Keyboard(KeyEvent { code: Key::Up, .. }) => {
                Some(Msg::Menu(MenuMsg::ActiveLoadGame))
            }
            Event::Keyboard(KeyEvent {
                code: Key::Down, ..
            }) => Some(Msg::Menu(MenuMsg::ActiveNewGame)),
            _ => None,
        }
    }
}

#[derive(Default)]
pub struct Button {
    props: Props,
}

impl Button {
    pub fn foreground(mut self, fg: Color) -> Self {
        self.attr(Attribute::Foreground, AttrValue::Color(fg));
        self
    }

    pub fn borders(mut self, b: Borders) -> Self {
        self.attr(Attribute::Borders, AttrValue::Borders(b));
        self
    }

    pub fn alignment(mut self, a: Alignment) -> Self {
        self.attr(Attribute::Alignment, AttrValue::Alignment(a));
        self
    }

    pub fn text(mut self, s: &[TextSpan]) -> Self {
        self.attr(
            Attribute::Text,
            AttrValue::Payload(PropPayload::Vec(
                s.iter().cloned().map(PropValue::TextSpan).collect(),
            )),
        );
        self
    }

    pub fn wrap(mut self, wrap: bool) -> Self {
        self.attr(Attribute::TextWrap, AttrValue::Flag(wrap));
        self
    }
}

impl MockComponent for Button {
    fn view(&mut self, render: &mut Frame, area: Rect) {
        // Make a Span
        if self.props.get_or(Attribute::Display, AttrValue::Flag(true)) == AttrValue::Flag(true) {
            // Make text items
            let focus = self
                .props
                .get_or(Attribute::Focus, AttrValue::Flag(false))
                .unwrap_flag();
            let text: Vec<Spans> = match self.props.get(Attribute::Text).map(|x| x.unwrap_payload())
            {
                Some(PropPayload::Vec(spans)) => spans
                    .iter()
                    .cloned()
                    .map(|x| x.unwrap_text_span())
                    .map(|x| {
                        let (fg, bg, modifiers) =
                            tui_realm_stdlib::utils::use_or_default_styles(&self.props, &x);
                        Spans::from(vec![Span::styled(
                            x.content,
                            if focus {
                                Style::default().add_modifier(modifiers).fg(fg).bg(bg)
                            } else {
                                Style::default()
                            },
                        )])
                    })
                    .collect(),
                _ => Vec::new(),
            };
            // Text properties
            let alignment: Alignment = self
                .props
                .get_or(Attribute::Alignment, AttrValue::Alignment(Alignment::Left))
                .unwrap_alignment();
            // Wrap
            let trim = self
                .props
                .get_or(Attribute::TextWrap, AttrValue::Flag(false))
                .unwrap_flag();
            let foreground = self
                .props
                .get_or(Attribute::Foreground, AttrValue::Color(Color::Reset))
                .unwrap_color();
            let background = self
                .props
                .get_or(Attribute::Background, AttrValue::Color(Color::Reset))
                .unwrap_color();
            let modifiers = self
                .props
                .get_or(
                    Attribute::TextProps,
                    AttrValue::TextModifiers(TextModifiers::empty()),
                )
                .unwrap_text_modifiers();
            let borders = self
                .props
                .get_or(Attribute::Borders, AttrValue::Borders(Borders::default()))
                .unwrap_borders();
            let block_style = match focus {
                true => Style::default()
                    .fg(foreground)
                    .bg(background)
                    .add_modifier(modifiers),
                false => Style::default(),
            };
            let title = self.props.get(Attribute::Title).map(|x| x.unwrap_title());
            let div = tui_realm_stdlib::utils::get_block(borders, title, focus, None);
            render.render_widget(
                TuiParagraph::new(text)
                    .block(div)
                    .style(block_style)
                    .alignment(alignment)
                    .wrap(Wrap { trim }),
                area,
            );
        }
    }

    fn query(&self, attr: Attribute) -> Option<AttrValue> {
        self.props.get(attr)
    }

    fn attr(&mut self, attr: Attribute, value: AttrValue) {
        self.props.set(attr, value)
    }

    fn state(&self) -> State {
        State::None
    }

    fn perform(&mut self, _cmd: Cmd) -> CmdResult {
        CmdResult::None
    }
}
