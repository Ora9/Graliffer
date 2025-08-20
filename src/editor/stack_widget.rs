use std::sync::{Arc, Mutex};

use egui::{FontFamily, RichText, Widget};

use crate::Frame;

pub struct StackWidget {
    frame: Arc<Mutex<Frame>>,
}

impl StackWidget {
    pub fn new(frame: Arc<Mutex<Frame>>) -> Self {
        Self { frame }
    }
}

impl Widget for StackWidget {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        if let Ok(frame) = self.frame.try_lock() {
            egui::ScrollArea::vertical()
                .stick_to_bottom(true)
                .show(ui, |ui| {
                    egui::Frame::new()
                        .inner_margin(egui::Vec2 { x: 20.0, y: 10.0 })
                        .show(ui, |ui| {
                            egui::Grid::new("stack_ui")
                                .striped(false)
                                .spacing((5.0, 0.0))
                                .num_columns(2)
                                .show(ui, |ui| {
                                    for (i, operand) in frame.stack.iter().enumerate() {
                                        ui.label(
                                            RichText::new(format!("{i}: "))
                                                .size(15.0)
                                                .family(FontFamily::Monospace),
                                        );

                                        ui.label(
                                            RichText::new(operand.as_cell().content().to_string())
                                            .size(15.0)
                                            .family(FontFamily::Monospace),
                                        );
                                        ui.end_row();
                                    }
                                });
                        });
                });
        } else {
            ui.label("Could not show the stack");
        }

        ui.response()
    }
}
