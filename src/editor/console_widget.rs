use std::sync::{Arc, Mutex};

use egui::{Sense, Widget};

use crate::{editor::{ShortcutContext, View, ViewsIds}, Frame};

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
        if response.gained_focus() {
            ShortcutContext::store(ui.ctx(), ShortcutContext::Console);
        } else if response.lost_focus() {
            ShortcutContext::store(ui.ctx(), ShortcutContext::None);
        }

        if let Ok(_frame_guard) = self.frame.try_lock() {
            ui.label("Console! Bip boup");
        } else {
            ui.label("Could not open console :'(");
        }

        ui.response()
    }
}
