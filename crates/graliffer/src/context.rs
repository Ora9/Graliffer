use std::{cell::RefCell, rc::Rc};

use crate::{
    Config, KeyContextFlag, KeyContextFlagKey, KeyContextPredicate, ViewId,
    input::{InputMode, KeyContext},
};

#[derive(Debug)]
pub struct ContextInner {
    pub config: Config,

    focus: ViewId,
    input_mode: InputMode,

    key_context: KeyContext,
}

#[derive(Debug, Clone)]
pub struct Context(Rc<RefCell<ContextInner>>);

impl Context {
    pub fn new(config: Config, default_focus: ViewId) -> Self {
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

    // pub fn config<O>(&self, reader: impl FnOnce(&Config) -> O) -> O {
    //     reader(&self.0.borrow().config)
    // }

    pub fn read<O>(&self, reader: impl FnOnce(&ContextInner) -> O) -> O {
        reader(&self.0.borrow())
    }

    pub fn write<O>(&mut self, writer: impl FnOnce(&mut ContextInner) -> O) -> O {
        writer(&mut self.0.borrow_mut())
    }

    pub fn get_input_mode(&self) -> InputMode {
        self.0.borrow().input_mode
    }

    pub fn set_input_mode(&mut self, input_mode: InputMode) {
        self.0.borrow_mut().input_mode = input_mode;
        self.set_input_mode_flag(input_mode);
    }

    fn set_input_mode_flag(&mut self, input_mode: InputMode) {
        self.insert_flag_with_key(KeyContextFlagKey::InputMode, input_mode.to_string());
    }

    pub fn get_focus(&self) -> ViewId {
        self.0.borrow().focus.clone()
    }

    pub fn set_focus(&mut self, focus: impl Into<ViewId>) {
        let focus = focus.into();

        self.0.borrow_mut().focus = focus.clone();
        self.set_focus_flag(focus);
    }

    fn set_focus_flag(&mut self, focus: ViewId) {
        self.insert_flag_with_key(KeyContextFlagKey::Focus, focus.to_string());
    }

    pub fn insert_flag(&mut self, flag: impl Into<KeyContextFlag>) {
        self.0.borrow_mut().key_context.insert(flag.into());
    }

    pub fn remove_flag(&mut self, flag: impl Into<KeyContextFlag>) {
        self.0.borrow_mut().key_context.remove(&flag.into());
    }

    pub fn has_flag(&self, flag: impl Into<KeyContextFlag>) -> bool {
        self.0.borrow().key_context.has(&flag.into())
    }

    pub fn insert_flag_with_key(
        &mut self,
        key: impl Into<KeyContextFlagKey>,
        flag: impl Into<KeyContextFlag>,
    ) {
        self.0
            .borrow_mut()
            .key_context
            .insert_with_key(key.into(), flag.into());
    }

    pub fn remove_flag_with_key(&mut self, key: impl Into<KeyContextFlagKey>) {
        self.0.borrow_mut().key_context.remove_with_key(&key.into());
    }

    pub fn has_flag_with_key(
        &self,
        key: impl Into<KeyContextFlagKey>,
        flag: impl Into<KeyContextFlag>,
    ) -> bool {
        self.0
            .borrow()
            .key_context
            .has_with_key(&key.into(), &flag.into())
    }

    pub fn matches_key_context(&self, predicate: &KeyContextPredicate) -> bool {
        self.0.borrow().key_context.matches(predicate)
    }
}
