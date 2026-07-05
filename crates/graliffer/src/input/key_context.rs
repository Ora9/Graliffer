use std::{
    collections::{HashMap, HashSet},
    error,
    fmt::Display,
    hash::{DefaultHasher, Hash, Hasher},
    num::Wrapping,
    slice::Iter,
};

use log::debug;

use crate::{Context, FocusId, KeyContextPredicate::Flag, input::InputMode};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct KeyContextFlag(String);

impl KeyContextFlag {
    pub fn new(flag: &str) -> Self {
        Self(flag.to_string())
    }
}

impl From<&str> for KeyContextFlag {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

impl From<String> for KeyContextFlag {
    fn from(value: String) -> Self {
        Self::new(&value)
    }
}

// impl From<&String> for KeyContextFlag {
//     fn from(value: &String) -> &Self {
//         Self(*value)
//     }
// }

#[derive(Debug, thiserror::Error)]
pub enum KeyContextPredicateParseError {
    #[error(
        "not enough operands for `{operation}` ({operation:#}) operation in `{predicate}`, expected {}, found 0", operation.arity()
    )]
    NotEnoughOperand {
        predicate: String,
        operation: KeyContextPredicateOperation,
    },

    #[error("too much operands, not enough operations in `{predicate}`")]
    TooMuchOperandNotEnoughOperations { predicate: String },
}

#[derive(Debug, Clone, Copy)]
pub enum KeyContextPredicateOperation {
    And,
    Or,
    Xor,
    Not,
}

impl KeyContextPredicateOperation {
    pub fn from_str(value: &str) -> Option<Self> {
        match value {
            "&&" => Some(Self::And),
            "||" => Some(Self::Or),
            "^^" => Some(Self::Xor),
            "!" => Some(Self::Not),
            _ => None,
        }
    }

    pub fn name(&self) -> String {
        match self {
            Self::And => "And",
            Self::Or => "Or",
            Self::Xor => "Xor",
            Self::Not => "Not",
        }
        .to_string()
    }

    pub fn symbol(&self) -> String {
        match self {
            Self::And => "&&",
            Self::Or => "||",
            Self::Xor => "^^",
            Self::Not => "!",
        }
        .to_string()
    }

    pub fn arity(&self) -> usize {
        match self {
            Self::And | Self::Or | Self::Xor => 2,
            Self::Not => 1,
        }
    }
}

impl Display for KeyContextPredicateOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            f.write_str(&self.symbol())
        } else {
            f.write_str(&self.name())
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum KeyContextPredicate {
    None,
    Flag(KeyContextFlag),
    And(Box<KeyContextPredicate>, Box<KeyContextPredicate>),
    Or(Box<KeyContextPredicate>, Box<KeyContextPredicate>),
    Xor(Box<KeyContextPredicate>, Box<KeyContextPredicate>),
    Not(Box<KeyContextPredicate>),
}

impl KeyContextPredicate {
    pub fn parse(source: &str) -> Result<Self, KeyContextPredicateParseError> {
        let mut predicate: Vec<KeyContextPredicate> = Vec::new();

        let mut pop = |operation: KeyContextPredicateOperation,
                       stack: &mut Vec<KeyContextPredicate>| match stack.pop()
        {
            None => Err(KeyContextPredicateParseError::NotEnoughOperand {
                operation: operation,
                predicate: source.to_string(),
            }),
            Some(flag) => Ok(Box::new(flag)),
        };

        let parts = source.split_whitespace();
        for part in parts {
            let to_push = if let Some(operation) = KeyContextPredicateOperation::from_str(part) {
                use KeyContextPredicateOperation::*;
                match operation {
                    Not => {
                        let operand = pop(operation, &mut predicate)?;

                        KeyContextPredicate::Not(operand)
                    }
                    And | Or | Xor => {
                        let lhs = pop(operation, &mut predicate)?;
                        let rhs = pop(operation, &mut predicate)?;

                        match operation {
                            And => KeyContextPredicate::And(lhs, rhs),
                            Or => KeyContextPredicate::Or(lhs, rhs),
                            Xor => KeyContextPredicate::Xor(lhs, rhs),
                            _ => unreachable!(),
                        }
                    }
                }
            } else {
                KeyContextPredicate::Flag(part.into())
            };

            predicate.push(to_push);
        }

        if predicate.len() > 1 {
            Err(
                KeyContextPredicateParseError::TooMuchOperandNotEnoughOperations {
                    predicate: source.to_string(),
                },
            )
        } else {
            match predicate.pop() {
                None => Ok(Self::None),
                Some(predicate) => Ok(predicate),
            }
        }
    }
}

// impl TryFrom<&str> for Option<KeyContextPredicate> {
//     type Error = KeyContextPredicateParseError;

//     fn try_from(value: &str) -> Result<Self, Self::Error> {
//         Self::parse(value)
//     }
// }

#[derive(Debug, Default, Clone, PartialEq, Eq)]
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

impl KeyContext {
    pub fn empty() -> Self {
        Self::default()
    }

    pub fn iter(&self) -> std::collections::hash_set::Iter<'_, KeyContextFlag> {
        self.0.iter()
    }

    pub fn insert(&mut self, flag: KeyContextFlag) {
        self.0.insert(flag);
        debug!("{self:?}");
    }

    pub fn remove<'a>(&mut self, flag: &KeyContextFlag) {
        self.0.remove(flag);
    }

    pub fn has<'a>(&self, flag: &KeyContextFlag) -> bool {
        self.0.contains(flag)
    }

    pub fn matches(&self, predicate: &KeyContextPredicate) -> bool {
        use KeyContextPredicate::*;

        match predicate {
            None => true,
            Flag(KeyContextFlag(flag)) => self.has(&flag.clone().into()),
            Not(predicate) => !self.matches(predicate),
            And(lhs, rhs) => self.matches(lhs) && self.matches(rhs),
            Or(lhs, rhs) => self.matches(lhs) || self.matches(rhs),
            Xor(lhs, rhs) => self.matches(lhs) ^ self.matches(rhs),
        }
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
