use std::collections::HashMap;

use crate::{action::EditorAction, editor::{grid_widget::GridEditorAction, history_utils::HistoryAction}, Editor};

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
            vec![
                Box::new(HistoryAction::Redo),
                Box::new(HistoryAction::Undo),

                Box::new(GridEditorAction::Insert(String::default())),
            ];

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
    fn get_action_from_context(&self, input_event: InputEvent, current_context: EventContext) -> Option<Box<dyn EditorAction>> {
        let get = |context| {
            self.event_registry
                .data
                .get(&(input_event.clone(), context))
        };

        dbg!(&current_context, &input_event);

        if let Some(action) = get(current_context) {
            Some(action.clone())
        } else if let Some(action) = get(EventContext::None) {
            Some(action.clone())
        } else {
            dbg!("a");
            None
        }
    }

    pub fn listen_for_events(
        &self,
        ctx: &egui::Context,
        event: egui::Event,
    ) -> Option<Box<dyn EditorAction>> {
        let current_context = EventContext::load(ctx);

        match event {
            egui::Event::Text(text_input) => {
                let input_event = InputEvent::Text(text_input);
                self.get_action_from_context(input_event, current_context)
            }
            egui::Event::Key {
                key,
                modifiers,
                pressed: true,
                ..
            } => {
                let input_event = InputEvent::Key { key, modifiers };
                self.get_action_from_context(input_event, current_context)
            }
            _ => None,
        }
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
