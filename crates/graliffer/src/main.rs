use color_eyre::Result;
use event::{Event, EventHandler};
use ratatui::{Terminal, backend::CrosstermBackend};

pub mod app;
use app::App;

pub mod event;

pub mod ui;

pub mod tui;
use tui::Tui;

pub mod update;
use update::update;

fn main() -> Result<()> {
    let mut app = App::new();

    color_eyre::install()?;

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
            Event::Key(key_event) => update(&mut app, key_event),
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
        };
    }

    tui.exit()?;
    Ok(())
}
