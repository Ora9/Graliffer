use ratatui::{
    Frame,
    buffer::Buffer,
    layout::{Alignment, Constraint, Direction, Layout, Margin, Rect, Spacing},
    style::{Color, Style, Stylize},
    symbols::merge::MergeStrategy,
    text::{Line, Span, ToSpan},
    widgets::{Block, BorderType, Borders, Paragraph, StatefulWidget, Widget},
};

use crate::app::{App, FocusHandle, Focusable};

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
        &Layout::vertical(vec![Constraint::Fill(1), Constraint::Percentage(25)])
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

    let input_mode = MenuLine::from_title(MenuTitle::Info(app.input_mode.formated()))
        .bottom()
        .right();

    let grid_pane_title = MenuTitle::NumberPrefix {
        title: "Grid".to_span(),
        prefix: NumberPrefix::Num1,
        focused: app.focused(Focusable::Grid),
    };

    let file_title = MenuTitle::Inline {
        title: "Files".to_span(),
        highlight_char: "F".to_string(),
        focused: false,
    };

    let edit_title = MenuTitle::Inline {
        title: "Edit".to_span(),
        highlight_char: "E".to_string(),
        focused: false,
    };

    let main_menu_bar = MenuGroup::default()
        .push_title(file_title)
        .push_title(edit_title);

    let grid_menu_bar = MenuLine::default()
        .push_title_in_new_group(grid_pane_title)
        .push_group(main_menu_bar);

    let console_menu_bar = MenuLine::from_title(MenuTitle::NumberPrefix {
        title: "Console".to_span(),
        // highlight: "o".to_string(),
        prefix: NumberPrefix::Num2,
        focused: app.focused(Focusable::Console),
    });

    let stack_menu_bar = MenuLine::from_title(MenuTitle::NumberPrefix {
        title: "Stack".to_span(),
        prefix: NumberPrefix::Num3,
        focused: app.focused(Focusable::Stack),
    });

    PaneBorder::new()
        .add_menu_line(grid_menu_bar)
        .render(grid_area, frame.buffer_mut());

    PaneBorder::new()
        .add_menu_line(console_menu_bar)
        .add_menu_line(input_mode)
        .render(output_area, frame.buffer_mut());

    PaneBorder::new()
        .add_menu_line(stack_menu_bar)
        .render(stack_area, frame.buffer_mut());
}
