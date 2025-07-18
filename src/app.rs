use graliffer::{
	editor::Editor, grid::{Cell, Grid, Position}, Frame, RunDescriptor
};
use egui_tiles::{TileId, Tiles, Tree};
use strum_macros::AsRefStr;


pub struct GralifferApp {
    layout_tree: egui_tiles::Tree<Pane>,
    state: GralifferState,
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

        let state = GralifferState::new();

        Self {
            layout_tree: Self::create_layout_tree(),
            state
        }
    }

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
                    if ui.button("About Graliffer").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                    if ui.button("Quit").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                    ui.separator();
                    egui::widgets::global_theme_preference_buttons(ui);
                    ui.separator();
                    ui.checkbox(&mut self.state.inspect, "Inspect");

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


                if self.state.inspect {
                    let since_last_frame = std::time::Duration::from_secs_f32(frame.info().cpu_usage.unwrap());
                    ui.label(format!("{:?}", since_last_frame));
                }

                if ui.button("Step").clicked() {
                    let artifact = self.state.frame.step();

                    self.state.editor.history.append(artifact);
                }

                if ui.button("Undo").clicked() {
                    self.state.editor.history.undo(&mut self.state.frame);
                }

                if ui.button("Redo").clicked() {
                    self.state.editor.history.redo(&mut self.state.frame);
                }
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            self.layout_tree.ui(&mut self.state, ui);
        });

        if self.state.inspect {
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

pub struct GralifferState {
    inspect: bool,
    first_frame: bool,
    frame: Frame,
    editor: Editor,
}

impl GralifferState {
    pub fn new() -> Self {
       	let mut initial_grid = Grid::new();
        initial_grid.set(Position::from_textual('A', 'A').unwrap(), Cell::new("100").unwrap());
        initial_grid.set(Position::from_textual('B', 'A').unwrap(), Cell::new("&BB").unwrap());
        initial_grid.set(Position::from_textual('C', 'A').unwrap(), Cell::new("div").unwrap());
        initial_grid.set(Position::from_textual('B', 'B').unwrap(), Cell::new("@CB").unwrap());
        initial_grid.set(Position::from_textual('C', 'B').unwrap(), Cell::new("3").unwrap());
        // initial_grid.set(Position::from_textual('D', 'A').unwrap(), Cell::new("").unwrap());
        initial_grid.set(Position::from_textual('E', 'A').unwrap(), Cell::new("20").unwrap());
        initial_grid.set(Position::from_textual('F', 'A').unwrap(), Cell::new("sub").unwrap());
        initial_grid.set(Position::from_textual('H', 'A').unwrap(), Cell::new("@AB").unwrap());
        initial_grid.set(Position::from_textual('I', 'A').unwrap(), Cell::new("set").unwrap());

        let frame = Frame::new(RunDescriptor {
            grid: initial_grid,
            ..Default::default()
        });

        let editor = Editor::default();

        Self {
            frame,
            editor,

            first_frame: true,
            inspect: false,
        }
    }
}

impl<'a> egui_tiles::Behavior<Pane> for GralifferState {


    fn tab_title_for_pane(&mut self, pane: &Pane) -> egui::WidgetText {
        pane.as_ref().into()
    }

    fn pane_ui(
        &mut self,
        ui: &mut egui::Ui,
        _tile_id: egui_tiles::TileId,
        pane: &mut Pane,
    ) -> egui_tiles::UiResponse {
        match pane {
            Pane::Grid => {
                self.editor.grid_ui(ui, &mut self.frame);
            }
            Pane::Stack => {
                self.editor.stack_ui(ui, &mut self.frame);
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
