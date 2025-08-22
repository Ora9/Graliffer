use std::collections::HashMap;

use egui::{Event, KeyboardShortcut};

use crate::{action::{EditorAction}, editor::history_utils::HistoryAction, Editor};

pub struct ShortcutRegistry {
    data: HashMap<(KeyboardShortcut, ShortcutContext), Box<dyn EditorAction>>
}

impl ShortcutRegistry {
    pub fn build() -> Self {
        let actions: Vec<Box<dyn EditorAction>> = vec![
            Box::new(HistoryAction::Redo),
            Box::new(HistoryAction::Undo),
        ];

        let mut registry
            : HashMap<(KeyboardShortcut, ShortcutContext), Box<dyn EditorAction>>
            = HashMap::new();

        for action in actions {
            if let Some((shortcut, context)) = action.shortcut_and_context() {
                registry.insert((shortcut, context), action);
            }
        }

        // have to re-sort, in shortcut order (shift alt etc first)

        Self {
            data: registry
        }
    }
}

impl Editor {
    pub fn listen_for_shortcut(&self, ctx: &egui::Context) -> Option<Box<dyn EditorAction>> {
        let current_context = ShortcutContext::load(ctx);
        let events = ctx.input(|i| i.events.clone());

        for event in events {
            return match event {
                Event::Key {
                    key,
                    modifiers,
                    pressed: true,
                    ..
                } => {

                    let event_shortcut = KeyboardShortcut {
                        logical_key: key,
                        modifiers
                    };

                    let get_action = |context| {
                        self.shortcut_registry.data.get(&(event_shortcut, context))
                    };

                    if let Some(action) = get_action(current_context) {
                        dbg!("oui");
                        Some(action.clone())
                    } else if let Some(action) = get_action(ShortcutContext::None) {
                        dbg!("oui mais deux");
                        Some(action.clone())
                    } else {
                        None
                    }
                }
                _ => {
                    None
                }
            }
        }
        None
    }
}

#[derive(Debug, Default, Clone, Hash, PartialEq, Eq)]
pub enum ShortcutContext {
    #[default]
    None,
    Grid,
    GridSelecting,
    Stack,
    Console,
    Graphic,
    CommandPanel,
}

impl ShortcutContext {
    const ID: &'static str = "KEYBIND_CONTEXT";

    pub fn store(ctx: &egui::Context, keybind_context: ShortcutContext) {
        ctx.data_mut(|data| {
            data.insert_persisted(egui::Id::new(Self::ID), keybind_context);
        });
    }

    pub fn load(ctx: &egui::Context) -> ShortcutContext {
        ctx.data_mut(|data| {
            data.get_persisted(egui::Id::new(Self::ID)).unwrap_or_default()
        })
    }
}
