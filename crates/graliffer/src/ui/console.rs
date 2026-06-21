use action::{Action, Revert, State};
use crossterm::event::MouseEvent;
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Layout, Margin, Rect},
    style::{Color, Style, Stylize},
    symbols::merge::MergeStrategy,
    text::{Line, Text},
    widgets::{Block, BorderType, Paragraph, StatefulWidget, Widget},
};

use tui_scrollbar::{
    GlyphSet, ScrollBar, ScrollBarArrows, ScrollBarInteraction, ScrollCommand, ScrollLengths,
    ScrollMetrics,
};

use crate::app;

#[derive(Debug)]
pub struct ConsoleState {
    show: bool,
    layouts: Option<ConsoleLayout>,

    content: Vec<String>,

    scroll_offset: usize,
    stick_to_bottom: bool,

    max_line_history: usize,

    scrollbar_interaction: ScrollBarInteraction,
}

impl ConsoleState {
    pub fn new(line_history: usize) -> Self {
        Self {
            show: true,
            layouts: None,

            content: Vec::new(),

            scroll_offset: 0,
            stick_to_bottom: true,

            max_line_history: line_history,

            scrollbar_interaction: ScrollBarInteraction::default(),
        }
    }

    pub fn layouts(&self) -> Option<ConsoleLayout> {
        self.layouts
    }

    pub fn max_line_history(&self) -> usize {
        self.max_line_history
    }
}

/// # Content
impl ConsoleState {
    pub fn lines(&self) -> usize {
        self.content.len()
    }

    pub fn content(&self) -> &Vec<String> {
        &self.content
    }

    pub fn set_content(&mut self, mut content: Vec<String>) {
        self.content = content;
        self.apply_max_history();

        self.scroll_to_bottom_if_sticky();
    }

    pub fn append_line(&mut self, line: String) {
        self.content.push(line);
        self.apply_max_history();

        self.scroll_to_bottom_if_sticky();
    }

    pub fn append_string(&mut self, string: String) {
        if let Some(mut last_line) = self.content.last_mut() {
            last_line.push_str(&string);
        } else {
            self.append_line(string);
        }
    }

    pub fn clear_content(&mut self) {
        self.set_content(Vec::default());
    }
}

/// # Inputs
impl ConsoleState {
    pub fn handle_mouse_event(&mut self, event: MouseEvent) {
        let Some(layouts) = self.layouts else {
            return;
        };

        let metrics = self.metrics_for_layouts(layouts);

        if let Some(command) = Console::build_vertical_scrollbar(metrics).handle_mouse_event(
            layouts.vertical_scrollbar_area,
            event,
            &mut self.scrollbar_interaction,
        ) {
            self.apply_command(command);
        }
    }

    pub fn apply_command(&mut self, command: ScrollCommand) {
        let ScrollCommand::SetOffset(offset) = command;

        self.stick_to_bottom = false;

        self.scroll_offset = offset
    }
}

/// # Layout
impl ConsoleState {
    fn update_layouts(&mut self, layouts: ConsoleLayout) {
        self.layouts = Some(layouts);
    }

    fn metrics_for_layouts(&self, layouts: ConsoleLayout) -> ScrollMetrics {
        let content_height = self.lines();
        let viewport_height = layouts.viewport_area.height;

        ScrollMetrics::new(
            ScrollLengths {
                content_len: content_height,
                viewport_len: viewport_height as usize,
            },
            self.scroll_offset,
            viewport_height,
        )
    }

    fn content_area_height(&self) -> Option<usize> {
        self.layouts
            .and_then(|layouts| Some(layouts.viewport_area.height as usize))
    }

    fn apply_max_history(&mut self) {
        self.content
            .drain(..(self.content.len().saturating_sub(self.max_line_history)));
    }
}

/// # Scrolling
impl ConsoleState {
    pub fn need_scroll(&self) -> bool {
        match self.content_area_height() {
            Some(content_area_height) => self.lines() > content_area_height,
            None => false,
        }
    }

    /// Is `0` when self.need_scroll() is `false` (when content does not exceed the container)
    pub fn max_scroll(&self) -> usize {
        self.lines()
            .saturating_sub(self.content_area_height().unwrap_or(0))
    }

    pub fn at_top(&self) -> bool {
        self.scroll_offset == 0
    }

    pub fn at_bottom(&self) -> bool {
        self.scroll_offset == self.max_scroll()
    }

    pub fn scroll_to_bottom(&mut self) {
        if self.need_scroll() {
            self.scroll_offset = self.max_scroll();
        }
    }

    pub fn scroll_to_top(&mut self) {
        self.scroll_offset = 0;
    }

    pub fn scroll_to_bottom_if_sticky(&mut self) {
        if self.stick_to_bottom {
            self.scroll_to_bottom();
        }
    }

    /// Scroll to bottom, and stay at the bottom (continue scrolling on new content)
    pub fn stick_to_bottom(&mut self) {
        self.stick_to_bottom = true;
        self.scroll_to_bottom();
    }

    pub fn scroll_down_by(&mut self, lines: usize) {
        if self.need_scroll() {
            self.scroll_offset = self
                .scroll_offset
                .saturating_add(lines)
                .min(self.max_scroll());
        }
    }

    pub fn scroll_up_by(&mut self, lines: usize) {
        if self.need_scroll() {
            self.scroll_offset = self.scroll_offset.saturating_sub(lines).max(0);
        }
    }

    pub fn scroll_by(&mut self, lines: isize) {
        match lines.signum() {
            1 => {
                self.scroll_up_by(lines.unsigned_abs());
            }
            -1 => {
                self.scroll_down_by(lines.unsigned_abs());
            }
            0 | _ => {}
        }
    }

    pub fn scroll_page_up(&mut self) {
        if let Some(viewport_height) = self.content_area_height() {
            self.scroll_up_by(viewport_height);
        }
    }

    pub fn scroll_page_down(&mut self) {
        if let Some(viewport_height) = self.content_area_height() {
            self.scroll_down_by(viewport_height);
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ConsoleLayout {
    viewport_area: Rect,
    vertical_scrollbar_area: Rect,
}

impl ConsoleLayout {
    pub fn viewport_area(&self) -> Rect {
        self.viewport_area
    }

    pub fn vertical_scrollbar_area(&self) -> Rect {
        self.vertical_scrollbar_area
    }
}

#[derive(Debug)]
pub struct Console;

impl Console {
    pub fn new() -> Self {
        Console
    }

    fn build_vertical_scrollbar(metrics: ScrollMetrics) -> ScrollBar {
        let lengths = ScrollLengths {
            content_len: metrics.content_len(),
            viewport_len: metrics.viewport_len(),
        };

        let glyph_set = GlyphSet {
            arrow_vertical_start: '↑',
            arrow_vertical_end: '↓',
            ..Default::default()
        };

        ScrollBar::vertical(lengths)
            .track_style(Style::new().bg(Color::Reset))
            .arrow_style(Style::new().bg(Color::Reset))
            .thumb_style(Style::new().bg(Color::Reset))
            .glyph_set(glyph_set)
            .arrows(ScrollBarArrows::Both)
            .offset(metrics.offset())
    }
}

impl StatefulWidget for Console {
    type State = ConsoleState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let [viewport_area, vertical_scrollbar_area] = area.layout(&Layout::horizontal([
            Constraint::Fill(1),
            Constraint::Length(1),
        ]));

        let layouts = ConsoleLayout {
            viewport_area,
            vertical_scrollbar_area,
        };
        let metrics = state.metrics_for_layouts(layouts);

        state.update_layouts(layouts);
        state.scroll_to_bottom_if_sticky();

        let rendered_lines = state
            .content
            .iter()
            .skip(state.scroll_offset)
            .take(viewport_area.height as usize)
            .cloned();

        let paragraph = Paragraph::new(Text::from(
            rendered_lines
                .reduce(|acc, item| format!("{acc}\n{item}"))
                .unwrap_or("".to_string()),
        ));

        paragraph.render(viewport_area, buf);

        if state.need_scroll() {
            let lengths = ScrollLengths {
                content_len: metrics.content_len(),
                viewport_len: metrics.viewport_len(),
            };

            let scrollbar = Self::build_vertical_scrollbar(metrics);

            scrollbar.render(vertical_scrollbar_area, buf);
        }
    }
}

#[derive(Debug, Clone)]
pub enum ConsoleAction {
    ScrollUp,
    ScrollDown,
    ScrollPageUp,
    ScrollPageDown,
    ScrollTop,
    ScrollBottom,
    ScrollBy(isize),
    Clear,
}

impl Action for ConsoleAction {}

impl State for ConsoleState {
    type Action = ConsoleAction;
    type Error = eyre::Error;

    fn act(&mut self, action: &Self::Action) -> Result<Revert, Self::Error> {
        use ConsoleAction::*;

        match action {
            ScrollUp => {
                self.scroll_up_by(1);
                Ok(Revert::None)
            }
            ScrollDown => {
                self.scroll_down_by(1);
                Ok(Revert::None)
            }
            ScrollPageUp => {
                self.scroll_page_up();
                Ok(Revert::None)
            }
            ScrollPageDown => {
                self.scroll_page_down();
                Ok(Revert::None)
            }
            ScrollTop => {
                self.scroll_to_top();
                Ok(Revert::None)
            }
            ScrollBottom => {
                self.stick_to_bottom();
                Ok(Revert::None)
            }
            ScrollBy(isize) => {
                self.scroll_by(*isize);
                Ok(Revert::None)
            }
            Clear => {
                self.clear_content();
                Ok(Revert::None)
            }
        }
    }
}
