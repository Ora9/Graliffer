use std::{cell::RefCell, rc::Rc};

use crate::{
    ViewId,
    config::Config,
    input::{InputMode, KeyContext, KeyContextFlag, KeyContextPredicate, Keymap},
};

#[derive(Debug)]
struct ContextInner {
    config: Config,
    keymap: Keymap,

    key_context: KeyContext,
}

/// Context passed to views
///
/// It exposes :
///  - [`KeyContext`], with state like [`Focus`](ViewId), or [`InputMode`]
///  - [`Config`]
///  - [`Keymap`]
///
/// Cheaply cloned, clones always refers to the same mutable data (using refcounting internally).
#[derive(Debug, Clone)]
pub struct Context(Rc<RefCell<ContextInner>>);

impl Context {
    /// Create a `Context`
    pub(crate) fn new(config: Config, keymap: Keymap, focus: ViewId) -> Self {
        Self(Rc::new(RefCell::new(ContextInner {
            config,
            keymap,

            key_context: KeyContext::new(focus, InputMode::default()),
        })))
    }

    /// Read only access to [`ContextInner`]
    fn read<O>(&self, reader: impl FnOnce(&ContextInner) -> O) -> O {
        reader(&self.0.borrow())
    }

    /// Read and write access [`ContextInner`]
    fn write<O>(&mut self, writer: impl FnOnce(&mut ContextInner) -> O) -> O {
        writer(&mut self.0.borrow_mut())
    }
}

/// # Config
impl Context {
    /// Read only access to the [config](Config)
    pub fn config<O>(&self, reader: impl FnOnce(&Config) -> O) -> O {
        self.read(|ctx| reader(&ctx.config))
    }
}

/// # Keymap
impl Context {
    /// Read only access to the [keymap](Keymap)
    pub fn keymap<O>(&self, reader: impl FnOnce(&Keymap) -> O) -> O {
        self.read(|ctx| reader(&ctx.keymap))
    }

    /// Read only access to both the [keymap](Keymap) and the [key context](KeyContext)
    pub fn keys<O>(&self, reader: impl FnOnce(&Keymap, &KeyContext) -> O) -> O {
        self.read(|ctx| reader(&ctx.keymap, &ctx.key_context))
    }
}

/// # Key context
impl Context {
    /// Read only access to [key context](KeyContext)
    pub fn key_context<O>(&self, reader: impl FnOnce(&KeyContext) -> O) -> O {
        self.read(|ctx| reader(&ctx.key_context))
    }

    /// Read and write access to [key context](KeyContext)
    pub fn key_context_mut<O>(&mut self, writer: impl FnOnce(&mut KeyContext) -> O) -> O {
        self.write(|ctx| writer(&mut ctx.key_context))
    }

    /// Does the given `predicate` matches the current key context
    pub fn matches_key_context(&self, predicate: &KeyContextPredicate) -> bool {
        self.key_context(|key_context| key_context.matches(predicate))
    }

    /// Insert the given `flag` in the key context
    ///
    /// Note: see [`KeyContextFlag`] for flag naming guidelines
    pub fn insert_flag(&mut self, flag: impl Into<KeyContextFlag>) {
        self.key_context_mut(|key_context| key_context.insert(flag.into()));
    }

    /// Remove the given `flag` from the key context
    pub fn remove_flag(&mut self, flag: impl Into<KeyContextFlag>) {
        self.key_context_mut(|key_context| key_context.remove(&flag.into()));
    }

    /// Does the current key context contains the given `flag`
    pub fn has_flag(&self, flag: impl Into<KeyContextFlag>) -> bool {
        self.key_context(|key_context| key_context.has(&flag.into()))
    }
}

/// # Input mode
impl Context {
    /// Current [`InputMode`]
    pub fn input_mode(&self) -> InputMode {
        self.read(|ctx| ctx.key_context.input_mode())
    }

    /// Set the [`InputMode`]
    pub fn set_input_mode(&mut self, input_mode: InputMode) {
        self.write(|ctx| ctx.key_context.set_input_mode(input_mode));
    }
}

/// # Focus
impl Context {
    /// Currently focused [`ViewId`]
    pub fn focus(&self) -> ViewId {
        self.read(|ctx| ctx.key_context.focus())
    }

    /// Set the focused [`ViewId`]
    pub fn set_focus(&mut self, focus: impl Into<ViewId>) {
        self.write(|ctx| ctx.key_context.set_focus(focus.into()));
    }
}
