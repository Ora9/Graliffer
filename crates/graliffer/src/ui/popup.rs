use log::debug;
use ratatui::{
    buffer::Buffer,
    layout::{
        Constraint, Direction,
        Flex::{self, SpaceAround},
        Layout, Margin, Offset, Position, Rect, Size,
    },
    macros::horizontal,
    style::Style,
    symbols::border::{self, Set},
    text::{Line, Text},
    widgets::{Block, Borders, Clear, GraphType::Area, StatefulWidget, Widget},
};

use crate::ui::Align::Center;

#[derive(Debug, Clone, Copy)]
pub enum Align {
    Start,
    Center,
    End,
}

impl Align {
    pub const TOP: Self = Self::Start;
    pub const BOTTOM: Self = Self::End;
    pub const CENTER: Self = Self::Center;
    pub const LEFT: Self = Self::Start;
    pub const RIGHT: Self = Self::End;
}

#[derive(Debug, Clone, Copy)]
pub struct Align2 {
    x: Align,
    y: Align,
}

impl Align2 {
    pub const TOP_LEFT: Self = Self::new(Align::Start, Align::Start);
    pub const TOP_CENTER: Self = Self::new(Align::Start, Align::Center);
    pub const TOP_RIGHT: Self = Self::new(Align::Start, Align::End);

    pub const CENTER_LEFT: Self = Self::new(Align::Center, Align::Start);
    pub const CENTER_CENTER: Self = Self::new(Align::Center, Align::Center);
    pub const CENTER_RIGHT: Self = Self::new(Align::Center, Align::End);

    pub const BOTTOM_LEFT: Self = Self::new(Align::End, Align::Start);
    pub const BOTTOM_CENTER: Self = Self::new(Align::End, Align::Center);
    pub const BOTTOM_RIGHT: Self = Self::new(Align::End, Align::End);

    pub const fn new(x: Align, y: Align) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum PopupPosition {
    Edge { side: Align2, margin: Margin },
    At { position: Position, anchor: Align2 },
}

pub struct Popup<'content> {
    // pub body: Option<W>,
    pub size: Size,
    pub position: PopupPosition,

    pub title: Line<'content>,
    pub style: Style,

    pub borders: Borders,
    pub border_set: Set<'content>,
    pub border_style: Style,
}

impl<'content> Popup<'content> {
    // pub fn sized(size: Size) -> Self {}

    pub fn new(size: Size) -> Self {
        Self {
            size,
            position: PopupPosition::Edge {
                side: Align2::TOP_CENTER,
                margin: Margin {
                    horizontal: 5,
                    vertical: 3,
                },
            },

            title: Line::default(),
            style: Style::default(),

            borders: Borders::all(),
            border_set: border::ROUNDED,
            border_style: Style::default(),
        }
    }

    pub fn size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }

    pub fn position(mut self, position: PopupPosition) -> Self {
        self.position = position;
        self
    }

    pub fn title(mut self, title: impl Into<Line<'content>>) -> Self {
        self.title = title.into();
        self
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    pub fn borders(mut self, borders: Borders) -> Self {
        self.borders = borders;
        self
    }

    pub fn border_set(mut self, border_set: Set<'content>) -> Self {
        self.border_set = border_set;
        self
    }

    pub fn border_style(mut self, border_style: Style) -> Self {
        self.border_style = border_style;
        self
    }

    pub fn has_border(&self, borders: Borders) -> bool {
        self.borders.intersects(borders)
    }

    pub fn inner(&self, area: Rect) -> Rect {
        let mut inner = self.area(area);

        if self.has_border(Borders::LEFT) {
            inner.x = inner.x.saturating_add(1).min(inner.right());
            inner.width = inner.width.saturating_sub(1);
        }
        if self.has_border(Borders::TOP) {
            inner.y = inner.y.saturating_add(1).min(inner.bottom());
            inner.height = inner.height.saturating_sub(1);
        }
        if self.has_border(Borders::RIGHT) {
            inner.width = inner.width.saturating_sub(1);
        }
        if self.has_border(Borders::BOTTOM) {
            inner.height = inner.height.saturating_sub(1);
        }

        inner
    }

    pub fn area(&self, mut area: Rect) -> Rect {
        let border_height =
            u16::from(self.has_border(Borders::TOP)) + u16::from(self.has_border(Borders::BOTTOM));
        let border_width =
            u16::from(self.has_border(Borders::LEFT)) + u16::from(self.has_border(Borders::RIGHT));

        let width = self.size.width.saturating_add(border_width);
        let height = self.size.height.saturating_add(border_height);

        use PopupPosition::*;

        pub fn side_aligned(
            area: Rect,
            direction: Direction,
            align: Align,
            margin: u16,
            size: u16,
        ) -> Rect {
            let (constraints, flex, index) = match align {
                Align::Start => (
                    vec![Constraint::Length(margin), Constraint::Length(size)],
                    Flex::Start,
                    1,
                ),
                Align::Center => (vec![Constraint::Length(size)], Flex::Center, 0),
                Align::End => (
                    vec![Constraint::Length(size), Constraint::Length(margin)],
                    Flex::End,
                    0,
                ),
            };

            area.layout_vec(&Layout::new(direction, constraints).flex(flex))[index]
        }

        pub fn at_anchored(
            area: Rect,
            direction: Direction,
            anchor_align: Align,
            position: u16,
            size: u16,
        ) -> Rect {
            let constraints = match anchor_align {
                Align::Start => vec![Constraint::Length(position), Constraint::Length(size)],
                Align::Center => vec![
                    Constraint::Length(position.saturating_sub(size / 2)),
                    Constraint::Length(size),
                ],
                Align::End => vec![
                    Constraint::Length(position.saturating_sub(size)),
                    Constraint::Length(size),
                ],
            };

            area.layout_vec(&Layout::new(direction, constraints))[1]
        }

        match self.position {
            Edge { side, margin } => {
                area = side_aligned(
                    area,
                    Direction::Horizontal,
                    side.y,
                    margin.horizontal,
                    width,
                );
                area = side_aligned(area, Direction::Vertical, side.x, margin.vertical, height);
            }
            At { position, anchor } => {
                area = at_anchored(area, Direction::Horizontal, anchor.y, position.x, width);
                area = at_anchored(area, Direction::Vertical, anchor.x, position.y, height);
            }
        };

        area
    }
}

impl Widget for Popup<'_> {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let popup_inner = self.inner(area);

        Clear.render(popup_inner, buf);
        if self.borders != Borders::NONE {
            let block = Block::default()
                .borders(self.borders)
                .border_set(self.border_set)
                .border_style(self.border_style)
                .title(self.title)
                .style(self.style);

            let inner_area = block.inner(popup_inner);
            block.render(popup_inner, buf);
        };
    }
}
