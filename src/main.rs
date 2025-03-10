use serde::Serialize;
use eframe::egui;
use graliffer::GralifferApp;

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native("Graliffer", native_options, Box::new(|cc| Ok(Box::new(GralifferApp::new(cc)))));
}
