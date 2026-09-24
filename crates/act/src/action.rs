use std::{any::Any, fmt::Debug};

/// An action that can be passed to a [`State`]
///
/// # Example
///
/// Simple unit-only enum :
/// ```
/// #[derive(Debug, Clone)]
/// enum DuckAction {
///     Play,
///     Quack,
///     Walk,
///     Swim,
/// }
///
/// impl act::Action for DuckAction {};
/// ```
///
/// Parameterized actions :
/// ```
/// # #[derive(Debug, Clone)]
/// # struct Frog;
/// /// A stack of frogs
/// #[derive(Debug, Clone)]
/// enum FrogStackAction {
///     Push(Frog),
///     PopLast,
/// }
///
/// impl act::Action for FrogStackAction {};
/// ```
pub trait Action: Clone + Debug {}

pub trait ActionDisplay {
    // /// The namespace of an action
    // ///
    // /// # Naming convention
    // /// - short
    // /// - all lowercase
    // ///
    // /// E.g. : `frog`, `pond`, `campfire`..
    // fn namespace() -> &'static str;

    /// The "machine" name of an action, as in : used primarly by and for machine
    ///
    /// # Naming convention
    /// - short
    /// - `UpperCamelCase`
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
