use std::env;

use color_eyre::Result;
use event::{Event, EventHandler};
use log::debug;
use ratatui::{Terminal, backend::CrosstermBackend};

pub mod app;
use app::App;

pub mod event;

pub mod ui;

pub mod tui;
use tui::Tui;

pub mod inputs;
use inputs::*;

fn main() -> Result<()> {
    let mut app = App::new();

    color_eyre::install()?;

    tui_logger::init_logger(log::LevelFilter::Trace)?;
    tui_logger::set_default_level(log::LevelFilter::Trace);

    let mut temp_dir = env::temp_dir();
    temp_dir.push("graliffer.log");

    let file_options = tui_logger::TuiLoggerFile::new(temp_dir.to_str().unwrap())
        .output_level(Some(tui_logger::TuiLoggerLevelOutput::Abbreviated))
        .output_file(false)
        .output_separator(':');

    tui_logger::set_log_file(file_options);
    debug!(target:"App", "Logging to {}", temp_dir.to_str().unwrap());
    debug!(target:"App", "Logging initialized");

    let backend = CrosstermBackend::new(std::io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(2000);

    let mut tui = Tui::new(terminal, events);
    tui.enter()?;

    while app.should_run {
        tui.draw(&mut app)?;

        match tui.events.next()? {
            Event::Tick => {
                app.tick();
            }
            Event::Key(key_event) => app.handle_key_events(key_event),
            Event::Mouse(mouse_event) => {
                app.handle_mouse_event(mouse_event);
            }
            Event::Resize(_, _) => {}
        };
    }

    tui.exit()?;
    Ok(())
}
