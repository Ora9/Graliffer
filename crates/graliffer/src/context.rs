use std::{cell::RefCell, rc::Rc};

use crate::{
    Config, KeyContextFlag, KeyContextFlagKey, KeyContextPredicate, ViewId,
    input::{InputMode, KeyContext},
};

#[derive(Debug)]
pub struct ContextInner {
    config: Config,

    focus: ViewId,
    input_mode: InputMode,

    key_context: KeyContext,
}

/// Context passed to views
///
/// Contains [`Context`], and state like [`Focus`](ViewId), [`InputMode`] and [`KeyContext`]
///
/// Cheaply cloned, clones always refers to the same mutable data (using refcounting internally).
#[derive(Debug, Clone)]
pub struct Context(Rc<RefCell<ContextInner>>);

impl Context {
    /// Create a `Context`
    pub(crate) fn new(config: Config, default_focus: ViewId) -> Self {
        let input_mode = InputMode::default();

        let mut context = Self(Rc::new(RefCell::new(ContextInner {
            focus: default_focus.clone(),
            input_mode,

            config,

            key_context: KeyContext::default(),
        })));

        context.set_focus_flag(default_focus);
        context.set_input_mode_flag(input_mode);

        context
    }

    /// Read only access to the config
    pub fn config<O>(&self, reader: impl FnOnce(&Config) -> O) -> O {
        reader(&self.0.borrow().config)
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

/// # Input mode
impl Context {
    /// Current [`InputMode`]
    pub fn input_mode(&self) -> InputMode {
        self.read(|ctx| ctx.input_mode)
    }

    /// Set the [`InputMode`]
    pub fn set_input_mode(&mut self, input_mode: InputMode) {
        self.write(|ctx| ctx.input_mode = input_mode);
        self.set_input_mode_flag(input_mode);
    }

    fn set_input_mode_flag(&mut self, input_mode: InputMode) {
        self.insert_flag_with_key(KeyContextFlagKey::InputMode, input_mode.to_string());
    }
}

/// # Focus
impl Context {
    /// Currently focused [`ViewId`]
    pub fn focus(&self) -> ViewId {
        self.read(|ctx| ctx.focus.clone())
    }

    /// Set the focused [`ViewId`]
    pub fn set_focus(&mut self, focus: impl Into<ViewId>) {
        let focus = focus.into();

        self.write(|ctx| ctx.focus = focus.clone());
        self.set_focus_flag(focus);
    }

    fn set_focus_flag(&mut self, focus: ViewId) {
        self.insert_flag_with_key(KeyContextFlagKey::Focus, focus.to_string());
    }
}

/// # Key context flag
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

    /// Insert the given `flag` in the key context with `key`
    pub fn insert_flag_with_key(
        &mut self,
        key: impl Into<KeyContextFlagKey>,
        flag: impl Into<KeyContextFlag>,
    ) {
        self.key_context_mut(|key_context| key_context.insert_with_key(key.into(), flag.into()));
    }

    /// Remove the flag associated to `key` from the key context
    pub fn remove_flag_with_key(&mut self, key: impl Into<KeyContextFlagKey>) {
        self.key_context_mut(|key_context| key_context.remove_with_key(&key.into()));
    }

    /// Does the key context contains the given `key` and `flag` association
    pub fn has_flag_with_key(
        &self,
        key: impl Into<KeyContextFlagKey>,
        flag: impl Into<KeyContextFlag>,
    ) -> bool {
        self.key_context(|key_context| key_context.has_with_key(&key.into(), &flag.into()))
    }
}
