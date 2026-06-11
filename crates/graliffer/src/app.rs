use std::{cell::RefCell, iter, ops::AddAssign, rc::Rc};

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers, MouseEvent};
use rand::seq::SliceRandom;
use ratatui::layout::Position;

use crate::ui::{Console, ConsoleState, GridState};

#[derive(Debug)]
pub struct App {
    pub should_run: bool,
    pub console_state: ConsoleState,
    pub grid_state: GridState,
    pub focused: Focused,
}

impl App {
    pub fn new() -> Self {
        let mut grid = grai::Grid::new();

        grid.set(
            grai::Position::from_string("AA").unwrap(),
            grai::Cell::new_trim("100"),
        );
        grid.set(
            grai::Position::from_string("BA").unwrap(),
            grai::Cell::new_trim("&BB"),
        );
        grid.set(
            grai::Position::from_string("CA").unwrap(),
            grai::Cell::new_trim("div"),
        );
        grid.set(
            grai::Position::from_string("BB").unwrap(),
            grai::Cell::new_trim("@CB"),
        );
        grid.set(
            grai::Position::from_string("CB").unwrap(),
            grai::Cell::new_trim("3"),
        );

        grid.set(
            grai::Position::from_string("EA").unwrap(),
            grai::Cell::new_trim("20"),
        );
        grid.set(
            grai::Position::from_string("FA").unwrap(),
            grai::Cell::new_trim("sub"),
        );
        grid.set(
            grai::Position::from_string("HA").unwrap(),
            grai::Cell::new_trim("@AB"),
        );
        grid.set(
            grai::Position::from_string("IA").unwrap(),
            grai::Cell::new_trim("set"),
        );
        let mut frame = Rc::new(RefCell::new(grai::Frame {
            grid,
            head: grai::Head::default(),
            stack: grai::Stack::default(),
        }));

        let mut app = Self {
            should_run: true,
            focused: Focused::Grid,
            console_state: ConsoleState::new(1000),
            grid_state: GridState::new(frame),
        };

        let mut rng = rand::rng();
        let phrase = "Lorem ipsum dolor sit amet, consectetur adipiscing elit.".to_string();

        let mut shuffler = || {
            let mut phrase = phrase.split(" ").collect::<Vec<&str>>();
            phrase.shuffle(&mut rng);
            phrase.join(" ").to_string()
        };

        for i in 0..100 {
            app.console_state.append_line(shuffler());
        }

        app
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&mut self) {
        // self.console_state.scroll_down_by(1);

        // self.console_state.scroll_offset = self.console_state.scroll_offset.wrapping_add(1);
    }

    pub fn handle_key_events(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Esc | KeyCode::Char('q') => self.quit(),
            KeyCode::Char('c') | KeyCode::Char('C')
                if key_event.modifiers == KeyModifiers::CONTROL =>
            {
                self.quit()
            }
            // KeyCode::Right | KeyCode::Char('j') => app.increment_counter(),
            // KeyCode::Left | KeyCode::Char('k') => app.decrement_counter(),
            _ => {}
        };
    }

    pub fn handle_mouse_event(&mut self, mouse_event: MouseEvent) {
        // TODO: this is a temporary solution to filter event targets based on position
        if let Some(console_layouts) = self.console_state.layouts() {
            // if mouse_event console_layouts.viewport_area()

            let contained = console_layouts
                .viewport_area()
                .union(console_layouts.vertical_scrollbar_area())
                .contains(Position {
                    x: mouse_event.column,
                    y: mouse_event.row,
                });

            if contained {
                self.console_state.handle_mouse_event(mouse_event);
            }
        }
    }

    /// Set should_quit to true to quit the application.
    pub fn quit(&mut self) {
        self.should_run = false;
    }
}

#[derive(Debug)]
pub enum Focused {
    Grid,
    Stack,
    Console,
}
