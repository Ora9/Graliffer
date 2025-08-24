use std::collections::HashMap;

use crate::{Editor, action::EditorAction, editor::history_utils::HistoryAction};

/// A selective copy of egui::Event but only the event we care about, in a way
/// we can store the event match against
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum InputEvent {
    Copy,
    Cut,
    Paste,
    Text(String),
    Key {
        key: egui::Key,
        modifiers: egui::Modifiers,
    },
}

pub struct EventRegistry {
    data: HashMap<(InputEvent, EventContext), Box<dyn EditorAction>>,
}

impl EventRegistry {
    pub fn build() -> Self {
        let actions: Vec<Box<dyn EditorAction>> =
            vec![Box::new(HistoryAction::Redo), Box::new(HistoryAction::Undo)];

        let mut registry: HashMap<(InputEvent, EventContext), Box<dyn EditorAction>> =
            HashMap::new();

        for action in actions {
            if let Some((event, context)) = action.events_and_context() {
                registry.insert((event, context), action);
            }
        }

        // have to re-sort, in shortcut order (shift alt etc first)

        Self { data: registry }
    }
}

impl Editor {
    // pub fn listen_for_events(&self, ctx: &egui::Context) -> Option<Box<dyn EditorAction>> {
    pub fn listen_for_events(
        &self,
        ctx: &egui::Context,
        event: egui::Event,
    ) -> Option<Box<dyn EditorAction>> {
        let current_context = EventContext::load(ctx);

        match event {
            egui::Event::Text(text_input) => {
                dbg!(text_input);
                None
            }
            egui::Event::Key {
                key,
                modifiers,
                pressed: true,
                ..
            } => {
                let input_event = InputEvent::Key { key, modifiers };

                let get_action = |context| {
                    self.event_registry
                        .data
                        .get(&(input_event.clone(), context))
                };

                if let Some(action) = get_action(current_context) {
                    Some(action.clone())
                } else if let Some(action) = get_action(EventContext::None) {
                    Some(action.clone())
                } else {
                    None
                }
            }
            _ => None,
        }

        //         _ => {
        //         }
        //     }

        // }
        //     return match event {
        //         egui::Event::Text(text_input) => {

        //             dbg!(text_input);
        //             None
        //         }
        // egui::Event::Key {
        //     key,
        //     modifiers,
        //     pressed: true,
        //     ..
        // } => {
        //     let input_event = InputEvent::Key {
        //         key,
        //         modifiers,
        //     };

        //     let get_action = |context| {
        //         self.event_registry.data.get(&(input_event.clone(), context))
        //     };

        //     if let Some(action) = get_action(current_context) {
        //         Some(action.clone())
        //     } else if let Some(action) = get_action(EventContext::None) {
        //         Some(action.clone())
        //     } else {
        //         None
        //     }
        // }
        //     }
        // }
        // None
    }
}

#[derive(Debug, Default, Clone, Hash, PartialEq, Eq)]
pub enum EventContext {
    #[default]
    None,
    Grid,
    GridSelecting,
    Stack,
    Console,
    Graphic,
    CommandPanel,
}

impl EventContext {
    const ID: &'static str = "EVENT_CONTEXT";

    pub fn store(ctx: &egui::Context, event_context: EventContext) {
        ctx.data_mut(|data| {
            data.insert_persisted(egui::Id::new(Self::ID), event_context);
        });
    }

    pub fn load(ctx: &egui::Context) -> EventContext {
        ctx.data_mut(|data| {
            data.get_persisted(egui::Id::new(Self::ID))
                .unwrap_or_default()
        })
    }
}
