use std::{
    sync::{Arc, Mutex},
    thread,
};

use egui::Widget;

use egui_tiles::{Tiles, Tree};
use crate::{
    action::History, grid::{Cell, Grid, Position}, Frame
};
use strum_macros::AsRefStr;

mod cursor;
use cursor::Cursor;

mod history_merge;
use history_merge::HistoryMerge;

mod grid_widget;
use grid_widget::GridWidget;

mod console_widget;
use console_widget::ConsoleWidget;

mod stack_widget;
use stack_widget::StackWidget;

pub struct Editor {
    layout_tree: egui_tiles::Tree<Pane>,
    tile_behavior: TilesBehavior,

    frame: Arc<Mutex<Frame>>,

    inspect: bool,
    first_frame: bool,

    cursor: Cursor,
    history: History,
    history_merge: HistoryMerge,

    // A timeout for the next acceptable text input that would be
    // merged in undo history. This is used to merge close
    // text input (timewise), and make undo/redo a bit less granular
    // `None` or any already passed timestamp would mean to create a new
    // history entry
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

            frame: frame_arc,

            first_frame: true,
            inspect: false,

            cursor: Cursor::default(),
            history: History::default(),
            history_merge: HistoryMerge::default(),
        }
    }

    fn grid_ui(ui: &mut egui::Ui, frame: Arc<Mutex<Frame>>) {
        GridWidget::new(frame).ui(ui);
    }

    fn console_ui(ui: &mut egui::Ui, frame: Arc<Mutex<Frame>>) {
        ConsoleWidget::new(frame).ui(ui);
    }

    fn stack_ui(ui: &mut egui::Ui, frame: Arc<Mutex<Frame>>) {
        StackWidget::new(frame).ui(ui);
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

    // fn handle_inputs(&mut self, ui: &mut egui::Ui, frame: &mut Frame) -> egui::Response {
    //     let container_rect = ui.max_rect();
    //     let container_id = ui.id();
    //     let response = ui.interact(container_rect, container_id, egui::Sense::click_and_drag());

    //     if let Some(pointer_pos) = ui.ctx().input(|i| i.pointer.hover_pos()) {
    //         if container_rect.contains(pointer_pos) {
    //             if response.clicked_by(egui::PointerButton::Primary) {
    //                 response.request_focus();

    //                 // from pointer position, figure out hovered cell rect and pos
    //                 // *_t for translated, as in grid render coordinates
    //                 let pointer_pos_t = self.screen_transform.inverse().mul_pos(pointer_pos);
    //                 let hovered_cell_pos_t = Pos2 {
    //                     x: (pointer_pos_t.x / GridWidget::CELL_FULL_SIZE).clamp(PositionAxis::MIN_NUMERIC as f32, PositionAxis::MAX_NUMERIC as f32),
    //                     y: (pointer_pos_t.y / GridWidget::CELL_FULL_SIZE).clamp(PositionAxis::MIN_NUMERIC as f32, PositionAxis::MAX_NUMERIC as f32),
    //                 };

    //                 // Ceil implementation says in https://doc.rust-lang.org/std/primitive.f32.html#method.ceil :
    //                 // « Returns the smallest integer greater than or equal to self. » wich mean that 62.0 is still 62.0 not 63.0
    //                 // So we truncate and add 1.0 instead
    //                 let hovered_cell_rect_t = Rect {
    //                     min: hovered_cell_pos_t.floor() * GridWidget::CELL_FULL_SIZE,
    //                     max: Pos2 {
    //                         x: (hovered_cell_pos_t.x.trunc() + 1.0) * GridWidget::CELL_FULL_SIZE,
    //                         y: (hovered_cell_pos_t.y.trunc() + 1.0) * GridWidget::CELL_FULL_SIZE,
    //                     }
    //                 };

    //                 let hovered_cell_x = hovered_cell_pos_t.x.floor() as u32;
    //                 let hovered_cell_y = hovered_cell_pos_t.y.floor() as u32;
    //                 // let hovered_cell_pos = self.screen_transform.mul_pos(hovered_cell_pos_t);
    //                 let hovered_cell_rect = self.screen_transform.mul_rect(hovered_cell_rect_t);

    //                 if hovered_cell_rect.contains(pointer_pos) {
    //                     // TODO: move the cursor to the right spot when clicking on text
    //                     // Should be possible if we work on Cursor with prefered position
    //                     if let Ok(grid_pos) = Position::from_numeric(hovered_cell_x, hovered_cell_y) {
    //                         self.cursor.move_to(cursor::PreferredGridPosition::At(grid_pos), cursor::PreferredCharPosition::AtEnd, frame);
    //                     }
    //                 }
    //             }

    //             let pointer_in_layer = self.screen_transform.inverse() * pointer_pos;
    //             let zoom_delta = ui.ctx().input(|i| i.zoom_delta());
    //             let pan_delta = ui.ctx().input(|i| i.smooth_scroll_delta * 1.5);
    //             // let multi_touch_info = ui.ctx().input(|i| i.multi_touch());

    //             // Zoom in on pointer:
    //             self.grid_transform = self.grid_transform
    //                 * TSTransform::from_translation(pointer_in_layer.to_vec2())
    //                 * TSTransform::from_scaling(zoom_delta)
    //                 * TSTransform::from_translation(-pointer_in_layer.to_vec2());

    //             // Pan:
    //             self.grid_transform = TSTransform::from_translation(pan_delta * 2.0) * self.grid_transform;
    //         }
    //     }

    //     let event_filter = egui::EventFilter {
    //         horizontal_arrows: true,
    //         vertical_arrows: true,
    //         escape: true,
    //         tab: true,
    //     };

    //     if response.has_focus() {
    //         ui.memory_mut(|mem| mem.set_focus_lock_filter(container_id, event_filter));
    //         let events = ui.input(|i| i.filtered_events(&event_filter));

    //         for event in &events {
    //             use {egui::Event, egui::Key};

    //             match event {
    //                 // Text input
    //                 // TODO: better check to avoid whitespaces
    //                 Event::Text(text) if text != " " => {
    //                     let pos = self.cursor.grid_position();
    //                     let mut cell = frame.grid.get(pos);

    //                     let char_inserted = cell.insert_at(text, self.cursor.char_position()).unwrap_or(0);

    //                     if char_inserted > 0 {
    //                         let artifact = frame.act(Box::new(GridAction::Set(pos, cell)));
    //                         self.cursor.move_to(cursor::PreferredGridPosition::At(pos), cursor::PreferredCharPosition::ForwardBy(char_inserted), frame);

    //                         if self.history_merge.should_merge_input() {
    //                             dbg!("Merging !");
    //                             self.history.merge_with_last(artifact);
    //                         } else {
    //                             self.history.append(artifact);
    //                         }

    //                         self.history_merge.update_input_timeout();
    //                         self.history_merge.cancel_deletion_merge();
    //                     }
    //                 }

    //                 // Open file
    //                 Event::Key {
    //                     key: Key::O,
    //                     pressed: true,
    //                     modifiers,
    //                     ..
    //                 } if modifiers.ctrl => {
    //                 }

    //                 // Clipboard
    //                 Event::Copy => {
    //                     let cell = frame.grid.get(self.cursor.grid_position());
    //                     if !cell.is_empty() {
    //                         ui.ctx().copy_text(cell.content());
    //                     }
    //                 }

    //                 Event::Cut => {
    //                     let pos = self.cursor.grid_position();
    //                     let mut cell = frame.grid.get(pos);

    //                     if !cell.is_empty() {
    //                         ui.ctx().copy_text(cell.content());

    //                         cell.clear();

    //                         let artifact = frame.act(Box::new(GridAction::Set(pos, cell)));
    //                         self.cursor.move_to(cursor::PreferredGridPosition::At(pos), cursor::PreferredCharPosition::AtEnd, frame);
    //                         self.history_merge.cancel_all_merge();

    //                         self.history.append(artifact);
    //                     }
    //                 }

    //                 Event::Paste(text) => {
    //                     let pos = self.cursor.grid_position();
    //                     let mut cell = frame.grid.get(pos);

    //                     let char_inserted = cell.insert_at(text, self.cursor.char_position()).unwrap_or(0);

    //                     if char_inserted > 0 {
    //                         let artifact = frame.act(Box::new(GridAction::Set(pos, cell)));
    //                         // artifact.push(frame.act(Box::new(CursorAction::CharMoveTo(cursor::PreferredCharPosition::ForwardBy(char_inserted)))));
    //                         self.cursor.move_to(cursor::PreferredGridPosition::At(pos), cursor::PreferredCharPosition::ForwardBy(char_inserted), frame);
    //                         self.history_merge.cancel_all_merge();

    //                         self.history.append(artifact);
    //                     }
    //                 }

    //                 // Undo redo
    //                 Event::Key {
    //                     key: Key::Z,
    //                     pressed: true,
    //                     modifiers,
    //                     ..
    //                 } if modifiers.ctrl => {
    //                     self.history.undo(frame);
    //                     self.history_merge.cancel_all_merge();
    //                 }

    //                 Event::Key {
    //                     key: Key::U,
    //                     pressed: true,
    //                     modifiers,
    //                     ..
    //                 } if modifiers.ctrl => {
    //                     let file_out = serde_json::to_string_pretty(&frame).unwrap();

    //                     let file_in = serde_json::from_str::<Frame>(&file_out);
    //                     dbg!(file_in);
    //                 }

    //                 Event::Key {
    //                     key: Key::Y,
    //                     pressed: true,
    //                     modifiers,
    //                     ..
    //                 } if modifiers.ctrl => {
    //                     self.history.redo(frame);
    //                     self.history_merge.cancel_all_merge();
    //                 }

    //                 // Cursor movements
    //                 Event::Key {
    //                     key: key @ (
    //                         Key::ArrowRight
    //                         | Key::Tab
    //                         | Key::Space
    //                         | Key::Enter
    //                         | Key::ArrowDown
    //                         | Key::ArrowLeft
    //                         | Key::ArrowUp),
    //                     pressed: true,
    //                     modifiers,
    //                     ..
    //                 } => {

    //                     self.history_merge.cancel_all_merge();

    //                     match key {
    //                         Key::ArrowUp => {
    //                             if modifiers.ctrl {

    //                             } else {

    //                             }

    //                             self.cursor.move_to(
    //                                 PreferredGridPosition::InDirectionByOffset(Direction::Up, 1),
    //                                 PreferredCharPosition::AtEnd,
    //                                 frame
    //                             );
    //                         },
    //                         Key::ArrowDown | Key::Enter => {
    //                             self.cursor.move_to(
    //                                 PreferredGridPosition::InDirectionByOffset(Direction::Down, 1),
    //                                 PreferredCharPosition::AtEnd,
    //                                 frame
    //                             );
    //                         }
    //                         Key::Tab | Key::Space => {
    //                             self.cursor.move_to(
    //                                 PreferredGridPosition::InDirectionByOffset(Direction::Right, 1),
    //                                 PreferredCharPosition::AtEnd,
    //                                 frame
    //                             );
    //                         }
    //                         Key::ArrowRight => {
    //                             let current_cell_len = frame.grid.get(self.cursor.grid_position()).len();
    //                             // if we are already at the end of this cell
    //                             if self.cursor.char_position() == current_cell_len {
    //                                 if modifiers.ctrl {
    //                                     self.cursor.move_to(
    //                                         PreferredGridPosition::InDirectionUntilNonEmpty(Direction::Right),
    //                                         PreferredCharPosition::AtStart,
    //                                         frame
    //                                     );
    //                                 } else {
    //                                     self.cursor.move_to(
    //                                         PreferredGridPosition::InDirectionByOffset(Direction::Right, 1),
    //                                         PreferredCharPosition::AtStart,
    //                                         frame
    //                                     );
    //                                 }
    //                             } else {
    //                                 if modifiers.ctrl {
    //                                     self.cursor.char_move_to(
    //                                         PreferredCharPosition::AtEnd,
    //                                         frame
    //                                     );
    //                                 } else {
    //                                     self.cursor.char_move_to(
    //                                         PreferredCharPosition::ForwardBy(1),
    //                                         frame
    //                                     );
    //                                 }
    //                             };
    //                         },
    //                         Key::ArrowLeft => {
    //                             if self.cursor.char_position() == 0 {
    //                                 if modifiers.ctrl {
    //                                     self.cursor.move_to(
    //                                         PreferredGridPosition::InDirectionUntilNonEmpty(Direction::Left),
    //                                         PreferredCharPosition::AtStart,
    //                                         frame
    //                                     );
    //                                 } else {
    //                                     self.cursor.move_to(
    //                                         PreferredGridPosition::InDirectionByOffset(Direction::Left, 1),
    //                                         PreferredCharPosition::AtEnd,
    //                                         frame
    //                                     );
    //                                 }
    //                             } else {
    //                                 if modifiers.ctrl {
    //                                     self.cursor.char_move_to(
    //                                         PreferredCharPosition::AtStart,
    //                                         frame
    //                                     );
    //                                 } else {
    //                                     self.cursor.char_move_to(
    //                                         PreferredCharPosition::BackwardBy(1),
    //                                         frame
    //                                     );
    //                                 }
    //                             }
    //                         },
    //                         _ => unreachable!(),
    //                     }
    //                 },

    //                 Event::Key {
    //                     key: Key::Backspace,
    //                     pressed: true,
    //                     modifiers,
    //                     ..
    //                 } => {
    //                     let grid_pos = self.cursor.grid_position();
    //                     let char_pos = self.cursor.char_position();

    //                     if char_pos > 0 {
    //                         let mut cell = frame.grid.get(grid_pos);

    //                         let char_range = if modifiers.ctrl {
    //                             0..char_pos
    //                         } else {
    //                             (char_pos - 1)..char_pos
    //                         };

    //                         let chars_deleted = cell.delete_char_range(char_range).unwrap_or(0);

    //                         let artifact = frame.act(Box::new(GridAction::Set(grid_pos, cell)));

    //                         self.cursor.char_move_to(
    //                             PreferredCharPosition::BackwardBy(chars_deleted),
    //                             frame
    //                         );

    //                         if self.history_merge.should_merge_deletion() {
    //                             self.history.merge_with_last(artifact);
    //                         } else {
    //                             self.history.append(artifact);
    //                         }
    //                         self.history_merge.update_deletion_timeout();
    //                         self.history_merge.cancel_input_merge();
    //                     } else {
    //                         self.cursor.move_to(
    //                             PreferredGridPosition::InDirectionByOffset(Direction::Left, 1),
    //                             PreferredCharPosition::AtEnd,
    //                             frame
    //                         );
    //                         self.history_merge.cancel_all_merge();
    //                     }
    //                 }

    //                 Event::Key {
    //                     key: Key::Delete,
    //                     pressed: true,
    //                     modifiers,
    //                     ..
    //                 } => {
    //                     let grid_pos = self.cursor.grid_position();
    //                     let char_pos = self.cursor.char_position();
    //                     let mut cell = frame.grid.get(grid_pos);

    //                     if char_pos < cell.len() {
    //                         let range = if modifiers.ctrl {
    //                             char_pos..cell.len()
    //                         } else {
    //                             char_pos..(char_pos + 1)
    //                         };

    //                         let _ = cell.delete_char_range(range);
    //                         let artifact = frame.act(Box::new(GridAction::Set(grid_pos, cell)));
    //                         self.history_merge.cancel_all_merge();
    //                         self.history.append(artifact);
    //                     }
    //                 }
    //                 _ => {}
    //             };
    //         }
    //     }

    //     // if should_merge_artifact {
    //     //     self.history.merge_with_last(artifact);
    //     // } else {
    //     // }

    //     response
    // }
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
                    self.history.undo(&mut self.frame.lock().unwrap());
                }

                if ui.button("Redo").clicked() {
                    self.history.redo(&mut self.frame.lock().unwrap());
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

struct TilesBehavior {
    frame: Arc<Mutex<Frame>>,
}

impl TilesBehavior {
    fn new(frame: Arc<Mutex<Frame>>) -> Self {
        Self { frame }
    }
}

impl egui_tiles::Behavior<Pane> for TilesBehavior {
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
                ui.label(pane.as_ref().to_string());
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
