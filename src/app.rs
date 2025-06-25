use crate::{
	artifact::History, editor::Editor, grid::{Cell, Grid, Position}, Frame, RunDescriptor
};

pub struct GralifferApp {
    frame: Frame,
    editor: Editor,
    first_frame: bool,
    inspect: bool,

    history: History,
}

impl GralifferApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.

       	let mut initial_grid = Grid::new();
        initial_grid.set(Position::from_textual('A', 'A').unwrap(), Cell::new("100").unwrap());
        initial_grid.set(Position::from_textual('B', 'A').unwrap(), Cell::new("&BB").unwrap());
        initial_grid.set(Position::from_textual('C', 'A').unwrap(), Cell::new("div").unwrap());
        initial_grid.set(Position::from_textual('B', 'B').unwrap(), Cell::new("@CB").unwrap());
        initial_grid.set(Position::from_textual('C', 'B').unwrap(), Cell::new("3").unwrap());
        // initial_grid.set(Position::from_textual('D', 'A').unwrap(), Cell::new("").unwrap());
        initial_grid.set(Position::from_textual('E', 'A').unwrap(), Cell::new("20").unwrap());
        initial_grid.set(Position::from_textual('F', 'A').unwrap(), Cell::new("sub").unwrap());
        initial_grid.set(Position::from_textual('H', 'A').unwrap(), Cell::new("@AA").unwrap());
        initial_grid.set(Position::from_textual('I', 'A').unwrap(), Cell::new("jmp").unwrap());

        let frame = Frame::new(RunDescriptor {
            grid: initial_grid,
            ..Default::default()
        });

        // for _ in 0..20 {
        // 	frame.step();
        // }

        println!("last pos: {:?}", frame.head.position.as_textual());

        Self {
            frame,
            editor: Editor::default(),

            first_frame: true,
            inspect: false,

            history: History::new(),
        }
    }
}

impl eframe::App for GralifferApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("Graliffer", |ui| {
                    if ui.button("About Graliffer").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                    if ui.button("Quit").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                    ui.separator();
                    egui::widgets::global_theme_preference_buttons(ui);
                    ui.separator();
                    ui.checkbox(&mut self.inspect, "Inspect");

                });
                ui.menu_button("File", |ui| {
                    if ui.button("Open file").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                    if ui.button("Open example").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
                ui.menu_button("Tools", |ui| {
                    if ui.button("Ouais").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
                // ui.add_space(16.0);

                ui.add_space(16.0);


                if self.inspect {
                    let since_last_frame = std::time::Duration::from_secs_f32(frame.info().cpu_usage.unwrap());
                    ui.label(format!("{:?}", since_last_frame));
                }

                if ui.button("Step").clicked() {
                    let artifact = self.frame.step();

                    self.history.append(artifact);
                }

                if ui.button("Undo").clicked() {
                    let artifact = self.history.undo(&mut self.frame);
                }

                if ui.button("Redo").clicked() {
                    let artifact = self.history.redo(&mut self.frame);
                }



                // ui.centered_and_justified(add_contents)

            });
        });

        if self.inspect {
            egui::Window::new("insection ouais").show(ctx, |ui| {
                ctx.inspection_ui(ui);
            });
            egui::Window::new("settings ouais").show(ctx, |ui| {
                ctx.settings_ui(ui);
            });
            egui::Window::new("memory ouais").show(ctx, |ui| {
                ctx.memory_ui(ui);
            });
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            // // Autofocus on app startup
            // if self.first_frame {
            //     ui.response().request_focus();
            //     self.first_frame = false;
            // }

            // ctx.input(|input_state| {
            //     dbg!(&input_state);
            // });

            // let artifact = self.editor.show(ui, &mut self.frame);

            self.editor.show(ui, &mut self.frame);

            // self.history.append(artifact);
        });

   }
}
