use action::{Action, AnyAction, State};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers, ModifierKeyCode, MouseEvent};
use ratatui::{
    layout::Position,
    style::Stylize,
    text::{Span, ToSpan},
};

use crate::{
    app::{
        App,
        AppAction::{self, FocusStack},
        Focusable,
    },
    ui::{ConsoleAction, FocusedPane::Console},
};

// pub struct KeyContext(Vec<KeyContextEntry>);

// pub struct KeyContextEntry {
//     key: String,
//     value: Option<String>,
// }

// impl KeyContextEntry {
//     pub fn new_key(key: String) -> Self {
//         Self { key, value: None }
//     }

//     pub fn new_key_value(key: String, value: String) -> Self {
//         Self {
//             key,
//             value: Some(value),
//         }
//     }
// }

// // "Grid && mode == insert"
// // "ProjectPanel && mode == "
// pub enum ContextPredicate {
//     Identifier(String),
//     // Equal(String, String),
//     // NotEqual(String, String),

//     // Not(Box<ContextPredicate>),
//     // And(Box<ContextPredicate>, Box<ContextPredicate>),
//     // Or(Box<ContextPredicate>, Box<ContextPredicate>),
// }

// pub struct ContextTree {}

#[derive(Debug, Clone, Copy)]
pub struct KeyContext {
    pub focus: Focusable,
    pub input_mode: InputMode,
}

#[derive(Debug, Default)]
pub struct KeyContextPredicate {
    focus: Option<Focusable>,
    input_mode: Option<InputMode>,
}

impl KeyContextPredicate {
    pub fn matches(&self, key_context: KeyContext) -> bool {
        if let Some(focus) = self.focus
            && focus != key_context.focus
        {
            false
        } else if let Some(input_mode) = self.input_mode
            && input_mode != key_context.input_mode
        {
            false
        } else {
            true
        }
    }
}

#[derive(Debug)]
pub struct Keybind {
    modifiers: KeyModifiers,
    key: KeyCode,
}

impl Keybind {
    pub fn from_key(key: KeyCode) -> Self {
        Self {
            key,
            modifiers: KeyModifiers::NONE,
        }
    }

    pub fn matches(&self, key_event: KeyEvent) -> bool {
        self.key == key_event.code && self.modifiers == key_event.modifiers
    }
}

#[derive(Debug)]
pub struct KeymapEntry {
    keybind: Keybind,
    context_predicate: KeyContextPredicate,
    action: AnyAction,
}

impl KeymapEntry {
    pub fn new(
        keybind: Keybind,
        context_predicate: KeyContextPredicate,
        action: AnyAction,
    ) -> Self {
        Self {
            keybind,
            context_predicate,
            action,
        }
    }
}

#[derive(Debug, Default)]
pub struct Keymap(Vec<KeymapEntry>);

impl Keymap {
    pub fn new() -> Self {
        let mut map = Self::default();

        map.push(
            Keybind::from_key(KeyCode::Char('q')),
            KeyContextPredicate::default(),
            AppAction::Quit,
        );

        map.push(
            Keybind::from_key(KeyCode::Char('i')),
            KeyContextPredicate {
                input_mode: Some(InputMode::Command),
                ..Default::default()
            },
            AppAction::InsertMode,
        );

        map.push(
            Keybind::from_key(KeyCode::Esc),
            KeyContextPredicate {
                input_mode: Some(InputMode::Insert),
                ..Default::default()
            },
            AppAction::CommandMode,
        );

        map.push(
            Keybind {
                key: KeyCode::Char('c'),
                modifiers: KeyModifiers::CONTROL,
            },
            KeyContextPredicate::default(),
            AppAction::Quit,
        );

        map.push(
            Keybind {
                key: KeyCode::Char('a'),
                modifiers: KeyModifiers::CONTROL,
            },
            KeyContextPredicate::default(),
            AppAction::About,
        );
        map.push(
            Keybind {
                key: KeyCode::Char('c'),
                modifiers: KeyModifiers::CONTROL | KeyModifiers::SHIFT,
            },
            KeyContextPredicate::default(),
            ConsoleAction::Clear,
        );

        map.push(
            Keybind {
                key: KeyCode::Char('f'),
                modifiers: KeyModifiers::CONTROL,
            },
            KeyContextPredicate::default(),
            AppAction::FocusStack,
        );

        map
    }

    pub fn push(
        &mut self,
        keybind: Keybind,
        context_predicate: KeyContextPredicate,
        action: impl Action,
    ) {
        self.0.push(KeymapEntry {
            keybind,
            context_predicate,
            action: AnyAction::new(action),
        });
    }

    pub fn find(&self, key_event: KeyEvent, key_context: KeyContext) -> Option<AnyAction> {
        self.0
            .iter()
            .find(|item| {
                item.keybind.matches(key_event) && item.context_predicate.matches(key_context)
            })
            .and_then(|item| Some(item.action.clone()))
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum InputMode {
    Insert,
    Command,
}

impl InputMode {
    pub fn formated<'a>(&self) -> Span<'a> {
        use InputMode::*;
        match self {
            Command => "COMMAND".red(),
            Insert => "INSERT".to_span(),
        }
    }
}

impl App {
    pub fn handle_key_events(&mut self, key_event: KeyEvent, key_context: KeyContext) {
        if let Some(action) = self.keymap.find(key_event, key_context) {
            self.act(&action);
        }

        // self.grid_state.handle_key_event(key_event);
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

        if let Some(grid_layout) = self.grid_state.layout() {
            let contained = grid_layout.contains(Position {
                x: mouse_event.column,
                y: mouse_event.row,
            });

            if contained {
                self.grid_state.handle_mouse_event(mouse_event);
            }
        }
    }
}
