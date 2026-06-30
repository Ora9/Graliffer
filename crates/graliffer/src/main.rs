use std::env;

use color_eyre::Result;
use log::debug;
use ratatui::{Terminal, backend::CrosstermBackend};

mod app;
pub use app::*;

mod event;
pub use event::*;

mod ui;
pub use ui::*;

mod tui;
pub use tui::*;

mod input;
pub use input::*;

mod grid;
pub use grid::*;

mod console;
pub use console::*;

fn main() -> Result<()> {
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
    let events = EventHandler::new(200);

    let mut tui = Tui::new(terminal, events);
    tui.enter()?;

    let mut app_state = AppState::new();

    while app_state.should_run {
        tui.draw(App::new(), &mut app_state)?;

        match tui.events.next()? {
            Event::Tick => {
                app_state.tick();
            }
            Event::Key(key_event) => {
                app_state.handle_key_events(key_event, app_state.key_context())
            }
            Event::Mouse(mouse_event) => {
                app_state.handle_mouse_event(mouse_event);
            }
            Event::Resize(_, _) => {}
        };
    }

    tui.exit()?;
    Ok(())
}
