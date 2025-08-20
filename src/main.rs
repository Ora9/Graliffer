use graliffer::GralifferEditor;

fn main() {
    let native_options = eframe::NativeOptions::default();
    let _ = eframe::run_native(
        "Graliffer",
        native_options,
        Box::new(|cc| Ok(Box::new(GralifferEditor::new(cc)))),
    );
}
