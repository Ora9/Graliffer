use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Margin, Rect, Spacing},
    style::Style,
    widgets::{StatefulWidget, Widget},
};

use crate::{
    About, AboutView, App, AppWidget, GralifferAction, MenuGroup, MenuLine, MenuTitle, Pane,
    Picker, PickerView, StackWidget, View, ViewId, input::InputMode,
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
            let output_height = (area.height / 3).min(10);
            let inspect_width = (area.width / 3).min(30);

            let [top_area, output_area] = area.layout(
                &Layout::vertical(vec![Constraint::Fill(1), Constraint::Length(output_height)])
                    .spacing(Spacing::Overlap(1)),
            );

            let [grid_area, inspect_area] = top_area.layout(
                &Layout::horizontal(vec![Constraint::Fill(1), Constraint::Length(inspect_width)])
                    .spacing(Spacing::Overlap(1)),
            );

            (grid_area, inspect_area, output_area)
        };

        // TODO: restore this :
        // - calling it app_menu_bar instead of main_menu_bar
        // - use a keymap aware menu display

        // let edit_title = MenuTitle::Inline {
        //     title: "Edit".to_span(),
        //     highlight_char: "E".to_string(),
        //     focused: false,
        // };

        // let main_menu_bar = MenuGroup::default()
        //     .push_title(file_title.clone())
        //     .push_title(edit_title);

        //         let grid_pane_title = MenuTitle::NumberPrefix {
        //             style: Style::new(),
        //             title: "Grid".to_string(),
        //             prefix: NumberPrefix::Num1,
        //             highlighted: app.focusing(GridView::view_id()),
        //         };
        //
        //         let grid_menu_bar = MenuLine::default().push_title_in_new_group(grid_pane_title);
        //         // .push_group(main_menu_bar);

        let file_title = MenuTitle::Inline {
            style: Style::new(),
            title: "File".to_string(),
            highlight_char: "F".to_string(),
            highlighted: false,
        };

        let edit_title = MenuTitle::Inline {
            style: Style::new(),
            title: "Edit".to_string(),
            highlight_char: "E".to_string(),
            highlighted: false,
        };

        let main_menu_bar = MenuGroup::default()
            .push_title(file_title)
            .push_title(edit_title);

        let grid_views = vec![(ViewId::Grid, GralifferAction::FocusGrid.into())];
        Pane::new(grid_views, &app.context)
            .add_menu_line(MenuLine::from_group(main_menu_bar))
            .render(grid_area, buf);

        GridWidget::new().render(grid_area.inner(Margin::from(1)), buf, &mut app.grid_view);

        let inspect_views = vec![(ViewId::Stack, GralifferAction::FocusStack.into())];
        Pane::new(inspect_views, &app.context)
            // .add_menu_line(stack_menu_bar)
            .render(inspect_area, buf);

        StackWidget::new().render(
            inspect_area.inner(Margin::from(1)),
            buf,
            &mut app.stack_view,
        );

        let input_mode = MenuLine::from_title(MenuTitle::Info {
            highlighted: false,
            title: app.input_mode().to_string().to_ascii_uppercase(),
            style: match app.input_mode() {
                InputMode::Command => Style::new().red(),
                InputMode::Insert => Style::new(),
            },
        })
        .bottom()
        .right();

        let output_views = vec![(ViewId::Console, GralifferAction::FocusConsole.into())];
        Pane::new(output_views, &app.context)
            // .add_title_menu(app.context)
            .add_menu_line(input_mode)
            .render(output_area, buf);

        ConsoleWidget::new().render(
            output_area.inner(Margin::from(1)),
            buf,
            &mut app.console_view,
        );

        if app.focusing(AboutView::view_id()) {
            About.render(area, buf);
        }

        if app.focusing(PickerView::view_id()) {
            Picker.render(area, buf, &mut app.command_picker_state);
        }
    }
}
