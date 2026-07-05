use std::cell::RefCell;

use crate::{
    FocusId, KeyContextFlag, KeyContextPredicate,
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

        context
    }

    pub fn get_input_mode(&self) -> InputMode {
        self.0.borrow().input_mode
    }

    pub fn set_input_mode(&mut self, input_mode: InputMode) {
        self.0.get_mut().input_mode = input_mode
    }

    pub fn get_focus(&self) -> FocusId {
        self.0.borrow().focus
    }

    pub fn set_focus(&mut self, focus_id: impl Into<FocusId>) {
        let focus_id = focus_id.into();

        self.set_focus_flag(focus_id);
        self.0.get_mut().focus = focus_id;
    }

    pub fn set_focus_flag(&mut self, focus_id: FocusId) {
        let last = self.get_focus();

        self.remove_flag(last.to_string());
        self.insert_flag(focus_id.to_string());
    }

    pub fn insert_flag(&mut self, flag: impl Into<KeyContextFlag>) {
        self.0.get_mut().key_context.insert(flag.into());
    }

    pub fn remove_flag(&mut self, flag: impl Into<KeyContextFlag>) {
        self.0.get_mut().key_context.remove(&flag.into());
    }

    pub fn has_flag(&self, flag: &KeyContextFlag) -> bool {
        self.0.borrow().key_context.has(flag)
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
