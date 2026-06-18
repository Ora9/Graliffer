use ratatui::{
    Frame,
    buffer::Buffer,
    layout::{Alignment, Constraint, Direction, Layout, Margin, Rect, Spacing},
    style::{Color, Style, Stylize},
    symbols::merge::MergeStrategy,
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph, StatefulWidget, Widget},
};

use crate::app::App;

mod menu;
pub use menu::*;

mod pane;
pub use pane::*;

mod console;
pub use console::*;

mod grid;
pub use grid::*;

pub fn render(app: &mut App, frame: &mut Frame) {
    let [top_area, output_area] = frame.area().layout(
        &Layout::vertical(vec![Constraint::Fill(1), Constraint::Percentage(20)])
            .spacing(Spacing::Overlap(1)),
    );

    let [grid_area, stack_area] = top_area.layout(
        &Layout::horizontal(vec![Constraint::Fill(1), Constraint::Percentage(20)])
            .spacing(Spacing::Overlap(1)),
    );

    GridWidget::new().render(
        grid_area.inner(Margin::from(1)),
        frame.buffer_mut(),
        &mut app.grid_state,
    );

    Console::new().render(
        output_area.inner(Margin::from(1)),
        frame.buffer_mut(),
        &mut app.console_state,
    );

    PaneBorder::new(MenuTitle::NumberPrefix {
        title: "Grid".to_string(),
        prefix: NumberPrefix::Num0,
        focused: app.focused_pane.grid(),
    })
    .render(grid_area, frame.buffer_mut());

    PaneBorder::new(MenuTitle::NumberPrefix {
        title: "Console".to_string(),
        // highlight: "o".to_string(),
        prefix: NumberPrefix::Num2,
        focused: app.focused_pane.console(),
    })
    .render(output_area, frame.buffer_mut());

    PaneBorder::new(MenuTitle::NumberPrefix {
        title: "Stack".to_string(),
        prefix: NumberPrefix::Num2,
        focused: app.focused_pane.stack(),
    })
    .render(stack_area, frame.buffer_mut());
}
