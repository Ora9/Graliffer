use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{
    AppAction,
    input::{KeyContext, KeyContextPredicate, Keystroke},
};

static DEFAULT_KEYMAP: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/assets/default_keymap.jsonc"
));

#[derive(Debug, Serialize, Deserialize)]
pub struct Keymap(Vec<KeymapBindingGroup>);

impl Keymap {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&mut self, binding_group: KeymapBindingGroup) {
        self.0.push(binding_group);
    }

    /// Find a [`Binding`] given a predicate, depending on the given [`Context`]
    ///
    /// # Precedence
    /// The predicate can return `true` for multiples bindings (eg. when finding a keystroke, and
    /// this keystroke is present in multiples applicable [`KeyContext`] in the keymap),
    /// precedence is resolved with two rules :
    /// - Bindings declared with a [`KeyContext`] that is more "specific" takes precedence (see
    ///   [specificity rule](KeyContext::specificity_score))
    /// - Declaration order, with the latest declared taking precedence. User defined keymap are
    ///   loaded after default ones, allowing user bindings to take overwrite defaults
    pub fn find_binding(
        &self,
        mut predicate: impl FnMut(&Binding) -> bool,
        key_context: &KeyContext,
    ) -> Option<Binding> {
        let mut selected_binding: Option<Binding> = None;
        let mut highest_score = 0;

        self.0
            .iter()
            .filter(|binding_group| key_context.matches(&binding_group.context))
            .for_each(|binding_group| {
                let score = binding_group.context.specificity_score();
                if score >= highest_score
                    && let Some(action) = binding_group.find_binding(|binding| predicate(binding))
                {
                    highest_score = score;
                    selected_binding = Some(action);
                }
            });

        selected_binding
    }

    /// Find an [`AppAction`] tied to a [`Keystroke`] depending on the given [`Context`]
    ///
    /// # Precedence
    /// See [precedence rules](Keymap::find_binding#Precedence)
    pub fn find_action(&self, keystroke: Keystroke, key_context: &KeyContext) -> Option<AppAction> {
        self.find_binding(|binding| binding.keystroke == keystroke, key_context)
            .and_then(|binding| Some(binding.action))
    }

    /// Find a [`Keystroke`] tied to an [`AppAction`] depending on the given [`Context`]
    ///
    /// # Precedence
    /// See [precedence rules](Keymap::find_binding#Precedence)
    pub fn find_keystroke(&self, action: AppAction, key_context: &KeyContext) -> Option<Keystroke> {
        self.find_binding(|binding| binding.action == action, key_context)
            .and_then(|binding| Some(binding.keystroke))
    }
}

impl Default for Keymap {
    fn default() -> Self {
        serde_json::from_str(DEFAULT_KEYMAP).expect("default keymap must be valid!")
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct KeymapBindingGroup {
    context: KeyContextPredicate,
    bindings: HashMap<Keystroke, AppAction>,
}

impl KeymapBindingGroup {
    fn find_binding(&self, mut predicate: impl FnMut(&Binding) -> bool) -> Option<Binding> {
        self.bindings.iter().find_map(|(keystroke, action)| {
            let binding = Binding::new(*keystroke, action.clone());
            predicate(&binding).then_some(binding)
        })
    }
}

#[derive(Debug)]
pub struct Binding {
    keystroke: Keystroke,
    action: AppAction,
}

impl Binding {
    pub fn new(keystroke: Keystroke, action: AppAction) -> Self {
        Self { keystroke, action }
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::{
        ConsoleAction,
        GralifferAction::{self},
        GridAction,
        input::{Key, KeyContext},
    };

    use super::*;

    #[test]
    fn parse_default_keymap() {
        Keymap::default();
    }

    #[test]
    fn find() {
        let keymap_json = json!([
            {
                "context": "",
                "bindings": {
                    "1": "FocusGrid",
                    "2": "FocusStack",
                    "3": "FocusConsole"
                }
            },
            {
                "context": "A B &&",
                "bindings": {
                    "1": "console::ScrollUp",
                    "2": "console::ScrollDown",
                    "3": "console::ScrollBottom"
                }
            },
            {
                "context": "A",
                "bindings": {
                    "1": "grid::Undo",
                    "2": "grid::Redo"
                }
            },
            {
                "context": "B",
                "bindings": {
                    "1": "picker::SelectionUp",
                    "2": "picker::SelectionDown"
                }
            }
        ]);

        let keymap: Keymap = serde_json::from_value(keymap_json).unwrap();

        let mut context = KeyContext::default();

        let find =
            |in_context: &KeyContext, keystroke: Option<Keystroke>, action: Option<AppAction>| {
                if let Some(keystroke) = keystroke {
                    assert_eq!(keymap.find_action(keystroke, in_context), action);
                }

                if let Some(action) = action {
                    assert_eq!(keymap.find_keystroke(action, in_context), keystroke);
                }
            };

        find(&context, Some(Keystroke::from_key(Key::Up)), None);

        find(
            &context,
            Some(Keystroke::from_key(Key::Char('1'))),
            Some(AppAction::GralifferAction(GralifferAction::FocusGrid)),
        );

        find(
            &context,
            Some(Keystroke::from_key(Key::Char('1'))),
            Some(AppAction::GralifferAction(GralifferAction::FocusGrid)),
        );

        find(
            &context,
            Some(Keystroke::from_key(Key::Char('3'))),
            Some(AppAction::GralifferAction(GralifferAction::FocusConsole)),
        );

        context.insert("A".into());

        find(
            &context,
            Some(Keystroke::from_key(Key::Char('1'))),
            Some(AppAction::GridAction(GridAction::Undo)),
        );

        find(
            &context,
            Some(Keystroke::from_key(Key::Char('3'))),
            Some(AppAction::GralifferAction(GralifferAction::FocusConsole)),
        );

        context.insert("B".into());

        find(
            &context,
            Some(Keystroke::from_key(Key::Char('1'))),
            Some(AppAction::ConsoleAction(ConsoleAction::ScrollUp)),
        );

        find(
            &context,
            Some(Keystroke::from_key(Key::Char('3'))),
            Some(AppAction::ConsoleAction(ConsoleAction::ScrollBottom)),
        );
    }
}
