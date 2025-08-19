use std::{
    sync::{Arc, Mutex},
    thread,
};

use egui_tiles::{Tiles, Tree};
use graliffer::{
    Frame,
    editor::Editor,
    grid::{Cell, Grid, Position},
};
use strum_macros::AsRefStr;

pub struct GralifferApp {
    layout_tree: egui_tiles::Tree<Pane>,
    tile_behavior: GralifferTilesBehavior,

    frame: Arc<Mutex<Frame>>,
    editor: Editor,

    inspect: bool,
    first_frame: bool,
}

impl GralifferApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let mut fonts = egui::FontDefinitions::default();
        egui_phosphor::add_to_fonts(&mut fonts, egui_phosphor::Variant::Regular);

        cc.egui_ctx.set_fonts(fonts);

        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.

        let mut initial_grid = Grid::new();
        initial_grid.set(
            Position::from_textual('A', 'A').unwrap(),
            Cell::new("100").unwrap(),
        );
        initial_grid.set(
            Position::from_textual('B', 'A').unwrap(),
            Cell::new("&BB").unwrap(),
        );
        initial_grid.set(
            Position::from_textual('C', 'A').unwrap(),
            Cell::new("div").unwrap(),
        );
        initial_grid.set(
            Position::from_textual('B', 'B').unwrap(),
            Cell::new("@CB").unwrap(),
        );
        initial_grid.set(
            Position::from_textual('C', 'B').unwrap(),
            Cell::new("3").unwrap(),
        );
        // initial_grid.set(Position::from_textual('D', 'A').unwrap(), Cell::new("").unwrap());
        initial_grid.set(
            Position::from_textual('E', 'A').unwrap(),
            Cell::new("20").unwrap(),
        );
        initial_grid.set(
            Position::from_textual('F', 'A').unwrap(),
            Cell::new("sub").unwrap(),
        );
        initial_grid.set(
            Position::from_textual('H', 'A').unwrap(),
            Cell::new("@AB").unwrap(),
        );
        initial_grid.set(
            Position::from_textual('I', 'A').unwrap(),
            Cell::new("set").unwrap(),
        );

        let frame = Frame {
            grid: initial_grid,
            ..Default::default()
        };

        let frame_arc = Arc::new(Mutex::new(frame));

        Self {
            tile_behavior: GralifferTilesBehavior::new(frame_arc.clone()),
            layout_tree: Self::create_layout_tree(),

            // actions_registry:
            frame: frame_arc,
            editor: Editor::new(),
            first_frame: true,
            inspect: false,
        }
    }

    pub fn load_file(&self) {
        use rfd::FileDialog;

        println!("Open File!");

        let frame_arc = self.frame.clone();

        thread::spawn(async move || {
            dbg!("in thread");
            let files = FileDialog::new()
                .add_filter("text", &["txt", "rs"])
                .add_filter("rust", &["rs", "toml"])
                .set_directory("/")
                .pick_file()
                .unwrap();

            dbg!(files);
            // let data = files.read();
            // dbg!(frame_arc.lock().unwrap());
            let mut frame = frame_arc.lock().unwrap();

            frame.act(Box::new(graliffer::grid::GridAction::Set(
                Position::from_numeric(5, 5).unwrap(),
                Cell::new_trim("OUI"),
            )));
        });
    }

    /// Create the default tile layout
    fn create_layout_tree() -> Tree<Pane> {
        let mut tiles = Tiles::default();

        let stack = tiles.insert_pane(Pane::Stack);
        let grid = tiles.insert_pane(Pane::Grid);
        let heads = tiles.insert_pane(Pane::Heads);
        let console = tiles.insert_pane(Pane::Console);
        let graphical = tiles.insert_pane(Pane::Graphical);

        let stack_head = tiles.insert_container(egui_tiles::Tabs {
            children: vec![stack, heads],
            active: Some(stack),
        });

        let horizontal = tiles.insert_container({
            let mut linear = egui_tiles::Linear {
                children: vec![grid, stack_head],
                dir: egui_tiles::LinearDir::Horizontal,
                shares: Default::default(),
            };
            linear.shares.set_share(grid, 0.8);
            linear.shares.set_share(stack_head, 0.2);

            linear
        });

        let outputs = tiles.insert_container(egui_tiles::Tabs {
            children: vec![console, graphical],
            active: Some(console),
        });

        let vertical = tiles.insert_container({
            let mut linear = egui_tiles::Linear {
                children: vec![horizontal, outputs],
                dir: egui_tiles::LinearDir::Vertical,
                shares: Default::default(),
            };
            linear.shares.set_share(horizontal, 0.8);
            linear.shares.set_share(outputs, 0.2);

            linear
        });

        Tree::new("GralifferTree", vertical, tiles)
    }
}

impl eframe::App for GralifferApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::MenuBar::new().ui(ui, |ui| {
                ui.menu_button("Graliffer", |ui| {
                    if ui.button("Open file").clicked() {
                        self.load_file();
                    }

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
                    let since_last_frame =
                        std::time::Duration::from_secs_f32(frame.info().cpu_usage.unwrap());
                    ui.label(format!("{:?}", since_last_frame));
                }

                if ui.button("Step").clicked() {
                    let mut frame_guard = self.frame.lock().unwrap();
                    let artifact = frame_guard.step();

                    self.editor.history.append(artifact);
                }

                if ui.button("Undo").clicked() {
                    self.editor.history.undo(&mut self.frame.lock().unwrap());
                }

                if ui.button("Redo").clicked() {
                    self.editor.history.redo(&mut self.frame.lock().unwrap());
                }
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            self.layout_tree.ui(&mut self.tile_behavior, ui);
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
    }
}

struct GralifferTilesBehavior {
    frame: Arc<Mutex<Frame>>,
}

impl GralifferTilesBehavior {
    fn new(frame: Arc<Mutex<Frame>>) -> Self {
        Self { frame }
    }
}

impl<'a> egui_tiles::Behavior<Pane> for GralifferTilesBehavior {
    fn tab_title_for_pane(&mut self, pane: &Pane) -> egui::WidgetText {
        pane.as_ref().into()
    }

    fn pane_ui(
        &mut self,
        ui: &mut egui::Ui,
        _tile_id: egui_tiles::TileId,
        pane: &mut Pane,
    ) -> egui_tiles::UiResponse {
        let frame = self.frame.clone();

        match pane {
            Pane::Grid => {
                Editor::grid_ui(ui, frame);
            }
            Pane::Stack => {
                Editor::stack_ui(ui, frame);
            }
            Pane::Console => {
                Editor::console_ui(ui, frame);
            }
            _ => {
                ui.label(format!("{}", pane.as_ref()));
            }
        }

        Default::default()
    }
}

#[derive(Debug, AsRefStr)]
enum Pane {
    Grid,
    Stack,
    Heads,
    Console,
    Graphical,
}
