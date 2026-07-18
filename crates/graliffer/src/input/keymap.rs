use std::collections::HashMap;

use log::debug;
use serde::{Deserialize, Serialize};

use crate::{AnyAppAction, Context, KeyContextPredicate, Keystroke};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Keymap(Vec<KeymapBindingGroup>);

// #[derive(Debug, Default)]
// pub struct Keymap(HashMap<KeyContextPredicate, KeymapEntries>);

impl Keymap {
    pub fn new() -> Self {
        let mut map = Self::default();

        let mut none = KeymapBindingGroup::new(KeyContextPredicate::None);

        let source = r#"[
            {
                "context": "Grid insert &&",
                "bindings": {
                    "ctrl-q": "Quit",
                    "q": "Quit",
                    "ctrl-a": "TogglAbout"
                }
            }
        ]"#;

        let result = serde_json::from_str::<Keymap>(source);

        // let deser = &mut serde_json::Deserializer::from_str(source);

        // let result: Result<Keymap, _> = serde_path_to_error::deserialize(deser);

        debug!("{:?}", result);

        // let a = serde_json::from_value::<AnyAppAction>(json!("grid::Prou"));

        // match &a {
        //     Ok(_) => {}
        //     Err(err) => debug!("{:?}", err.classify()),
        // }

        // debug!("{:?}", a);

        // none.push(Binding::new(
        //     Keystroke::from_str("ctrl-a").unwrap(),
        //     AnyAppAction::AppAction(AppAction::ToggleAbout),
        // ));

        // let mut grid_insert =
        //     KeymapBindingGroup::new(KeyContextPredicate::from_str("Grid insert &&").unwrap());

        // grid_insert.push(Binding::new(
        //     Keystroke::from_str("q").unwrap(),
        //     AnyAppAction::AppAction(AppAction::Quit),
        // ));

        // map.push(none);
        // map.push(grid_insert);

        // let grid_insert = "Grid insert &&".parse().unwrap();
        // let grid_command = "Grid command &&".parse().unwrap();

        // map.insert(
        //     &grid_insert,
        //     Binding::new(Keystroke::from_str("up").unwrap(), GridAction::CursorUp),
        // );

        // map.insert(
        //     &grid_insert,
        //     Binding::new(Keystroke::from_str("down").unwrap(), GridAction::CursorDown),
        // );

        // map.insert(
        //     &KeyContextPredicate::None,
        //     Binding::new(Keystroke::from_str("q").unwrap(), AppAction::Quit),
        // );

        // map.insert(
        //     &grid_insert,
        //     Binding::new(
        //         Keystroke::from_str("escape").unwrap(),
        //         AppAction::CommandMode,
        //     ),
        // );

        // map.insert(
        //     &grid_command,
        //     Binding::new(Keystroke::from_str("i").unwrap(), AppAction::InsertMode),
        // );

        // let popup = "focusing_popup".parse().unwrap();

        // map.insert(
        //     &popup,
        //     Binding::new(
        //         Keystroke::from_str("escape").unwrap(),
        //         AppAction::ClosePopup,
        //     ),
        // );

        // map.insert(
        //     &KeyContextPredicate::None,
        //     Binding::new(
        //         Keystroke::from_str("ctrl-a").unwrap(),
        //         AppAction::ToggleAbout,
        //     ),
        // );

        // map.insert(
        //     &KeyContextPredicate::None,
        //     Binding::new(
        //         Keystroke::from_str("ctrl-p").unwrap(),
        //         AppAction::ToggleCommandPicker,
        //     ),
        // );

        map
    }

    pub fn push(&mut self, binding_group: KeymapBindingGroup) {
        self.0.push(binding_group);
    }

    // pub fn insert(&mut self, context: &KeyContextPredicate, keymap_entry: Binding) {
    //     if let Some(for_context) = self.0.get_mut(context) {
    //         for_context.push(keymap_entry);
    //     } else {
    //         self.0.insert(context.clone(), vec![keymap_entry].into());
    //     }
    // }

    pub fn find(&self, app_context: Context, keystroke: Keystroke) -> Option<AnyAppAction> {
        // todo: make more specific context predicate have a higher priorities
        // or maybe just by order of declaration?

        self.0
            .iter()
            .filter(|binding_group| app_context.matches_key_context(&binding_group.context))
            .find_map(|binding_group| binding_group.find_keystroke(keystroke))
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct KeymapBindingGroup {
    // #[serde(rename = "context")]
    context: KeyContextPredicate,
    bindings: HashMap<Keystroke, AnyAppAction>,
}

impl KeymapBindingGroup {
    fn new(context: KeyContextPredicate) -> Self {
        Self {
            context,
            ..Default::default()
        }
    }

    fn push(&mut self, keystroke: Keystroke, action: AnyAppAction) {
        self.bindings.insert(keystroke, action);
    }

    fn find_keystroke(&self, keystroke: Keystroke) -> Option<AnyAppAction> {
        self.bindings
            .iter()
            .find(|(entry_keystroke, _)| **entry_keystroke == keystroke)
            .and_then(|(_, action)| Some(action.clone()))
    }
}

// impl Serialize for KeymapBindingGroup {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: serde::Serializer,
//     {
//         let mut group_state = serializer.serialize_struct("KeymapBindingGroup", 2)?;
//         group_state.serialize_field("context", &self.context);

//         let mut bindings_state = serializer.serialize_map(Some(self.bindings.len()))?;

//         for binding in self.bindings.iter() {
//             bindings_state.serialize_entry(&binding.keystroke, &binding.action)?;
//         }

//         bindings_state.end()

//         // state.serialize_field("bindings", &self.bindings)
//     }
// }

#[derive(Debug, Serialize, Deserialize)]
pub struct Binding {
    keystroke: Keystroke,
    action: AnyAppAction,
}

impl Binding {
    pub fn new(keystroke: Keystroke, action: AnyAppAction) -> Self {
        Self { keystroke, action }
    }
}

// impl From<Vec<Binding>> for KeymapEntries {
//     fn from(value: Vec<Binding>) -> Self {
//         Self(value)
//     }
// }

// impl Deref for KeymapEntries {
//     type Target = Vec<Binding>;

//     fn deref(&self) -> &Self::Target {
//         &self.0
//     }
// }

// impl DerefMut for KeymapEntries {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         &mut self.0
//     }
// }
