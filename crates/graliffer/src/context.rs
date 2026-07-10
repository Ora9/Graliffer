use std::cell::RefCell;

use log::debug;

use crate::{
    FocusId, KeyContextFlag, KeyContextFlagKey, KeyContextPredicate,
    input::{InputMode, KeyContext},
};

#[derive(Debug, Clone)]
pub struct ContextInner {
    focus: FocusId,
    input_mode: InputMode,

    key_context: KeyContext,
}

#[derive(Debug, Clone)]
pub struct Context(RefCell<ContextInner>);

impl Context {
    pub fn new(focus_id: impl Into<FocusId>, input_mode: InputMode) -> Self {
        let focus_id = focus_id.into();

        let mut context = Self(RefCell::new(ContextInner {
            focus: focus_id,
            input_mode,

            key_context: KeyContext::default(),
        }));

        context.set_focus_flag(focus_id);
        context.set_input_mode_flag(input_mode);

        context
    }

    pub fn get_input_mode(&self) -> InputMode {
        self.0.borrow().input_mode
    }

    pub fn set_input_mode(&mut self, input_mode: InputMode) {
        self.0.get_mut().input_mode = input_mode;
        self.set_input_mode_flag(input_mode);
    }

    fn set_input_mode_flag(&mut self, input_mode: InputMode) {
        self.insert_flag_with_key(KeyContextFlagKey::InputMode, input_mode.to_string());
    }

    pub fn get_focus(&self) -> FocusId {
        self.0.borrow().focus
    }

    pub fn set_focus(&mut self, focus_id: impl Into<FocusId>) {
        let focus_id = focus_id.into();

        self.0.get_mut().focus = focus_id;
        self.set_focus_flag(focus_id);
    }

    fn set_focus_flag(&mut self, focus_id: FocusId) {
        self.insert_flag_with_key(KeyContextFlagKey::Focus, focus_id.to_string());
    }

    pub fn insert_flag(&mut self, flag: KeyContextFlag) {
        self.0.get_mut().key_context.insert(flag);
    }

    pub fn remove_flag(&mut self, flag: &KeyContextFlag) {
        self.0.get_mut().key_context.remove(&flag);
    }

    pub fn has_flag(&self, flag: &KeyContextFlag) -> bool {
        self.0.borrow().key_context.has(flag)
    }

    pub fn insert_flag_with_key(
        &mut self,
        key: impl Into<KeyContextFlagKey>,
        flag: KeyContextFlag,
    ) {
        self.0
            .get_mut()
            .key_context
            .insert_with_key(key.into(), flag);
    }

    pub fn remove_flag_with_key(&mut self, key: impl Into<KeyContextFlagKey>) {
        self.0.get_mut().key_context.remove_with_key(&key.into());
    }

    pub fn has_flag_with_key(
        &self,
        key: impl Into<KeyContextFlagKey>,
        flag: &KeyContextFlag,
    ) -> bool {
        self.0.borrow().key_context.has_with_key(&key.into(), flag)
    }

    pub fn matches_key_context(&self, predicate: &KeyContextPredicate) -> bool {
        self.0.borrow().key_context.matches(predicate)

        // for flag in key_context.iter() {
        //     if self.has_flag(flag) {
        //         continue;
        //     } else {
        //         return false;
        //     }
        // }

        // true
    }
}
