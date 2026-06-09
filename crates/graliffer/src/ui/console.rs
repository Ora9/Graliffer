use std::{iter, ops::AddAssign};

use rand::seq::{IteratorRandom, SliceRandom};
use ratatui::{
    Frame,
    buffer::Buffer,
    layout::{Alignment, Constraint, Layout, Margin, Rect, Size},
    style::{Color, Style, Stylize},
    symbols::merge::MergeStrategy,
    text::{Line, Text},
    widgets::{
        Block, BorderType, GraphType::Area, Paragraph, ScrollbarState, StatefulWidget, Widget,
    },
};
// use tui_scrollview::{ScrollView, ScrollViewState, ScrollbarVisibility};

use tui_scrollbar::{
    GlyphSet, ScrollBar, ScrollBarArrows, ScrollBarInteraction, ScrollLengths, ScrollMetrics,
};

use crate::app;

#[derive(Debug)]
pub struct ConsoleState {
    max_line_history: usize,
    stick_to_bottom: bool,

    content: Vec<String>,

    // Scroll offset from the bottom
    scroll_offset: usize,
    content_area_height: Option<usize>,

    scrollbar_interaction: ScrollBarInteraction,
}

impl ConsoleState {
    pub fn new(line_history: usize) -> Self {
        let mut state = Self {
            max_line_history: line_history,
            stick_to_bottom: true,

            content: Vec::new(),

            scroll_offset: 0,
            content_area_height: None,

            scrollbar_interaction: ScrollBarInteraction::default(),
        };

        let mut rng = rand::rng();
        let phrase = "Lorem ipsum dolor sit amet, consectetur adipiscing elit.".to_string();

        let mut line_number = 0;

        let shuffler = || {
            let mut phrase = phrase.split(" ").collect::<Vec<&str>>();
            phrase.shuffle(&mut rng);

            line_number.add_assign(1);

            format!("{} {}", line_number, phrase.join(" "))
        };

        let content = iter::repeat_with(shuffler)
            .take(100)
            .collect::<Vec<String>>();

        state.set_content(content);

        state
    }

    fn apply_max_history(&mut self) {
        self.content
            .drain(..(self.content.len() - self.max_line_history));
    }

    pub fn need_scroll(&self) -> bool {
        match self.content_area_height {
            Some(content_area_height) => self.lines() > content_area_height,
            None => false,
        }
    }

    /// Is `0` when self.need_scroll() is `false` (when content does not exceed the container)
    pub fn max_scroll(&self) -> usize {
        self.lines()
            .saturating_sub(self.content_area_height.unwrap_or(0))
    }

    pub fn at_top(&self) -> bool {
        self.scroll_offset == 0
    }

    pub fn at_bottom(&self) -> bool {
        self.scroll_offset == self.max_scroll()
    }

    pub fn content(&self) -> &Vec<String> {
        &self.content
    }

    pub fn lines(&self) -> usize {
        self.content.len()
    }

    pub fn set_content(&mut self, mut content: Vec<String>) {
        self.content = content;
        self.apply_max_history();

        if self.stick_to_bottom {
            self.scroll_to_bottom();
        }
    }

    pub fn append_line(&mut self, line: String) {
        self.content.insert(0, line);
        self.apply_max_history();

        if self.stick_to_bottom {
            self.scroll_to_bottom();
        }
    }

    pub fn append_string(&mut self, string: String) {
        if let Some(mut last_line) = self.content.first_mut() {
            last_line.push_str(&string);
        } else {
            self.append_line(string);
        }
    }

    pub fn scroll_to_bottom(&mut self) {
        if self.need_scroll() {
            self.scroll_offset = self.max_scroll();
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
}

#[derive(Debug)]
pub struct Console;

impl Console {
    pub fn new() -> Self {
        Console
    }
}

impl StatefulWidget for Console {
    type State = ConsoleState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let block = Block::bordered()
            .border_type(BorderType::Rounded)
            .merge_borders(MergeStrategy::Fuzzy)
            .title(Line::from(vec![
                "┤".into(),
                "³".blue().into(),
                "Console".into(),
                "├".into(),
            ]))
            .title_bottom(
                Line::from(vec![
                    "┤".into(),
                    // "²".blue().into(),
                    "COMMAND".bold().red().into(),
                    "├".into(),
                ])
                .alignment(Alignment::Right),
            );

        let inner_area = area.inner(Margin::from(1));

        let [content_area, scrollbar_area] = inner_area.layout(&Layout::horizontal([
            Constraint::Fill(1),
            Constraint::Length(1),
        ]));

        let content_len = state.lines();
        let content_area_height = content_area.height as usize;

        // Update content_area_height based on the widget height
        state.content_area_height = Some(content_area_height);

        let content_lines = state
            .content
            .iter()
            .skip(state.scroll_offset)
            .take(content_area.height as usize)
            .cloned();

        let paragraph = Paragraph::new(Text::from(
            content_lines
                .reduce(|acc, item| format!("{acc}\n{item}"))
                .unwrap_or("".to_string()),
        ));

        paragraph.render(content_area, buf);
        block.render(area, buf);

        if content_len > content_area_height {
            let scroll_lengths = ScrollLengths {
                content_len,
                viewport_len: content_area_height,
            };

            let glyph_set = GlyphSet {
                arrow_vertical_start: '↑',
                arrow_vertical_end: '↓',
                ..Default::default()
            };

            let scrollbar = ScrollBar::vertical(scroll_lengths)
                .track_style(Style::new().bg(Color::Reset))
                .arrow_style(Style::new().bg(Color::Reset))
                .thumb_style(Style::new().bg(Color::Reset))
                .glyph_set(glyph_set)
                .arrows(ScrollBarArrows::Both)
                .offset(state.scroll_offset);

            scrollbar.render(scrollbar_area, buf);
        }
    }
}
