use log::debug;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Position, Rect, Size},
    style::Style,
    symbols::border::{self, Set},
    text::{Line, Text},
    widgets::{Block, Borders, Clear, GraphType::Area, StatefulWidget, Widget},
};

pub trait KnownSize {
    fn width(&self) -> usize;
    fn height(&self) -> usize;
}

impl KnownSize for Text<'_> {
    fn width(&self) -> usize {
        self.width()
    }

    fn height(&self) -> usize {
        self.height()
    }
}

impl KnownSize for &Text<'_> {
    fn width(&self) -> usize {
        Text::width(self)
    }

    fn height(&self) -> usize {
        Text::height(self)
    }
}

impl KnownSize for &str {
    fn width(&self) -> usize {
        Text::from(*self).width()
    }

    fn height(&self) -> usize {
        Text::from(*self).height()
    }
}

impl KnownSize for String {
    fn width(&self) -> usize {
        Text::from(self.as_str()).width()
    }

    fn height(&self) -> usize {
        Text::from(self.as_str()).height()
    }
}

pub struct Popup<'content, W> {
    pub body: W,
    pub size: Option<Size>,
    pub position: Option<Position>,
    pub title: Line<'content>,
    pub style: Style,
    pub borders: Borders,
    pub border_set: Set<'content>,
    pub border_style: Style,
}

impl<'content, W> Popup<'content, W> {
    pub fn new(body: W) -> Self {
        Self {
            body,
            position: None,
            size: None,
            title: Line::default(),
            style: Style::default(),
            borders: Borders::all(),
            border_set: border::ROUNDED,
            border_style: Style::default(),
        }
    }

    pub fn size(mut self, size: Size) -> Self {
        self.size = Some(size);
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

impl<W: Widget + KnownSize> Widget for Popup<'_, W> {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        // // let viewport_area = area;
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

impl<W: KnownSize> Popup<'_, W> {
    fn popup_area(&self, area: Rect) -> Rect {
        // if let Some(current) = state.area.take() {
        //     return current.clamp(area);
        // }

        let (width, height) = if let Some(size) = self.size {
            (size.width, size.height)
        } else {
            (
                u16::try_from(self.body.width()).unwrap_or(area.width),
                u16::try_from(self.body.height()).unwrap_or(area.height),
            )
        };

        let has_top = self.borders.intersects(Borders::TOP);
        let has_bottom = self.borders.intersects(Borders::BOTTOM);
        let has_left = self.borders.intersects(Borders::LEFT);
        let has_right = self.borders.intersects(Borders::RIGHT);

        let border_height = u16::from(has_top) + u16::from(has_bottom);
        let border_width = u16::from(has_left) + u16::from(has_right);

        let width = width.saturating_add(border_width);
        let height = height.saturating_add(border_height);

        if let Some(position) = self.position {
            Rect {
                x: position.x,
                y: position.y,
                width,
                height,
            }
        } else {
            area.centered(Constraint::Length(width), Constraint::Length(height))
        }
    }
}
