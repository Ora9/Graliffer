use ratatui::{
    Frame,
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::Stylize,
    symbols::merge::MergeStrategy,
    text::Line,
    widgets::{Block, BorderType, Paragraph, ScrollbarState, StatefulWidget},
};
use tui_scrollview::ScrollViewState;

pub struct ConsoleState {
    content: String, // scrollbar_state: ScrollbarState,
    scrollview_state: ScrollViewState,
}

#[derive(Debug)]
pub struct Console {}

impl Console {
    pub fn new() -> Self {
        Self {
            content: String::default(),
            scrollview_state: ScrollViewState::new(),
        }
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
            // .title_bottom(Line::from("prout").alignment(Alignment::Right)),
            .title_bottom(
                Line::from(vec![
                    "┤".into(),
                    // "²".blue().into(),
                    "COMMAND".bold().red().into(),
                    "├".into(),
                ])
                .alignment(Alignment::Right),
            );

        // let paragraph = Paragraph::new(self.content.clone()).sco;

        // frame.render_widget(block, area);
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
