use std::{fmt::Display, ops::Deref, str::FromStr};

use crate::KeyContextFlag;

#[derive(Debug, Clone, Eq, Hash)]
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

    fn operands(&self) -> Option<(&KeyContextPredicate, &KeyContextPredicate)> {
        match self {
            Self::Flag(_) | Self::Not(_) | Self::None => None,
            Self::And(lhs, rhs) | Self::Or(lhs, rhs) | Self::Xor(lhs, rhs) => {
                Some((lhs.deref(), rhs.deref()))
            }
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

impl PartialEq for KeyContextPredicate {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Flag(lhs), Self::Flag(rhs)) => lhs == rhs,
            (Self::None, Self::None) => true,
            (Self::Not(lhs), Self::Not(rhs)) => lhs == rhs,
            (Self::And(l1, l2), Self::And(r1, r2))
            | (Self::Or(l1, l2), Self::Or(r1, r2))
            | (Self::Xor(l1, l2), Self::Xor(r1, r2)) => {
                // assure symmetry And(a, b) == And(b, a)
                (l1 == r1) && (l2 == r2) || (l1 == r2) && (l2 == r1)
            }
            _ => false,
        }
    }
}

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum KeyContextPredicateParseError {
    #[error(
        "missing operand for `{operation}` ({operation:#}) operation in `{source}`, expected {}", operation.arity()
    )]
    MissingOperand {
        r#source: String,
        operation: KeyContextPredicateOperation,
    },

    #[error("Missing operation in `{source}`, the predicate stack did not folded completely")]
    MissingOperation { r#source: String },
}

impl FromStr for KeyContextPredicate {
    type Err = KeyContextPredicateParseError;

    fn from_str(source: &str) -> Result<Self, Self::Err> {
        let mut stack: Vec<KeyContextPredicate> = Vec::new();

        let pop = |operation, stack: &mut Vec<KeyContextPredicate>| {
            stack
                .pop()
                .ok_or(KeyContextPredicateParseError::MissingOperand {
                    operation: operation,
                    source: source.to_string(),
                })
                .and_then(|flag| Ok(Box::new(flag)))
        };

        let operate = |operation, stack: &mut Vec<KeyContextPredicate>| {
            use KeyContextPredicateOperation::*;
            match operation {
                Not => {
                    let operand = pop(operation, stack)?;

                    stack.push(KeyContextPredicate::Not(operand));
                }
                And | Or | Xor => {
                    let rhs = pop(operation, stack)?;
                    let lhs = pop(operation, stack)?;

                    match operation {
                        And => stack.push(KeyContextPredicate::And(lhs, rhs)),
                        Or => stack.push(KeyContextPredicate::Or(lhs, rhs)),
                        Xor => stack.push(KeyContextPredicate::Xor(lhs, rhs)),
                        _ => unreachable!(),
                    };
                }
            }
            Ok(())
        };

        let parts = source.split_whitespace();
        for part in parts {
            if let Some(operation) = KeyContextPredicateOperation::from_str(part) {
                operate(operation, &mut stack)?;
            } else if part.trim().is_empty() {
                // Whitespace or empty string are ignored
                continue;
            } else {
                stack.push(KeyContextPredicate::from_flag(part));
            };
        }

        let mut stack = stack.into_iter();

        match (stack.next(), stack.next()) {
            (None, None) => Ok(Self::None),
            (Some(predicate), None) => Ok(predicate),
            _ => Err(KeyContextPredicateParseError::MissingOperation {
                source: source.to_string(),
            }),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
        assert_eq!(
            KeyContextPredicate::from_flag("A")
                .and(KeyContextPredicate::from_flag("B"))
                .to_string(),
            "A B &&"
        );

        assert_eq!(
            KeyContextPredicate::from_flag("A")
                .and(KeyContextPredicate::from_flag("B").not())
                .and(KeyContextPredicate::from_flag("C"))
                .to_string(),
            "A B ! && C &&"
        );
    }

    // fn parse(s: &str) -> KeyContextPredicate {
    //     KeyContextPredicate::from_str(s).unwrap()
    // }

    #[test]
    fn parse_flag() -> Result<(), KeyContextPredicateParseError> {
        assert_eq!(
            KeyContextPredicate::from_str("A")?,
            KeyContextPredicate::Flag(KeyContextFlag::from("A"))
        );

        Ok(())
    }

    #[test]
    fn parse_and() -> Result<(), KeyContextPredicateParseError> {
        assert_eq!(
            KeyContextPredicate::from_str("A B &&")?,
            KeyContextPredicate::from_flag("A").and(KeyContextPredicate::from_flag("B"))
        );

        Ok(())
    }

    #[test]
    fn parse_or() -> Result<(), KeyContextPredicateParseError> {
        assert_eq!(
            KeyContextPredicate::from_str("A B ||")?,
            KeyContextPredicate::from_flag("A").or(KeyContextPredicate::from_flag("B"))
        );

        Ok(())
    }

    #[test]
    fn parse_xor() -> Result<(), KeyContextPredicateParseError> {
        assert_eq!(
            KeyContextPredicate::from_str("A B ^^")?,
            KeyContextPredicate::from_flag("A").xor(KeyContextPredicate::from_flag("B"))
        );

        Ok(())
    }

    #[test]
    fn parse_not() -> Result<(), KeyContextPredicateParseError> {
        assert_eq!(
            KeyContextPredicate::from_str("A !")?,
            KeyContextPredicate::from_flag("A").not()
        );

        Ok(())
    }

    #[test]
    fn parse_empty() -> Result<(), KeyContextPredicateParseError> {
        assert_eq!(
            KeyContextPredicate::from_str("")?,
            KeyContextPredicate::None
        );

        Ok(())
    }

    #[test]
    fn parse_ignore_excess_whitespace() -> Result<(), KeyContextPredicateParseError> {
        assert_eq!(
            KeyContextPredicate::from_str("A   B &&")?,
            KeyContextPredicate::from_str("A B &&")?
        );

        assert_eq!(
            KeyContextPredicate::from_str("  A  B  &&  ")?,
            KeyContextPredicate::from_str("A B &&")?
        );

        Ok(())
    }

    #[test]
    fn parse_missing_operand_error() {
        assert_eq!(
            KeyContextPredicate::from_str("A &&"),
            Err(KeyContextPredicateParseError::MissingOperand {
                source: "A &&".to_string(),
                operation: KeyContextPredicateOperation::And
            })
        );

        assert_eq!(
            KeyContextPredicate::from_str("!"),
            Err(KeyContextPredicateParseError::MissingOperand {
                source: "!".to_string(),
                operation: KeyContextPredicateOperation::Not
            })
        );

        assert_eq!(
            KeyContextPredicate::from_str("A B ^^ ||"),
            Err(KeyContextPredicateParseError::MissingOperand {
                source: "A B ^^ ||".to_string(),
                operation: KeyContextPredicateOperation::Or
            })
        );
    }

    #[test]
    fn parse_missing_operation_error() {
        assert_eq!(
            KeyContextPredicate::from_str("A A"),
            Err(KeyContextPredicateParseError::MissingOperation {
                source: "A A".to_string()
            })
        );

        assert_eq!(
            KeyContextPredicate::from_str("A A !"),
            Err(KeyContextPredicateParseError::MissingOperation {
                source: "A A !".to_string()
            })
        );
    }

    #[test]
    fn parse_order() -> Result<(), KeyContextPredicateParseError> {
        assert_eq!(
            KeyContextPredicate::from_str("A B C && &&")?,
            KeyContextPredicate::from_str("B C && A &&")?,
        );

        Ok(())
    }

    #[test]
    fn parse_lot() -> Result<(), KeyContextPredicateParseError> {
        assert_eq!(
            KeyContextPredicate::from_str("we are going to the beach today && && && && && &&")?,
            KeyContextPredicate::from_flag("we").and(
                KeyContextPredicate::from_flag("are").and(
                    KeyContextPredicate::from_flag("going").and(
                        KeyContextPredicate::from_flag("to").and(
                            KeyContextPredicate::from_flag("the").and(
                                KeyContextPredicate::from_flag("beach")
                                    .and(KeyContextPredicate::from_flag("today"))
                            )
                        )
                    )
                )
            )
        );

        Ok(())
    }

    #[test]
    fn parse_display_roundtrip() -> Result<(), KeyContextPredicateParseError> {
        let a = "just watch && the ! || sky ||";

        assert_eq!(KeyContextPredicate::from_str(a)?.to_string(), a);

        assert_eq!(
            KeyContextPredicate::from_str(&KeyContextPredicate::from_str(a)?.to_string())?
                .to_string(),
            a
        );

        Ok(())
    }

    #[test]
    fn eq() -> Result<(), KeyContextPredicateParseError> {
        assert_eq!(
            KeyContextPredicate::from_str("A B &&")?,
            KeyContextPredicate::from_str("A B &&")?,
        );
        Ok(())
    }

    #[test]
    fn eq_symmetry() -> Result<(), KeyContextPredicateParseError> {
        assert_eq!(
            KeyContextPredicate::from_str("A B &&")?,
            KeyContextPredicate::from_str("B A &&")?,
        );
        Ok(())
    }
}
