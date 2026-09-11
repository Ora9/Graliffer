use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Margin, Rect, Spacing},
    text::ToSpan,
    widgets::{StatefulWidget, Widget},
};

use crate::{
    About, AboutView, App, AppWidget, ConsoleView, GridView, MenuLine, MenuTitle, NumberPrefix,
    Pane, Picker, PickerView, StackView, StackWidget, View,
};
use crate::{ConsoleWidget, GridWidget};

impl StatefulWidget for AppWidget {
    type State = App;

    fn render(self, area: Rect, buf: &mut Buffer, app: &mut Self::State) {
        // Graliffer's UI is divided in three main area :
        // - Grid area: the main and largest pane, used to display the graliffer grid, contains an
        //   visual editor
        // - Inspector area: placed at the right of the grid area, contains every inspection
        //   utilities like head/stack view, breakpoints ..
        // - Output area: placed at the bottom, where most output are viewed (console, graphics,
        //   audio infos..)
        let (grid_area, inspect_area, output_area) = {
            let [top_area, output_area] = area.layout(
                &Layout::vertical(vec![Constraint::Fill(1), Constraint::Percentage(25)])
                    .spacing(Spacing::Overlap(1)),
            );

            let [grid_area, inspect_area] = top_area.layout(
                &Layout::horizontal(vec![Constraint::Fill(1), Constraint::Percentage(20)])
                    .spacing(Spacing::Overlap(1)),
            );

            (grid_area, inspect_area, output_area)
        };

        // TODO: restore this :
        // - calling it app_menu_bar instead of main_menu_bar
        // - use a keymap aware menu display
        //
        // let file_title = MenuTitle::Inline {
        //     title: "Files".to_span(),
        //     highlight_char: "F".to_string(),
        //     focused: false,
        // };

        // let edit_title = MenuTitle::Inline {
        //     title: "Edit".to_span(),
        //     highlight_char: "E".to_string(),
        //     focused: false,
        // };

        // let main_menu_bar = MenuGroup::default()
        //     .push_title(file_title.clone())
        //     .push_title(edit_title);

        let grid_pane_title = MenuTitle::NumberPrefix {
            title: "Grid".to_span(),
            prefix: NumberPrefix::Num1,
            focused: app.focusing(GridView::view_id()),
        };

        let grid_menu_bar = MenuLine::default().push_title_in_new_group(grid_pane_title);
        // .push_group(main_menu_bar);

        Pane::new()
            .add_menu_line(grid_menu_bar)
            .render(grid_area, buf);

        GridWidget::new().render(grid_area.inner(Margin::from(1)), buf, &mut app.grid_view);

        let console_menu_bar = MenuLine::from_title(MenuTitle::NumberPrefix {
            title: "Console".to_span(),
            prefix: NumberPrefix::Num2,
            focused: app.focusing(ConsoleView::view_id()),
        });

        let input_mode = MenuLine::from_title(MenuTitle::Info(app.input_mode().formated()))
            .bottom()
            .right();

        Pane::new()
            .add_menu_line(console_menu_bar)
            .add_menu_line(input_mode)
            .render(output_area, buf);

        ConsoleWidget::new().render(
            output_area.inner(Margin::from(1)),
            buf,
            &mut app.console_view,
        );

        let stack_menu_bar = MenuLine::from_title(MenuTitle::NumberPrefix {
            title: "Stack".to_span(),
            prefix: NumberPrefix::Num3,
            focused: app.focusing(StackView::view_id()),
        });
        Pane::new()
            .add_menu_line(stack_menu_bar)
            .render(inspect_area, buf);

        StackWidget::new().render(
            inspect_area.inner(Margin::from(1)),
            buf,
            &mut app.stack_view,
        );

        if app.focusing(AboutView::view_id()) {
            About.render(area, buf);
        }

        if app.focusing(PickerView::view_id()) {
            Picker.render(area, buf, &mut app.command_picker_state);
        }
    }
}
