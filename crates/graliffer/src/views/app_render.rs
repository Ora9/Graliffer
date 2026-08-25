use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Margin, Rect, Spacing},
    text::ToSpan,
    widgets::{StatefulWidget, Widget},
};

use crate::{
    About, AboutView, App, AppState, ConsoleView, GridView, MenuGroup, MenuLine, MenuTitle,
    NumberPrefix, PaneBorder, Picker, PickerView, StackView, StackWidget, View,
};
use crate::{ConsoleWidget, GridWidget};

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

        ConsoleWidget::new().render(
            output_area.inner(Margin::from(1)),
            buf,
            &mut state.console_state,
        );

        StackWidget::new().render(
            stack_area.inner(Margin::from(1)),
            buf,
            &mut state.stack_state,
        );

        let input_mode = MenuLine::from_title(MenuTitle::Info(state.input_mode().formated()))
            .bottom()
            .right();

        let grid_pane_title = MenuTitle::NumberPrefix {
            title: "Grid".to_span(),
            prefix: NumberPrefix::Num1,
            focused: state.is_focused(GridView::view_id()),
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
            focused: state.is_focused(ConsoleView::view_id()),
        });

        let stack_menu_bar = MenuLine::from_title(MenuTitle::NumberPrefix {
            title: "Stack".to_span(),
            prefix: NumberPrefix::Num3,
            focused: state.is_focused(StackView::view_id()),
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

        if state.is_focused(AboutView::view_id()) {
            About.render(area, buf);
        }

        if state.is_focused(PickerView::view_id()) {
            Picker.render(area, buf, &mut state.command_picker_state);
        }
    }
}
