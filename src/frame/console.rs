use std::fmt::Debug;

#[derive(Default, Debug)]
pub struct Console {
    pub buffer: String,
}

impl Console {
    pub const MAX_BUFFER_LENGTH: u32 = 1000;

    pub fn new() -> Self {
        Self {
            buffer: String::new(),
        }
    }

    pub fn print(&mut self, string: &str) {
        self.buffer.push_str(string);
        dbg!(string);
    }
}
