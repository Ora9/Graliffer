use act::Timeline;
use color_eyre::Result;
use log::debug;
use ratatui::{Terminal, backend::CrosstermBackend};
use std::env;

use graliffer::{App, AppState, Config, Event, EventHandler, Tui, handle_key_events};

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
    let events = EventHandler::new(200);

    let mut tui = Tui::new(terminal, events);
    tui.enter()?;

    // let mut app_state = AppState::new(config);

    let mut app_state_timeline = Timeline::new(AppState::new(config));

    while app_state_timeline.state().should_run {
        tui.draw(App::new(), app_state_timeline.state_mut())?;

        match tui.events.next()? {
            Event::Tick => {
                app_state_timeline.state_mut().tick();
            }
            Event::Key(key_event) => {
                let context = app_state_timeline.state().context.clone();
                handle_key_events(&mut app_state_timeline, key_event, context);
            }
            Event::Mouse(mouse_event) => {
                app_state_timeline
                    .state_mut()
                    .handle_mouse_event(mouse_event);
            }
            Event::Resize(_, _) => {}
        };
        debug!("{:#?}", app_state_timeline.undoes());
    }

    tui.exit()?;
    Ok(())
}
