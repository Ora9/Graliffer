use std::{env, vec};

use log::debug;
use ratatui::{
    buffer::Buffer,
    layout::{self, Constraint, Layout, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Span, Text, ToLine, ToSpan},
    widgets::Widget,
};

pub struct About;

impl About {
    pub const WIDTH: u16 = 90;
    pub const HEIGHT: u16 = 16;
}

impl Widget for About {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        // let graliffer = Line::from("Graliffer");

        let title = vec![
            r#"  ________             .__  .__  _____  _____                  _____)        _____)"#.to_line(),
            r#" /  _____/___________  |  | |__|/ ____\/ ____\___________     /_____/       /_____/"#.to_line(),
            r#"/   \  __\   __ \__  \ |  | |  |   ___\   __\/ __ \_  __ \    /    \        /    \ "#.to_line(),
            r#"\    \_\  \  | \// __ \|  |_|  ||  |   |  | \  ___/|  | \/   (  ()  )      (  ()  )"#.to_line(),
            r#" \______  /__|  (____  /____/__||__|   |__|  \___  |__|       \____/ ______ \____/ "#.to_line(),
            r#"        \/           \/                          \/                 /_____/        "#.to_line(),
        ];

        let desc = "An exotic programming language using a 2d grid holding code and data";

        let version = env!("CARGO_PKG_VERSION");
        let repo = env!("CARGO_PKG_REPOSITORY");
        let license = env!("CARGO_PKG_LICENSE");

        let [_, title_area, _, desc_area, misc_area] = area.layout(&Layout::vertical(vec![
            Constraint::Length(2),
            Constraint::Length(6),
            Constraint::Length(1),
            Constraint::Length(3),
            Constraint::Length(3),
        ]));

        let title = title
            .into_iter()
            .map(|line| line.bold().fg(Color::LightMagenta))
            .collect::<Vec<Line>>();

        Text::from(title).centered().render(title_area, buf);
        Text::raw(desc).centered().render(desc_area, buf);
        Text::from(vec![
            format!("{} • {} license", version, license).to_line(),
            repo.to_line(),
        ])
        .centered()
        .render(misc_area, buf);
    }
}
