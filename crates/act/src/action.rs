use std::{any::Any, fmt::Debug};

/// Collection of action
///
/// # Example
///
/// Simple unit-only enum :
/// ```
/// # use act::Action;
/// #[derive(Debug, Clone)]
/// enum DuckAction {
///     Play,
///     Quack,
///     Walk,
///     Swim,
/// }
///
/// impl Action for DuckAction {};
/// ```
///
/// Parameterized actions :
/// ```
/// # use act::Action;
/// # #[derive(Debug, Clone)]
/// # struct Frog;
/// /// A stack of frogs
/// #[derive(Debug, Clone)]
/// enum FrogStackAction {
///     Push(Frog),
///     PopLast,
/// }
///
/// impl Action for FrogStackAction {};
/// ```
pub trait Action: Any + ActionClone + Debug {}

pub trait ActionClone {
    fn dyn_clone(&self) -> Box<dyn Action>;
}

impl<T: Clone + Action> ActionClone for T {
    fn dyn_clone(&self) -> Box<dyn Action> {
        Box::new(self.clone())
    }
}

impl Action for Box<dyn Action> {}

impl Clone for Box<dyn Action> {
    fn clone(&self) -> Self {
        (**self).dyn_clone()
    }
}
