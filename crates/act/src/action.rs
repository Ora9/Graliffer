use std::{any::Any, fmt::Debug};

/// An action, that can be passed to a [`State`]
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

pub trait ActionDisplay {
    /// The namespace of an action
    ///
    /// # Naming convention
    /// - short
    /// - all lowercase
    ///
    /// E.g. : `frog`, `pond`, `campfire`..
    // fn namespace() -> &'static str;

    /// The "machine" name of an action, as in : used primarly by and for machine
    ///
    /// # Naming convention
    /// - short
    /// - UpperCamelCase
    ///
    /// E.g. : `Swim`, `PatPatFrog`, `LitFire`
    fn machine_name(&self) -> &'static str;

    /// The "human" name of an action, as in: used primarly for displaying to a human
    ///
    /// # Naming convention
    /// - may be longer and more contextualized
    /// - but should remain to the point and technical
    /// - all lowercase with spaces as word boudaries
    ///
    /// E.g. : `swim`, `pat pat the frog`, `lit fire`
    fn human_name(&self) -> &'static str;
}
