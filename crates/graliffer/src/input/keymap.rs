use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
    str::FromStr,
};

use action::{Action, AnyAction};

use crate::{AppAction, Context, GridAction, KeyContextPredicate, Keystroke};

#[derive(Debug, Default)]
pub struct Keymap(Vec<KeymapBindingGroup>);

// #[derive(Debug, Default)]
// pub struct Keymap(HashMap<KeyContextPredicate, KeymapEntries>);

impl Keymap {
    pub fn new() -> Self {
        let mut map = Self::default();

        let mut grid_insert =
            KeymapBindingGroup::new(KeyContextPredicate::from_str("Grid insert &&").unwrap());

        grid_insert.push(Binding::new(
            Keystroke::from_str("up").unwrap(),
            GridAction::CursorUp,
        ));

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

    pub fn find(&self, app_context: Context, keystroke: Keystroke) -> Option<AnyAction> {
        // todo: make more specific context predicate have a higher priorities
        // or maybe just by order of declaration?

        self.0
            .iter()
            .filter(|binding_group| app_context.matches_key_context(&binding_group.predicate))
            .find_map(|binding_group| binding_group.find_keystroke(keystroke))
    }
}

#[derive(Debug, Default)]
pub struct KeymapBindingGroup {
    predicate: KeyContextPredicate,
    bindings: Vec<Binding>,
}

impl KeymapBindingGroup {
    fn new(predicate: KeyContextPredicate) -> Self {
        Self {
            predicate,
            ..Default::default()
        }
    }

    fn push(&mut self, binding: Binding) {
        self.bindings.push(binding);
    }

    fn find_keystroke(&self, keystroke: Keystroke) -> Option<AnyAction> {
        self.bindings
            .iter()
            .find(|entry| entry.keystroke == keystroke)
            .and_then(|entry| Some(entry.action.clone()))
    }
}

#[derive(Debug)]
pub struct Binding {
    keystroke: Keystroke,
    action: AnyAction,
}

impl Binding {
    pub fn new(keystroke: Keystroke, action: impl Action) -> Self {
        Self {
            keystroke,
            action: AnyAction::new(action),
        }
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
