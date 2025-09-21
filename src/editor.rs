use std::{
    collections::HashMap,
    fmt::Debug,
    hash::Hash,
    sync::{Arc, Mutex},
    thread,
};

use egui::{Context, Event, Id, Key, Modifiers, Widget};

use crate::{
    editor::{cursor::{PreferredCharPosition, PreferredGridPosition}, grid_widget::GridWidgetState}, grid::{Cell, Grid, Position}, history::History, utils::Direction, Artifact, Frame, FrameAction
};
use egui_tiles::{Tiles, Tree};
use strum_macros::AsRefStr;

mod console_widget;
mod cursor;
// mod events;
mod grid_widget;
mod history_utils;
mod stack_widget;

use console_widget::ConsoleWidget;
use cursor::Cursor;
// pub use events::{EventContext, InputEvent};
use grid_widget::GridWidget;
use history_utils::HistoryMerge;
use stack_widget::StackWidget;

pub struct Editor {
    layout_tree: egui_tiles::Tree<View>,
    tile_behavior: TilesBehavior,

    egui_ctx: Context,

    frame: Arc<Mutex<Frame>>,

    inspect: bool,
    first_frame: bool,

    // cursor: Cursor,

    history: History,
    history_merge: HistoryMerge,

    grid_widget_id: Option<egui::Id>,

    // event_registry: EventRegistry,
}

impl Editor {
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
            tile_behavior: TilesBehavior::new(frame_arc.clone()),
            layout_tree: Self::create_layout_tree(),

            egui_ctx: cc.egui_ctx.clone(),

            frame: frame_arc,

            first_frame: true,
            inspect: false,

            // cursor: Cursor::default(),
            history: History::default(),
            history_merge: HistoryMerge::default(),

            grid_widget_id: None,

            // event_registry: EventRegistry::build(),
        }
    }

    fn act(&mut self, action: EditorAction) {
        action.act(self);
    }

    // fn handle_inputs(&mut self, ctx: &egui::Context) {
    fn handle_inputs(&mut self, ui: &egui::Ui) {
        let event_filter = egui::EventFilter {
            horizontal_arrows: true,
            vertical_arrows: true,
            escape: true,
            tab: true,
        };

        ui.memory_mut(|mem| mem.set_focus_lock_filter(ui.id(), event_filter));
        let events = ui.input(|i| i.filtered_events(&event_filter));

        for event in events {
            if let Some(action) = EditorAction::from_event(&event) {
                self.act(action);
            }
        }
    }

    async fn load_file(&self) {
        println!("Loading file..");
        thread::sleep(std::time::Duration::from_secs(1));
        println!("just kidding..");

        //     use rfd::FileDialog;

        //     println!("Open File!");

        //     let frame_arc = self.frame.clone();

        //     thread::spawn(async move || {
        //         dbg!("in thread");
        //         let files = FileDialog::new()
        //             .add_filter("text", &["txt", "rs"])
        //             .add_filter("rust", &["rs", "toml"])
        //             .set_directory("/")
        //             .pick_file()
        //             .unwrap();

        //         dbg!(files);
        //         // let data = files.read();
        //         // dbg!(frame_arc.lock().unwrap());
        //         let mut frame = frame_arc.lock().unwrap();

        //         frame.act(Box::new(crate::grid::GridAction::Set(
        //             Position::from_numeric(5, 5).unwrap(),
        //             Cell::new_trim("OUI"),
        //         )));
        //     });
    }

    /// Create the default tile layout
    fn create_layout_tree() -> Tree<View> {
        let mut tiles = Tiles::default();

        let stack = tiles.insert_pane(View::Stack);
        let grid = tiles.insert_pane(View::Grid);
        let console = tiles.insert_pane(View::Console);
        let graphical = tiles.insert_pane(View::Graphical);

        let stack = tiles.insert_container(egui_tiles::Tabs {
            children: vec![stack],
            active: Some(stack),
        });

        let horizontal = tiles.insert_container({
            let mut linear = egui_tiles::Linear {
                children: vec![grid, stack],
                dir: egui_tiles::LinearDir::Horizontal,
                shares: Default::default(),
            };
            linear.shares.set_share(grid, 0.8);
            linear.shares.set_share(stack, 0.2);

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

impl Editor {
    fn grid_ui(ui: &mut egui::Ui, frame: Arc<Mutex<Frame>>) {
        GridWidget::new(frame).ui(ui);
    }

    fn console_ui(ui: &mut egui::Ui, frame: Arc<Mutex<Frame>>) {
        ConsoleWidget::new(frame).ui(ui);
    }

    fn stack_ui(ui: &mut egui::Ui, frame: Arc<Mutex<Frame>>) {
        StackWidget::new(frame).ui(ui);
    }
}

#[derive(Debug, Clone)]
pub enum EditorAction {
    Undo,
    Redo,

    CursorStep(Direction),
    CursorLeap(Direction),
    CursorMove(Position),

    GridInsertAtCursor(String),
}

impl EditorAction {
    pub fn from_event(event: &Event) -> Option<Self> {
        match event {
            Event::Key {
                key: Key::Z,
                modifiers,
                pressed: true,
                ..
            } if modifiers.command => {
                Some(Self::Undo)
            }
            Event::Key {
                key: Key::Y,
                modifiers,
                pressed: true,
                ..
            } if modifiers.command => {
                Some(Self::Redo)
            }

            Event::Key {
                key: arrow @ (
                    Key::ArrowUp
                    | Key::ArrowRight
                    | Key::ArrowDown
                    | Key::ArrowLeft
                ),
                pressed: true,
                modifiers,
                ..
            } => {
                dbg!(arrow, modifiers);

                let direction = match arrow {
                    Key::ArrowUp => Direction::Up,
                    Key::ArrowRight => Direction::Right,
                    Key::ArrowDown => Direction::Down,
                    Key::ArrowLeft => Direction::Left,
                    _ => { unreachable!(); }
                };

                if modifiers.command {
                    Some(Self::CursorLeap(direction))
                } else {
                    Some(Self::CursorStep(direction))
                }
            }

            Event::Text(string) => {
                Some(Self::GridInsertAtCursor(string.clone()))
            }

            _ => {
                None
            }
        }
    }

    pub fn act(&self, editor: &mut Editor) {
        let mut frame = editor
            .frame
            .lock()
            .expect("Should be able to get the frame");

        use EditorAction::*;
        match self {
            Redo => {
                editor.history.redo(&mut frame);
            }
            Undo => {
                editor.history.undo(&mut frame);
            }

            CursorStep(direction) => {
                let mut grid_state = GridWidgetState::get(&editor.egui_ctx, View::Grid).unwrap_or_default();
                let char_pos = match direction {
                    Direction::Down | Direction::Up => {
                        PreferredCharPosition::At(grid_state.cursor.char_position())
                    }
                    Direction::Right => { PreferredCharPosition::AtStart }
                    Direction::Left => { PreferredCharPosition::AtEnd }
                };

                grid_state.cursor.move_to(
                    PreferredGridPosition::InDirectionByOffset(
                        *direction,
                        1
                    ),
                    char_pos,
                    &frame.grid
                );

                grid_state.set(&editor.egui_ctx, View::Grid);
            }

            CursorMove(position) => {
                let mut grid_state = GridWidgetState::get(&editor.egui_ctx, View::Grid).unwrap_or_default();
                grid_state.cursor.move_to(
                    PreferredGridPosition::At(*position),
                    PreferredCharPosition::AtEnd,
                    &frame.grid
                );
                grid_state.set(&editor.egui_ctx, View::Grid);
            }

            CursorLeap(direction) => {

            }

            GridInsertAtCursor(string) => {
                let mut grid_state = GridWidgetState::get(&editor.egui_ctx, View::Grid).unwrap_or_default();

                let pos = grid_state.cursor.grid_position();

                let mut cell = frame.grid.get(pos);

                dbg!(pos, string);

                let char_inserted = cell.insert_at(string, grid_state.cursor.char_position()).unwrap_or(0);

                if char_inserted > 0 {
                    let artifact = frame.act(FrameAction::GridSet(pos, cell));
                    grid_state.cursor.move_to(cursor::PreferredGridPosition::At(pos), cursor::PreferredCharPosition::ForwardBy(char_inserted), &frame.grid);

                    if editor.history_merge.should_merge_input() {
                        dbg!("Merging !");
                        editor.history.merge_with_last(artifact);
                    } else {
                        editor.history.append(artifact);
                    }

                    editor.history_merge.update_input_timeout();
                    editor.history_merge.cancel_deletion_merge();
                }

                grid_state.set(&editor.egui_ctx, View::Grid);
            }
        }
    }
}

impl eframe::App for Editor {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::MenuBar::new().ui(ui, |ui| {
                ui.menu_button("Graliffer", |ui| {
                    if ui.button("Open file").clicked() {
                        // self.load_file();
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

                    self.history.append(artifact);
                }

                if ui.button("Undo").clicked() {
                    self.act(EditorAction::Undo);
                }

                if ui.button("Redo").clicked() {
                    self.act(EditorAction::Redo);
                }
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            self.handle_inputs(ui);

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

#[derive(Debug, Default, Clone)]
pub struct ViewsIds {
    data: HashMap<View, egui::Id>,
}

impl ViewsIds {
    const ID: &'static str = "VIEWS_IDS";

    fn store(ctx: &egui::Context, id: egui::Id, view: View) {
        ctx.data_mut(|data| {
            let context_by_id: &mut ViewsIds = data.get_persisted_mut_or_default(Id::new(Self::ID));

            context_by_id.data.insert(view, id);
        });
    }

    fn load(ctx: &egui::Context) -> Option<ViewsIds> {
        ctx.data_mut(|data| data.get_persisted(Id::new(Self::ID)))
    }

    fn get_id(&self, view: View) -> Option<egui::Id> {
        self.data.get(&view).cloned()
    }
}

#[derive(Debug, Clone, AsRefStr, Hash, PartialEq, Eq)]
enum View {
    Grid,
    Stack,
    Console,
    Graphical,
    CommandPanel,
}

struct TilesBehavior {
    frame: Arc<Mutex<Frame>>,
}

impl TilesBehavior {
    fn new(frame: Arc<Mutex<Frame>>) -> Self {
        Self { frame }
    }
}

impl egui_tiles::Behavior<View> for TilesBehavior {
    fn tab_title_for_pane(&mut self, view: &View) -> egui::WidgetText {
        view.as_ref().into()
    }

    fn pane_ui(
        &mut self,
        ui: &mut egui::Ui,
        _tile_id: egui_tiles::TileId,
        view: &mut View,
    ) -> egui_tiles::UiResponse {
        let frame = self.frame.clone();

        match view {
            View::Grid => {
                Editor::grid_ui(ui, frame);
            }
            View::Stack => {
                Editor::stack_ui(ui, frame);
            }
            View::Console => {
                Editor::console_ui(ui, frame);
            }
            _ => {
                ui.label(view.as_ref().to_string());
            }
        }

        Default::default()
    }
}
