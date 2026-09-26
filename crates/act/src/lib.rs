//! `act` is an action based state mutation system
//!
//! ## Features
//! - Centralized interface for state mutation
//!     - Contained and reproducible state
//!     - (De)serialisation (e.g. through [`ActionDisplay`] and [`ActionFromStr`]) for later use or
//!       configuration
//! - Nested state and action
//!     - "Centralized" does not mean huge or "do it all", see (TODO: pond example link)
//! - Keeping track of a timeline
//!     - History reenactment
//!     - Undo/redo possibilities (with [`Timeline`](timeline::Timeline))
//!     - Each action can have a reciprocal (action that can undo what has been done)
//!
//! ## Use cases
//! In a user interface :
//!  - Actions can be exposed to users in a command picker
//!  - A keymap can bind keystroke to actions
//!
//! TODO: link some code using act in prod
//!
//! In an text input system :
//!  - Selective undoable, not-undoable actions (like cursor movements)
//!  - Insertion and deletion, pasting, cursor movement
//!
//! ## This crate exposes :
//!  - The [`Action`] trait, implemented on items that represent an action performed on a state
//!  - Two similar state traits, with one method `act` defining how actions affect the underlying state
//!     - [`State`], where [`act`](State::act) performs the action and returns no value
//!     - [`TimelinedState`](timeline::TimelinedState), where [`act`](timeline::TimelinedState::act)
//!       performs the action then returns a reciprocal of that action ([`Revert`](timeline::Revert),
//!       that is, an other action that would serve as an undo if performed)
//!  - The [`Timeline`](timeline::Timeline) struct holding both a
//!    [`TimelinedState`](timeline::TimelinedState) and an history of all actions performed on it.
//!    It can be used to hold any [state](timeline::TimelinedState) that
//!    would need undo/redo capabilities, and act through this struct to register actions
//!
//! [`timeline`] module is gated behind the `timeline` crate feature
//!
//! ## Limitations
//!
//! This crate's design has mostly been shaped for my own usage, as such :
//!  - Not async (i never really used async in rust, but might add it sometime in the future,
//!    especialy because it might be used thread safe mutation state mutation system
//!  - [`Action`] is not dyn-compatible (but it was at some point in the past, i did not made any
//!    effort to keep it that way..)
//!  - Some APIs might be weird (e.g. `human` vs `machine` distinction [`ActionDisplay`])
//!
//! Some things are fragile and error prone :
//!  - [`State::act`] returns nothing, it is write only
//!     - e.g. if we have a stack and want to pop it and get that popped item, we have to do it in
//!       2 calls, one to read the item about to get popped, then one to pop and discard,
//!       unlike Vec::pop(), this design allow for error (e.g. not the right order of operation,
//!       not agreeing on the meaning of pop, and if we add levels of indirection with nested state
//!       and action, the two call might not point to the same state..)
//!  - The reciprocity of an action is defined in [`TimelinedState`](timeline::TimelinedState).
//!    Soundness is not enforced by the system, if the returned [`Revert`](timeline::Revert) only
//!    partially or wrongly restore the state, undoing could corrupt the state, and any subsequent
//!    actions would base on it, and continue to diverge the state from what it should be
//!  - State mutation by other means might corrupt the state in a similar way
//!  - [`TimelinedState`](timeline::TimelinedState) is allowed to return [`Revert::None`](timeline::Revert::None) (e.g. cursor movements in a input, may not be )

mod action;
pub use action::{Action, ActionDisplay};

mod state;
pub use state::State;

#[cfg(feature = "timeline")]
pub mod timeline;

#[cfg(feature = "derive")]
pub use act_macros::ActionDisplay;
