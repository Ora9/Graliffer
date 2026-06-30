use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Margin, Rect, Spacing},
    text::ToSpan,
    widgets::{StatefulWidget, Widget},
};

mod menu;
pub use menu::*;

mod pane;
pub use pane::*;

mod popup;
pub use popup::*;

mod about;
pub use about::*;

mod picker;
pub use picker::*;

use crate::{App, AppState, Console, Focusable, GridWidget};

impl StatefulWidget for App {
    type State = AppState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let [top_area, output_area] = area.layout(
            &Layout::vertical(vec![Constraint::Fill(1), Constraint::Percentage(25)])
                .spacing(Spacing::Overlap(1)),
        );

        let [grid_area, stack_area] = top_area.layout(
            &Layout::horizontal(vec![Constraint::Fill(1), Constraint::Percentage(20)])
                .spacing(Spacing::Overlap(1)),
        );

        GridWidget::new().render(grid_area.inner(Margin::from(1)), buf, &mut state.grid_state);

        Console::new().render(
            output_area.inner(Margin::from(1)),
            buf,
            &mut state.console_state,
        );

        let input_mode = MenuLine::from_title(MenuTitle::Info(state.input_mode().formated()))
            .bottom()
            .right();

        let grid_pane_title = MenuTitle::NumberPrefix {
            title: "Grid".to_span(),
            prefix: NumberPrefix::Num1,
            focused: state.is_focused(Focusable::Grid),
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
            .push_title(file_title.clone())
            .push_title(edit_title);

        let grid_menu_bar = MenuLine::default()
            .push_title_in_new_group(grid_pane_title)
            .push_group(main_menu_bar);

        let console_menu_bar = MenuLine::from_title(MenuTitle::NumberPrefix {
            title: "Console".to_span(),
            prefix: NumberPrefix::Num2,
            focused: state.is_focused(Focusable::Console),
        });

        let stack_menu_bar = MenuLine::from_title(MenuTitle::NumberPrefix {
            title: "Stack".to_span(),
            prefix: NumberPrefix::Num3,
            focused: state.is_focused(Focusable::Stack),
        });

        PaneBorder::new()
            .add_menu_line(grid_menu_bar)
            .render(grid_area, buf);

        PaneBorder::new()
            .add_menu_line(console_menu_bar)
            .add_menu_line(input_mode)
            .render(output_area, buf);

        PaneBorder::new()
            .add_menu_line(stack_menu_bar)
            .render(stack_area, buf);

        if state.show_about {
            About.render(area, buf);
        }

        if state.show_command_picker {
            Picker.render(area, buf, &mut state.command_picker_state);
        }
    }
}
