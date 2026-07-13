use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
    str::FromStr,
};

use action::{Action, AnyAction};

use crate::{AppAction, Context, GridAction, KeyContextPredicate, Keystroke};

#[derive(Debug)]
pub struct KeymapEntry {
    keystroke: Keystroke,
    action: AnyAction,
}

impl KeymapEntry {
    pub fn new(keystroke: Keystroke, action: impl Action) -> Self {
        Self {
            keystroke,
            action: AnyAction::new(action),
        }
    }
}

#[derive(Debug, Default)]
struct KeymapEntries(Vec<KeymapEntry>);

impl KeymapEntries {
    fn find_keystroke(&self, keystroke: Keystroke) -> Option<AnyAction> {
        self.iter()
            .find(|entry| entry.keystroke == keystroke)
            .and_then(|entry| Some(entry.action.clone()))
    }
}

impl From<Vec<KeymapEntry>> for KeymapEntries {
    fn from(value: Vec<KeymapEntry>) -> Self {
        Self(value)
    }
}

impl Deref for KeymapEntries {
    type Target = Vec<KeymapEntry>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for KeymapEntries {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Debug, Default)]
pub struct Keymap(HashMap<KeyContextPredicate, KeymapEntries>);

impl Keymap {
    pub fn new() -> Self {
        let mut map = Self::default();

        let grid_insert = KeyContextPredicate::parse("Grid insert &&").unwrap();
        let grid_command = KeyContextPredicate::parse("Grid command &&").unwrap();

        map.insert(
            &grid_insert,
            KeymapEntry::new(Keystroke::from_str("up").unwrap(), GridAction::CursorUp),
        );

        map.insert(
            &grid_insert,
            KeymapEntry::new(Keystroke::from_str("down").unwrap(), GridAction::CursorDown),
        );

        map.insert(
            &KeyContextPredicate::None,
            KeymapEntry::new(Keystroke::from_str("q").unwrap(), AppAction::Quit),
        );

        map.insert(
            &grid_insert,
            KeymapEntry::new(
                Keystroke::from_str("escape").unwrap(),
                AppAction::CommandMode,
            ),
        );

        map.insert(
            &grid_command,
            KeymapEntry::new(Keystroke::from_str("i").unwrap(), AppAction::InsertMode),
        );

        let popup = KeyContextPredicate::parse("focusing_popup").unwrap();

        map.insert(
            &popup,
            KeymapEntry::new(
                Keystroke::from_str("escape").unwrap(),
                AppAction::ClosePopup,
            ),
        );

        map.insert(
            &KeyContextPredicate::None,
            KeymapEntry::new(
                Keystroke::from_str("ctrl-a").unwrap(),
                AppAction::ToggleAbout,
            ),
        );

        map.insert(
            &KeyContextPredicate::None,
            KeymapEntry::new(
                Keystroke::from_str("ctrl-p").unwrap(),
                AppAction::ToggleCommandPicker,
            ),
        );

        map
    }

    pub fn insert(&mut self, context: &KeyContextPredicate, keymap_entry: KeymapEntry) {
        if let Some(for_context) = self.0.get_mut(context) {
            for_context.push(keymap_entry);
        } else {
            self.0.insert(context.clone(), vec![keymap_entry].into());
        }
    }

    pub fn find(&self, app_context: Context, keystroke: Keystroke) -> Option<AnyAction> {
        // todo: make specific context predicate have a higher priorities

        self.0
            .iter()
            .filter(|(predicate, _)| app_context.matches_key_context(predicate))
            .find_map(|(_, entries)| entries.find_keystroke(keystroke))
    }
}
