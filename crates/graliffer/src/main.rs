use color_eyre::Result;
use event::{Event, EventHandler};
use ratatui::{Terminal, backend::CrosstermBackend};

pub mod app;
use app::App;

pub mod event;

pub mod ui;

pub mod tui;
use tui::Tui;
// use logger::TuiLoggerFile;

fn main() -> Result<()> {
    let mut app = App::new();

    color_eyre::install()?;

    // tui_logger::init_logger(log::LevelFilter::Trace)?;

    // tui_logger::set_default_level(log::LevelFilter::Trace);
    // tui_logger::set_log_file(TuiLoggerFile::new("/tmp/graliffer_log_file.txt"));

    let backend = CrosstermBackend::new(std::io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);

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
