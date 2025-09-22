use std::collections::HashMap;

use crate::{Editor, EditorAction};

pub struct EventRegistry {
    data: HashMap<(InputEvent, EventContext), EditorAction>,
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
