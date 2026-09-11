use color_eyre::Result;
use log::debug;
use ratatui::{Terminal, backend::CrosstermBackend};
use std::env;

use graliffer::{App, AppWidget, Config, Event, EventHandler, Tui};

fn main() -> Result<()> {
    let config = Config::default();

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
    let mut tui = Tui::new(terminal);
    tui.enter()?;

    let events = EventHandler::new(250);

    let mut app_state = App::new(config);

    while app_state.should_run {
        tui.draw(AppWidget::new(), &mut app_state)?;

        match events.next()? {
            Event::Tick => {
                app_state.tick();
            }
            Event::Key(key_event) => {
                let context = app_state.context.clone();
                app_state.handle_key_events(key_event, context);
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
