use log::debug;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Position, Rect, Size},
    style::Style,
    symbols::border::{self, Set},
    text::{Line, Text},
    widgets::{Block, Borders, Clear, GraphType::Area, StatefulWidget, Widget},
};

pub struct Popup<'content, W> {
    pub body: W,

    pub size: Size,
    pub position: Option<Position>,

    pub title: Line<'content>,
    pub style: Style,

    pub borders: Borders,
    pub border_set: Set<'content>,
    pub border_style: Style,
}

impl<'content, W> Popup<'content, W> {
    pub fn new(body: W, size: Size) -> Self {
        Self {
            body,

            size,
            position: None,

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

    pub fn position(mut self, position: Position) -> Self {
        self.position = Some(position);
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
}

impl<W: Widget> Widget for Popup<'_, W> {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let popup_area = self.popup_area(area);

        let block = Block::default()
            .borders(self.borders)
            .border_set(self.border_set)
            .border_style(self.border_style)
            .title(self.title)
            .style(self.style);

        let inner_area = block.inner(popup_area);

        debug!("{inner_area}, {popup_area}");

        Clear.render(popup_area, buf);
        block.render(popup_area, buf);
        self.body.render(inner_area, buf)
    }
}

impl<W> Popup<'_, W> {
    fn popup_area(&self, area: Rect) -> Rect {
        let has_top = self.borders.intersects(Borders::TOP);
        let has_bottom = self.borders.intersects(Borders::BOTTOM);
        let has_left = self.borders.intersects(Borders::LEFT);
        let has_right = self.borders.intersects(Borders::RIGHT);

        let border_height = u16::from(has_top) + u16::from(has_bottom);
        let border_width = u16::from(has_left) + u16::from(has_right);

        let width = self.size.width.saturating_add(border_width);
        let height = self.size.height.saturating_add(border_height);

        if let Some(position) = self.position {
            Rect::from((position, self.size))
        } else {
            area.centered(Constraint::Length(width), Constraint::Length(height))
        }
    }
}
