use std::{collections::HashSet, fmt::Display, hash::Hash};

use serde::{Deserialize, Serialize};

use crate::{
    ViewId,
    input::{InputMode, KeyContextPredicate},
};

/// A flag that can be matched by a predicate (eg. in keymap, when specifying a context)
///
/// # Flag naming guideline
/// Technically any string is valid, but some pattern can lead to erroneous
/// behavior :
/// - Whitespace or empty strings like `` or ` ` ..
/// - [`KeyContextPredicate`] operators like `&&`, `^^` or `!` ..
/// - [`ViewId`]s like `Grid` or `Picker`
/// - [`InputMode`] like `command` and `insert`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct KeyContextFlag(String);

impl From<&str> for KeyContextFlag {
    fn from(value: &str) -> Self {
        KeyContextFlag(value.to_string())
    }
}

impl From<String> for KeyContextFlag {
    fn from(value: String) -> Self {
        KeyContextFlag(value)
    }
}

impl Display for KeyContextFlag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeyContext {
    input_mode: InputMode,
    focus: ViewId,
    flags: HashSet<KeyContextFlag>,
}

// impl Hash for KeyContext {
//     fn hash<H: Hasher>(&self, state: &mut H) {
//         let mut sum: Wrapping<u64> = Wrapping::default();

//         for (key, flag) in &self.0 {
//             let mut hasher = DefaultHasher::new();
//             Hash::hash(key, &mut hasher);
//             Hash::hash(flag, &mut hasher);
//             sum += hasher.finish()
//         }

//         state.write_u64(sum.0);
//     }
// }

impl KeyContext {
    pub fn new(focus: ViewId, input_mode: InputMode) -> Self {
        Self {
            focus,
            input_mode,
            flags: HashSet::default(),
        }
    }

    pub fn insert(&mut self, flag: KeyContextFlag) {
        self.flags.insert(flag);
    }

    pub fn remove(&mut self, flag: &KeyContextFlag) {
        self.flags.remove(flag);
    }

    pub fn has(&self, flag: &KeyContextFlag) -> bool {
        // if we insert a flag with a ViewId name like `Grid`, the predicate "Console Grid &&" can
        // be true, we should avoid having these kind of flags in self.flags

        // TODO: this is kinda ugly
        if self.focus.to_string() == flag.to_string()
            || self.input_mode.to_string() == flag.to_string()
        {
            true
        } else {
            self.flags.contains(flag)
        }
    }

    pub fn set_focus(&mut self, focus: ViewId) {
        self.focus = focus;
    }

    pub fn focus(&self) -> ViewId {
        self.focus
    }

    pub fn set_input_mode(&mut self, input_mode: InputMode) {
        self.input_mode = input_mode;
    }

    pub fn input_mode(&self) -> InputMode {
        self.input_mode
    }

    pub fn matches(&self, predicate: &KeyContextPredicate) -> bool {
        use KeyContextPredicate::*;

        match predicate {
            None => true,
            Flag(flag) => self.has(&flag.clone()),
            Not(predicate) => !self.matches(predicate),
            And(lhs, rhs) => self.matches(lhs) && self.matches(rhs),
            Or(lhs, rhs) => self.matches(lhs) || self.matches(rhs),
            Xor(lhs, rhs) => self.matches(lhs) ^ self.matches(rhs),
        }
    }
}
