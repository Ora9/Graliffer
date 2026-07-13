use std::{fmt::Display, str::FromStr};

use crate::KeyContextFlag;

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
    fn operation(&self) -> Option<KeyContextPredicateOperation> {
        match self {
            Self::Flag(_) | Self::None => None,
            Self::And(_, _) => Some(KeyContextPredicateOperation::And),
            Self::Or(_, _) => Some(KeyContextPredicateOperation::Or),
            Self::Xor(_, _) => Some(KeyContextPredicateOperation::Xor),
            KeyContextPredicate::Not(_) => Some(KeyContextPredicateOperation::Not),
        }
    }

    // fn lhs(&self) -> KeyContextPredicate {
    //     // use KeyContextPredicate::*;
    //     // match self {
    //     //     Self::Flag(_) =>
    //     // }
    // }
    pub fn from_flag(flag: impl Into<KeyContextFlag>) -> Self {
        Self::Flag(flag.into())
    }

    pub fn and(self, other: KeyContextPredicate) -> Self {
        Self::And(Box::new(self), Box::new(other))
    }

    pub fn or(self, other: KeyContextPredicate) -> Self {
        Self::Or(Box::new(self), Box::new(other))
    }

    pub fn xor(self, other: KeyContextPredicate) -> Self {
        Self::Xor(Box::new(self), Box::new(other))
    }

    pub fn not(self) -> Self {
        Self::Not(Box::new(self))
    }
}

impl Display for KeyContextPredicate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => {
                write!(f, "")
            }
            Self::Flag(flag) => {
                write!(f, "{flag}")
            }
            Self::And(lhs, rhs) | Self::Or(lhs, rhs) | Self::Xor(lhs, rhs) => {
                // SAFETY : these operations have a valid operation representation (`&&`, `||`, `^^`)
                write!(f, "{lhs} {rhs} {:#}", self.operation().unwrap())
            }
            Self::Not(predicate) => {
                // SAFETY : not have a valid operation representation (`!`)
                write!(f, "{predicate} {:#}", self.operation().unwrap())
            }
        }
    }
}

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

impl FromStr for KeyContextPredicate {
    type Err = KeyContextPredicateParseError;

    fn from_str(source: &str) -> Result<Self, Self::Err> {
        let mut predicate: Vec<KeyContextPredicate> = Vec::new();

        let pop = |operation: KeyContextPredicateOperation,
                   stack: &mut Vec<KeyContextPredicate>| {
            match stack.pop() {
                None => Err(KeyContextPredicateParseError::NotEnoughOperand {
                    operation: operation,
                    predicate: source.to_string(),
                }),
                Some(flag) => Ok(Box::new(flag)),
            }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display() {
        let a = KeyContextPredicate::from_flag("someflag")
            .not()
            .and(KeyContextPredicate::from_flag("otherflag"))
            .and(KeyContextPredicate::from_flag("lastflag").not());

        assert_eq!(a.to_string(), "someflag ! otherflag && lastflag ! &&");
    }
}
