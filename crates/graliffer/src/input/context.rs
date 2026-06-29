// pub struct KeyContext(Vec<KeyContextEntry>);

// pub struct KeyContextEntry {
//     key: String,
//     value: Option<String>,
// }

// impl KeyContextEntry {
//     pub fn new_key(key: String) -> Self {
//         Self { key, value: None }
//     }

//     pub fn new_key_value(key: String, value: String) -> Self {
//         Self {
//             key,
//             value: Some(value),
//         }
//     }
// }

// // "Grid && mode == insert"
// // "ProjectPanel && mode == "
// pub enum ContextPredicate {
//     Identifier(String),
//     // Equal(String, String),
//     // NotEqual(String, String),

//     // Not(Box<ContextPredicate>),
//     // And(Box<ContextPredicate>, Box<ContextPredicate>),
//     // Or(Box<ContextPredicate>, Box<ContextPredicate>),
// }

// pub struct ContextTree {}

use crate::{app::FocusId, input::InputMode};

#[derive(Debug, Clone, Copy)]
pub struct KeyContext {
    pub focus: FocusId,
    pub input_mode: InputMode,
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct KeyContextPredicate {
    pub focus: Option<FocusId>,
    pub input_mode: Option<InputMode>,
}

impl KeyContextPredicate {
    pub fn matches(&self, key_context: KeyContext) -> bool {
        if let Some(focus) = self.focus
            && focus != key_context.focus
        {
            false
        } else if let Some(input_mode) = self.input_mode
            && input_mode != key_context.input_mode
        {
            false
        } else {
            true
        }
    }
}
