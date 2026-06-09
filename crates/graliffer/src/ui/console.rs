use std::iter;

use rand::seq::{IteratorRandom, SliceRandom};
use ratatui::{
    Frame,
    buffer::Buffer,
    layout::{Alignment, Constraint, Layout, Margin, Rect, Size},
    style::Stylize,
    symbols::merge::MergeStrategy,
    text::{Line, Text},
    widgets::{
        Block, BorderType, GraphType::Area, Paragraph, ScrollbarState, StatefulWidget, Widget,
    },
};
// use tui_scrollview::{ScrollView, ScrollViewState, ScrollbarVisibility};

use tui_scrollbar::{ScrollBar, ScrollBarInteraction, ScrollLengths, ScrollMetrics};

use crate::app;

#[derive(Debug)]
pub struct ConsoleState {
    content: Vec<String>,
    scrollbar_interaction: ScrollBarInteraction,
    scroll_offset: usize,
}

impl ConsoleState {
    pub fn new() -> Self {
        let phrase = "Lorem ipsum dolor sit amet, consectetur adipiscing elit.".to_string();

        let mut rng = rand::rng();

        let shuffler = || {
            let mut phrase = phrase.split(" ").collect::<Vec<&str>>();
            phrase.shuffle(&mut rng);
            phrase.join(" ").to_string()
        };
        let content = iter::repeat_with(shuffler)
            .take(100)
            .collect::<Vec<String>>();

        Self {
            content,
            scroll_offset: 0,
            scrollbar_interaction: ScrollBarInteraction::default(),
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

        let scroll_lengths = ScrollLengths {
            content_len: state.content.len(),
            viewport_len: content_area.height as usize,
        };

        // let scroll_metrics =
        //     ScrollMetrics::new(scroll_lengths, state.scroll_offset, content_area.height);

        let scrollbar = ScrollBar::vertical(scroll_lengths).offset(state.scroll_offset);

        // .render_widget(&vertical, vertical_bar);
        // let vertical_lengths = ScrollLengths {
        //     content_len: v_metrics.content_len(),
        //     viewport_len: v_metrics.viewport_len(),
        // };
        // let vertical = ScrollBar::vertical(vertical_lengths)
        //     .arrows(ScrollBarArrows::Both)
        //     .offset(self.vertical_offset)
        //     .scroll_step(SUBCELL)
        //     .track_style(track_style)
        //     .thumb_style(thumb_style)
        //     .arrow_style(arrow_style);

        let content_lines = state
            .content
            .iter()
            .skip(state.scroll_offset)
            .take(content_area.height as usize)
            .cloned();
        // .clone();
        // .collect::<String>();

        // let paragraph = Paragraph::new(content_lines.lines().count().to_string());
        // let paragraph = Paragraph::new(content_lines.to_string());

        let paragraph = Paragraph::new(Text::from(
            content_lines
                .reduce(|acc, item| format!("{acc}\n{item}"))
                .unwrap_or("".to_string()),
        ));

        // let mut scroll_view = ScrollView::new(Size {
        //     width: inner_area.width,
        //     height: content_lines as u16,
        // })
        // .horizontal_scrollbar_visibility(ScrollbarVisibility::Never);

        // scroll_view.render(inner_area, buf, &mut state.scrollview_state);

        paragraph.render(content_area, buf);
        scrollbar.render(scrollbar_area, buf);
        block.render(area, buf);
    }
}
//     pub fn render(&self, frame: &mut Frame, area: Rect) {

//         // frame.render_widget(
//         //     Paragraph::new(content).scroll((
//         //         vertical.get_position() as u16,
//         //         horizontal.get_position() as u16,
//         //     )),
//         //     area,
//         // );
//     }
// }
