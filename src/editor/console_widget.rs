use std::sync::{Arc, Mutex};

use egui::{Sense, Widget};

use crate::{
    Frame,
    editor::{View, ViewsIds},
};

#[derive(Debug)]
pub struct ConsoleWidget {
    frame: Arc<Mutex<Frame>>,
}

impl ConsoleWidget {
    pub fn new(frame: Arc<Mutex<Frame>>) -> Self {
        Self { frame }
    }
}

impl Widget for ConsoleWidget {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let response = ui.interact(ui.available_rect_before_wrap(), ui.id(), Sense::click());
        if response.clicked() {
            response.request_focus();
        }

        ViewsIds::store(ui.ctx(), ui.id(), View::Console);
        // if response.gained_focus() {
        //     EventContext::store(ui.ctx(), EventContext::Console);
        // } else if response.lost_focus() {
        //     EventContext::store(ui.ctx(), EventContext::None);
        // }

        if let Ok(_frame_guard) = self.frame.try_lock() {
            ui.label("Console! Bip boup");
        } else {
            ui.label("Could not open console :'(");
        }

        ui.response()
    }
}
