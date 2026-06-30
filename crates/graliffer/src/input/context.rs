// pub struct KeyContext(Vec<KeyContextEntry>);

// "insert && Grid && !editing"
// "popup_opened"

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

// "Grid && mode == insert"

use std::{
    collections::{HashMap, HashSet},
    hash::{DefaultHasher, Hash, Hasher},
    num::Wrapping,
    slice::Iter,
};

// pub struct ContextTree {}
use crate::{Context, FocusId, input::InputMode};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct KeyContextFlag(String);

impl From<&str> for KeyContextFlag {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct KeyContext(HashSet<KeyContextFlag>);

impl Hash for KeyContext {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let mut sum: Wrapping<u64> = Wrapping::default();

        for value in &self.0 {
            let mut hasher = DefaultHasher::new();
            Hash::hash(value, &mut hasher);
            sum += hasher.finish()
        }

        state.write_u64(sum.0);
    }
}

// impl<'a> IntoIterator for KeyContext {
//     type Item = &'a KeyContextFlag;
//     type IntoIter = std::collections::hash_set::Iter<'a, KeyContextFlag>;

//     fn into_iter(self) -> Self::IntoIter {
//         self.iter()
//     }
// }

// impl<'a, T, A: Allocator> IntoIterator for &'a Vec<T, A> {
//     type Item = &'a T;
//     type IntoIter = slice::Iter<'a, T>;

//     fn into_iter(self) -> Self::IntoIter {
//         self.iter()
//     }
// }

// impl<'a, T> IntoIterator for &'a Vec<T> {
//     type Item = &'a T;
//     type IntoIter = slice::Iter<'a, T>;

//     fn into_iter(self) -> slice::Iter<'a, T> { /* ... */ }
// }

impl KeyContext {
    // pub const NONE: Self = Self(HashSet::);

    pub fn empty() -> Self {
        Self::default()
    }

    // pub fn new() -> Self {

    // }

    pub fn iter(&self) -> std::collections::hash_set::Iter<'_, KeyContextFlag> {
        self.0.iter()
    }

    pub fn insert(&mut self, flag: impl Into<KeyContextFlag>) {
        self.0.insert(flag.into());
    }

    pub fn remove<'a>(&mut self, flag: impl Into<&'a KeyContextFlag>) {
        self.0.remove(flag.into());
    }

    pub fn has<'a>(&self, flag: impl Into<&'a KeyContextFlag>) -> bool {
        self.0.contains(flag.into())
    }

    // pub fn matches(&self, app_context: &Context) -> bool {
    //     for flag in self.0.iter() {
    //         if app_context.has_flag(flag) {
    //             continue;
    //         } else {
    //             return false;
    //         }
    //     }

    //     true
    // }
}

impl<I: Into<KeyContextFlag>> From<Vec<I>> for KeyContext {
    fn from(flags: Vec<I>) -> Self {
        Self(HashSet::from_iter(
            flags.into_iter().map(|flag| flag.into()),
        ))
    }
}
// #[derive(Debug, Default, PartialEq, Eq)]
// pub struct KeyContextPredicate {
//     pub focus: Option<FocusId>,
//     pub input_mode: Option<InputMode>,
// }

// impl KeyContextPredicate {
//     pub fn matches(&self, key_context: KeyContext) -> bool {
//         if let Some(focus) = self.focus
//             && focus != key_context.focus
//         {
//             false
//         } else if let Some(input_mode) = self.input_mode
//             && input_mode != key_context.input_mode
//         {
//             false
//         } else {
//             true
//         }
//     }
// }
