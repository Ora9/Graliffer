use std::sync::{Arc, Mutex};

use egui::Widget;

use crate::Frame;

#[derive(Debug)]
pub struct ConsoleWidget {
    frame: Arc<Mutex<Frame>>,
}

impl ConsoleWidget {
    pub fn new(frame: Arc<Mutex<Frame>>) -> Self {
        Self {
            frame
        }
    }
}

impl Widget for ConsoleWidget {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {

        if let Ok(_frame_guard) = self.frame.try_lock() {
            ui.label("Console! Bip boup");
        } else {
            ui.label("Could not open console :'(");
        }

        ui.response()
    }
}
