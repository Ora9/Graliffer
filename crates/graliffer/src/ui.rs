use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Spacing},
    style::{Color, Style, Stylize},
    symbols::merge::MergeStrategy,
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph, Widget},
};

use crate::app::App;

mod console;
pub use console::*;

pub fn render(app: &mut App, frame: &mut Frame) {
    // let vertical = Layout::vertical(vec![Constraint::Fill(1), Constraint::Percentage(20)]);
    // let horizontal = Layout::horizontal(vec![Constraint::Fill(1), Constraint::Percentage(20)]);

    let [editor, output] = frame.area().layout(
        &Layout::vertical(vec![Constraint::Fill(1), Constraint::Percentage(20)])
            .spacing(Spacing::Overlap(1)),
    );
    let [grid, stack] = editor.layout(
        &Layout::horizontal(vec![Constraint::Fill(1), Constraint::Percentage(20)])
            .spacing(Spacing::Overlap(1)),
    );

    frame.render_widget(
        Block::bordered()
            .border_type(BorderType::Rounded)
            .title(Line::from(vec![
                "┤".into(),
                "¹".blue().into(),
                "Grid".into(),
                "├".into(),
            ]))
            .title(
                Line::from(vec![
                    "┤".into(),
                    "²".blue().into(),
                    "Stack".into(),
                    "├".into(),
                ])
                .alignment(Alignment::Center),
            )
            .title(
                Line::from(vec![
                    "┤".into(),
                    "²".blue().into(),
                    "Stack".into(),
                    "├".into(),
                ])
                .alignment(Alignment::Center),
            ),
        // .title_bottom("prout"),
        // Paragraph::new("Grid").block(Block::new().borders(Borders::all())),
        grid,
    );

    frame.render_widget(
        Block::bordered()
            .border_type(BorderType::Rounded)
            .merge_borders(MergeStrategy::Fuzzy)
            .title(Line::from(vec![
                "┤".into(),
                "²".blue().into(),
                "Stack".into(),
                "├".into(),
            ])),
        // Paragraph::new("Stack").block(Block::new().borders(Borders::all())),
        stack,
    );

    frame.render_stateful_widget(Console::new(), output, &mut app.console_state);
    // app.console.render(frame, output);

    // frame.render_widget(
    //     Block::bordered()
    //         .border_type(BorderType::Rounded)
    //         .merge_borders(MergeStrategy::Fuzzy)
    //         .title(Line::from(vec![
    //             "┤".into(),
    //             "³".blue().into(),
    //             "Console".into(),
    //             "├".into(),
    //         ]))
    //         // .title_bottom(Line::from("prout").alignment(Alignment::Right)),
    //         .title_bottom(
    //             Line::from(vec![
    //                 "┤".into(),
    //                 // "²".blue().into(),
    //                 "COMMAND".bold().red().into(),
    //                 "├".into(),
    //             ])
    //             .alignment(Alignment::Right),
    //         ),
    //     // Paragraph::new("Console").block(Block::new().borders(Borders::all())),
    //     output,
    // );

    // frame.render_widget(
    //     Paragraph::new("outer 0").block(Block::new().borders(Borders::ALL)),
    //     outer_layout[0],
    // );
    // frame.render_widget(
    //     Paragraph::new("inner 0").block(Block::new().borders(Borders::ALL)),
    //     inner_layout[0],
    // );
    // frame.render_widget(
    //     Paragraph::new("inner 1").block(Block::new().borders(Borders::ALL)),
    //     inner_layout[1],
    // );

    // frame.render_widget(
    //     Paragraph::new(format!(
    //         "
    //     Press `Esc`, `Ctrl-C` or `q` to stop running.\n\
    //     Press `j` and `k` to increment and decrement the counter respectively.\n\
    //     Counter: {}
    //   ",
    //         app.should_run
    //     ))
    //     .block(
    //         Block::default()
    //             .title("Counter App")
    //             .title_alignment(Alignment::Center)
    //             .borders(Borders::ALL)
    //             .border_type(BorderType::Rounded),
    //     )
    //     .style(Style::default().fg(Color::Yellow))
    //     .alignment(Alignment::Center),
    //     frame.area(),
    // )
}
