use std::fmt::Debug;

use crate::Action;

/// The `State` trait define how actions affect the underlying state
///
/// An alternative [`TimelinedState`](crate::timeline::TimelinedState) exists to allow for
/// revertable actions
///
/// # Example
///
/// A simple stack
/// ```
/// struct Frog {
///     name: String
/// };
///
/// struct FrogStack(Vec<Frog>);
///
/// #[derive(act::Action, Debug, Clone)]
/// enum FrogStackAction {
///     Push(Frog),
///     Pop,
/// }
///
/// enum FrogStackError {
///     NothingToPop,
///     WouldOverflow,
/// }
///
/// impl State for FrogStack {
///     type Action = FrogStackAction;
///     type Error = FrogStackError;
///
///     fn act(&mut self, action: impl Into<Self::Action>) -> Result<(), Self::Error> {
///         match action.into() {
///             FrogStackAction::Push(frog) => {
///                 self.0.push(frog)
///             }
///             FrogStackAction::Pop() => {
///                 self.0.pop().unwrap()
///             }
///         }
///         Ok(())
///     }
/// }
///
/// ```
pub trait State: Debug {
    type Action: Action + Clone;
    type Error;

    fn act(&mut self, action: impl Into<Self::Action>) -> Result<(), Self::Error>;
}
