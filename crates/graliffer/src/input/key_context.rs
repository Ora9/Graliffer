use std::{
    collections::HashMap,
    fmt::Display,
    hash::{DefaultHasher, Hash, Hasher},
    num::Wrapping,
};

use crate::KeyContextPredicate;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct KeyContextFlagKeyHash(u64);

impl KeyContextFlagKeyHash {
    pub fn new() -> Self {
        Self(rand::random::<u64>())
    }
}

impl From<&str> for KeyContextFlagKeyHash {
    fn from(value: &str) -> Self {
        let mut hasher = DefaultHasher::new();
        Hash::hash(value, &mut hasher);
        Self(hasher.finish())
    }
}

impl From<String> for KeyContextFlagKeyHash {
    fn from(value: String) -> Self {
        value.as_str().into()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum KeyContextFlagKey {
    Focus,
    InputMode,
    Hash(KeyContextFlagKeyHash),
}

impl From<&str> for KeyContextFlagKey {
    fn from(value: &str) -> Self {
        Self::Hash(KeyContextFlagKeyHash::from(value))
    }
}

impl Default for KeyContextFlagKey {
    fn default() -> Self {
        Self::Hash(KeyContextFlagKeyHash::new())
    }
}

// pub type KeyContextFlag = String;
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct KeyContextFlag(String);

impl From<&str> for KeyContextFlag {
    /// Get Self from a `&str`
    ///
    /// # Validity
    /// There are some "invalid" inputs, some examples :
    /// - "&&", "!" .. (predicate operators)
    /// - "", " " (whitespace or empty)
    ///
    /// But it would be annoying to validate it with `TryFrom`, so idk just avoid using these string..
    fn from(value: &str) -> Self {
        KeyContextFlag(value.to_string())
    }
}

impl From<String> for KeyContextFlag {
    /// Get Self from a `&str`
    ///
    /// # Validity
    /// There are some "invalid" inputs, some examples :
    /// - "&&", "!" .. (predicate operators)
    /// - "", " " (whitespace or empty)
    ///
    /// But it would be annoying to validate it with `TryFrom`, so idk just avoid using these string..
    fn from(value: String) -> Self {
        KeyContextFlag(value)
    }
}

impl Display for KeyContextFlag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct KeyContext(HashMap<KeyContextFlagKey, KeyContextFlag>);

impl Hash for KeyContext {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let mut sum: Wrapping<u64> = Wrapping::default();

        for (key, flag) in &self.0 {
            let mut hasher = DefaultHasher::new();
            Hash::hash(key, &mut hasher);
            Hash::hash(flag, &mut hasher);
            sum += hasher.finish()
        }

        state.write_u64(sum.0);
    }
}

impl KeyContext {
    pub fn empty() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, flag: KeyContextFlag) {
        self.0.insert(KeyContextFlagKey::default(), flag);
    }

    pub fn remove(&mut self, flag: &KeyContextFlag) {
        self.0.retain(|_, value| value != flag);
    }

    pub fn has(&self, flag: &KeyContextFlag) -> bool {
        self.0.iter().find(|(_, value)| *value == flag).is_some()
    }

    pub fn insert_with_key(&mut self, key: KeyContextFlagKey, flag: KeyContextFlag) {
        self.0.insert(key, flag);
    }

    pub fn remove_with_key(&mut self, key: &KeyContextFlagKey) {
        self.0.remove(key);
    }

    pub fn has_with_key(&self, key: &KeyContextFlagKey, flag: &KeyContextFlag) -> bool {
        self.0
            .get(key)
            .and_then(|value| Some(value == flag))
            .unwrap_or(false)
    }

    pub fn matches(&self, predicate: &KeyContextPredicate) -> bool {
        use crate::KeyContextPredicate::*;

        match predicate {
            None => true,
            Flag(flag) => self.has(&flag.clone().into()),
            Not(predicate) => !self.matches(predicate),
            And(lhs, rhs) => self.matches(lhs) && self.matches(rhs),
            Or(lhs, rhs) => self.matches(lhs) || self.matches(rhs),
            Xor(lhs, rhs) => self.matches(lhs) ^ self.matches(rhs),
        }
    }
}
