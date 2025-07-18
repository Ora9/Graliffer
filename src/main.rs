mod app;
use app::{
    GralifferApp,
};

fn main() {
    let native_options = eframe::NativeOptions::default();
    let _ = eframe::run_native("Graliffer", native_options, Box::new(|cc| Ok(Box::new(GralifferApp::new(cc)))));
}
