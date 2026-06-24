use action::{Action, AnyAction, State};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers, ModifierKeyCode, MouseEvent};
use log::debug;
use ratatui::{
    layout::Position,
    style::Stylize,
    text::{Span, ToSpan},
};

use crate::{
    app::{
        App,
        AppAction::{self, FocusStack},
        FocusId, Focusable,
    },
    ui::{ConsoleAction, GridAction},
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
    pub focus: FocusId,
    pub input_mode: InputMode,
}

#[derive(Debug, Default)]
pub struct KeyContextPredicate {
    focus: Option<FocusId>,
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

#[derive(Debug, PartialEq, Eq)]
pub struct Keystroke {
    modifiers: KeyModifiers,
    key: KeyCode,
}

impl Keystroke {
    pub fn from_key(key: KeyCode) -> Self {
        Self {
            key,
            modifiers: KeyModifiers::NONE,
        }
    }

    pub fn from_event(key_event: KeyEvent) -> Self {
        Self {
            modifiers: key_event.modifiers,
            key: key_event.code,
        }
    }

    fn to_string(&self) -> String {
        format!("{} {}", self.modifiers.to_string(), self.key.to_string())

        // if self.modifiers.contains(KeyModifiers::CONTROL)
    }

    // fn parse(source: String) -> eyre::Result<Self> {
    //     let mut modifiers = KeyModifiers::NONE;
    //     let mut key: Option<KeyCode> = None;

    //     let mut parts = source.split('-');
    //     while let Some(part) = parts.next() {
    //         if part.eq_ignore_ascii_case("ctrl") {
    //             modifiers.insert(KeyModifiers::CONTROL);
    //             continue;
    //         } else if part.eq_ignore_ascii_case("shift") {
    //             modifiers.insert(KeyModifiers::SHIFT);
    //             continue;
    //         } else if part.eq_ignore_ascii_case("alt") {
    //             modifiers.insert(KeyModifiers::ALT);
    //             continue;
    //         };

    //         if let Some(next) = components.peek() {
    //             if next.is_empty() && source.ends_with('-') {
    //                 key = Some(KeyCode::Char('-'));
    //                 break;
    //             } else {
    //                 Err("invalid keystroke representation")
    //             }
    //         } else {

    //         }
    //     // todo!()
    // }

    pub fn matches(&self, key_event: KeyEvent) -> bool {
        self.key == key_event.code && self.modifiers == key_event.modifiers
    }
}

#[derive(Debug)]
pub struct KeymapEntry {
    keystroke: Keystroke,
    context_predicate: KeyContextPredicate,
    action: AnyAction,
}

impl KeymapEntry {
    pub fn new(
        keystroke: Keystroke,
        context_predicate: KeyContextPredicate,
        action: AnyAction,
    ) -> Self {
        Self {
            keystroke,
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
            Keystroke::from_key(KeyCode::Up),
            KeyContextPredicate {
                focus: Some(Focusable::Grid.into()),
                input_mode: Some(InputMode::Command),
            },
            GridAction::CursorUp,
        );

        map.push(
            Keystroke::from_key(KeyCode::Down),
            KeyContextPredicate {
                focus: Some(Focusable::Grid.into()),
                input_mode: Some(InputMode::Command),
            },
            GridAction::CursorDown,
        );

        map.push(
            Keystroke::from_key(KeyCode::Char('q')),
            KeyContextPredicate::default(),
            AppAction::Quit,
        );

        map.push(
            Keystroke::from_key(KeyCode::Char('i')),
            KeyContextPredicate {
                input_mode: Some(InputMode::Command),
                ..Default::default()
            },
            AppAction::InsertMode,
        );

        map.push(
            Keystroke::from_key(KeyCode::Esc),
            KeyContextPredicate {
                input_mode: Some(InputMode::Insert),
                ..Default::default()
            },
            AppAction::CommandMode,
        );

        map.push(
            Keystroke {
                key: KeyCode::Char('c'),
                modifiers: KeyModifiers::CONTROL,
            },
            KeyContextPredicate::default(),
            AppAction::Quit,
        );

        map.push(
            Keystroke {
                key: KeyCode::Char('a'),
                modifiers: KeyModifiers::CONTROL,
            },
            KeyContextPredicate::default(),
            AppAction::About,
        );
        map.push(
            Keystroke {
                key: KeyCode::Char('c'),
                modifiers: KeyModifiers::CONTROL | KeyModifiers::SHIFT,
            },
            KeyContextPredicate::default(),
            ConsoleAction::Clear,
        );

        map.push(
            Keystroke {
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
        keystroke: Keystroke,
        context_predicate: KeyContextPredicate,
        action: impl Action,
    ) {
        self.0.push(KeymapEntry {
            keystroke,
            context_predicate,
            action: AnyAction::new(action),
        });
    }

    pub fn find(&self, keystroke: Keystroke, key_context: KeyContext) -> Option<AnyAction> {
        self.0
            .iter()
            .find(|item| item.keystroke == keystroke && item.context_predicate.matches(key_context))
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
        let keystroke = Keystroke::from_event(key_event);

        debug!("{:?}", keystroke.to_string());

        if let Some(action) = self.keymap.find(keystroke, key_context) {
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
