use std::ops::Neg;

use crate::{
	grid::{Cell, Direction, Grid, GridAction, Head, Position, PositionAxis},
	Frame,
	RunDescriptor,
};

use egui::{emath::TSTransform, Pos2, Rect, Vec2, Widget};

/// A cursor wandering around a [`Grid`]
/// For now the cursor has only one [`Position`], but will probably have two in the future to represent a selection
// Work to make the char_position cursor better :
// a prefered position, to be used when moving a new grid_pos, because we want to be a certain place of the cell content
// or when clicking on a cell, we want to be at this place when
#[derive(Debug, Clone, Copy)]
struct Cursor {
    pub grid_position: Position,
    pub char_position: usize,
}

impl Cursor {
    fn new(grid_position: Position) -> Self {
        Self {
            grid_position,
            char_position: 0
        }
    }

    /// Move the cursor to new [`Position`] placing self.char_position after the last char of new cell.
    pub fn move_to(&mut self, grid_position: Position, grid: &Grid) {
        self.grid_position = grid_position;
        self.char_position = grid.get(grid_position).len();
    }

    /// Move the cursor one cell in the given [`Direction`],
    /// and move `self.char_position` after the last char of new cell.
    ///
    /// # Return value
    /// On success returns the new [`Position`], or None if the Cursors could not be moved
    /// because it could not go outside of the [`Grid`]'s limits
    pub fn move_in_direction(&mut self, direction: Direction, grid: &Grid) -> Option<Position> {
        use Direction::*;
        let new_pos_result = match direction {
            Right => self.grid_position.checked_increment_x(1),
            Down => self.grid_position.checked_increment_y(1),
            Left => self.grid_position.checked_decrement_x(1),
            Up => self.grid_position.checked_decrement_y(1),
        };

        if let Ok(new_pos) = new_pos_result {
            self.grid_position = new_pos;
            self.char_position = grid.get(new_pos).len();
            Some(new_pos)
        } else {
            None
        }
    }

    // pub fn move_char_position(&mut self, char_position: usize) {
    //     self.char_position = char_position;
    // }
}

pub struct Editor {
    cursor: Cursor,
    grid_transform: TSTransform,
}

impl Editor {
    fn show(&mut self, ui: &mut egui::Ui, frame: &mut Frame) {
        let (container_id, container_rect) = ui.allocate_space(ui.available_size());
        let response = ui.interact(container_rect, container_id, egui::Sense::click_and_drag());

        // // Autofocus on app startup
        // if self.first_frame {
        //     response.request_focus();
        //     self.first_frame = false;
        // }

        let transform =
            TSTransform::from_translation(ui.min_rect().left_top().to_vec2()) * self.grid_transform;

        // Handle pointer (drag, zoom ..)
        if let Some(pointer) = ui.ctx().input(|i| i.pointer.hover_pos()) {
            if container_rect.contains(pointer) {
                if response.clicked_by(egui::PointerButton::Primary) {
                    response.request_focus();

                    // from pointer position, figure out hovered cell rect and pos
                    // *_t for translated, as in grid render coordinates
                    let pointer_pos_t = transform.inverse().mul_pos(pointer);
                    let hovered_cell_pos_t = Pos2 {
                        x: (pointer_pos_t.x / GridWidget::CELL_FULL_SIZE).clamp(PositionAxis::MIN_NUMERIC as f32, PositionAxis::MAX_NUMERIC as f32),
                        y: (pointer_pos_t.y / GridWidget::CELL_FULL_SIZE).clamp(PositionAxis::MIN_NUMERIC as f32, PositionAxis::MAX_NUMERIC as f32),
                    };

                    // Ceil implementation says in https://doc.rust-lang.org/std/primitive.f32.html#method.ceil :
                    // « Returns the smallest integer greater than or equal to self. » wich mean that 62.0 is still 62.0 not 63.0
                    // So we truncate and add 1.0 instead
                    let hovered_cell_rect_t = Rect {
                        min: hovered_cell_pos_t.floor() * GridWidget::CELL_FULL_SIZE,
                        max: Pos2 {
                            x: (hovered_cell_pos_t.x.trunc() + 1.0) * GridWidget::CELL_FULL_SIZE,
                            y: (hovered_cell_pos_t.y.trunc() + 1.0) * GridWidget::CELL_FULL_SIZE,
                        }
                    };

                    let hovered_cell_x = hovered_cell_pos_t.x.floor() as u32;
                    let hovered_cell_y = hovered_cell_pos_t.y.floor() as u32;
                    let hovered_cell_pos = transform.mul_pos(hovered_cell_pos_t);
                    let hovered_cell_rect = transform.mul_rect(hovered_cell_rect_t);

                    if hovered_cell_rect.contains(pointer) {
                        // TODO: move the cursor to the right spot when clicking on text
                        // Should be possible if we work on Cursor with prefered position
                        self.cursor.move_to(Position::from_numeric(hovered_cell_x, hovered_cell_y).unwrap(), &frame.grid);
                    }
                }

                let pointer_in_layer = transform.inverse() * pointer;
                let zoom_delta = ui.ctx().input(|i| i.zoom_delta());
                let pan_delta = ui.ctx().input(|i| i.smooth_scroll_delta * 1.5);
                // let multi_touch_info = ui.ctx().input(|i| i.multi_touch());

                // Zoom in on pointer:
                self.grid_transform = self.grid_transform
                    * TSTransform::from_translation(pointer_in_layer.to_vec2())
                    * TSTransform::from_scaling(zoom_delta)
                    * TSTransform::from_translation(-pointer_in_layer.to_vec2());

                // Pan:
                self.grid_transform = TSTransform::from_translation(pan_delta * 2.0) * self.grid_transform;
            }
        }

        let event_filter = egui::EventFilter {
            horizontal_arrows: true,
            vertical_arrows: true,
            escape: true,
            tab: true,
            ..Default::default()
        };

        if response.has_focus() {
            ui.memory_mut(|mem| mem.set_focus_lock_filter(container_id, event_filter));
            let events = ui.input(|i| i.filtered_events(&event_filter));

            let mut focused_cell_temp = frame.grid.get(self.cursor.grid_position);
            let mut has_edited = false;

            for event in &events {
                use {egui::Event, egui::Key};
                match event {
                    Event::Key {
                        key: egui::Key::N,
                        modifiers: egui::Modifiers::SHIFT,
                        pressed: true,
                        ..
                    } => {
                        frame.step();
                    }

                    Event::Copy => {
                        let cell = frame.grid.get(self.cursor.grid_position);
                        if !cell.is_empty() {
                            ui.ctx().copy_text(cell.content());
                        }
                    }

                    Event::Cut => {
                        let cell = frame.grid.get_mut(self.cursor.grid_position);
                        if !cell.is_empty() {
                            ui.ctx().copy_text(cell.content());
                        }

                        cell.clear();
                        self.cursor.char_position = 0
                    }

                    Event::Paste(text) => {
                        let cell = frame.grid.get_mut(self.cursor.grid_position);

                        let char_inserted = cell.insert_at(text, self.cursor.char_position).unwrap_or(0);
                        self.cursor.char_position += char_inserted;
                    }

                    // TODO: better check to avoid whitespaces
                    Event::Text(text) if text != " " => {
                        dbg!(text);

                        let char_inserted = focused_cell_temp.insert_at(text, self.cursor.char_position).unwrap_or(0);
                        if char_inserted > 0 {has_edited = true};
                        self.cursor.char_position += char_inserted;
                    }

                    Event::Key {
                        key: key @ (
                            Key::ArrowRight
                            | Key::Tab
                            | Key::Space
                            | Key::Enter
                            | Key::ArrowDown
                            | Key::ArrowLeft
                            | Key::ArrowUp),
                        pressed: true,
                        ..
                    } => {
                        match key {
                            Key::ArrowUp => {
                                self.cursor.move_in_direction(Direction::Up, &frame.grid);
                            },
                            Key::ArrowDown | Key::Enter => {
                                self.cursor.move_in_direction(Direction::Down, &frame.grid);
                            }
                            Key::Tab | Key::Space => {
                                self.cursor.move_in_direction(Direction::Right, &frame.grid);
                            }
                            Key::ArrowRight => {
                                let current_cell_len = frame.grid.get(self.cursor.grid_position).len();

                                if self.cursor.char_position == current_cell_len {
                                    self.cursor.move_in_direction(Direction::Right, &frame.grid);
                                } else {
                                    self.cursor.char_position += 1;
                                }
                            },
                            Key::ArrowLeft => {
                                if self.cursor.char_position == 0 {
                                    self.cursor.move_in_direction(Direction::Left, &frame.grid);
                                } else {
                                    self.cursor.char_position -= 1;
                                }
                            },
                            _ => unreachable!(),
                        }
                    },

                    Event::Key {
                        key: Key::Backspace,
                        pressed: true,
                        modifiers,
                        ..
                    } => {
                        let cell_mut = frame.grid.get_mut(self.cursor.grid_position);

                        let char_pos = self.cursor.char_position;

                        if char_pos > 0 {
                            let range = if modifiers.ctrl {
                                // let range_start = 0;
                                // let range_end = char_pos;
                                0..char_pos
                            } else {
                                (char_pos - 1)..char_pos
                            };

                            let char_deleted = cell_mut.delete_char_range(range).unwrap_or(0);
                            self.cursor.char_position -= char_deleted;
                        } else {
                            self.cursor.move_in_direction(Direction::Left, &frame.grid);
                        }
                    }

                    Event::Key {
                        key: Key::Delete,
                        pressed: true,
                        modifiers,
                        ..
                    } => {
                        let cell_mut = frame.grid.get_mut(self.cursor.grid_position);

                        let char_pos = self.cursor.char_position;

                        if char_pos < cell_mut.len() {
                            let range = if modifiers.ctrl {
                                // let range_start = 0;
                                // let range_end = char_pos;
                                char_pos..cell_mut.len()
                            } else {
                                char_pos..(char_pos + 1)
                            };

                            let _ = cell_mut.delete_char_range(range);
                        }
                    }
                    _ => {}
                }
            }

            if has_edited {
                dbg!(frame.act(Box::new(GridAction::Set(self.cursor.grid_position, focused_cell_temp))));
            }

        }

        ui.put(container_rect, GridWidget {
            transform,
            has_focus: response.has_focus(),
            cursor: self.cursor,
            head: frame.head,
            grid: &frame.grid
        });
    }
}

pub struct GralifferApp {
    frame: Frame,
    editor: Editor,
    // cursor: Cursor,

    // grid_transform: TSTransform,

    first_frame: bool,
    inspect: bool,
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

        let mut frame = Frame::new(RunDescriptor {
        grid: initial_grid,
        ..Default::default()
        });

        // for _ in 0..20 {
        // 	frame.step();
        // }

        println!("last pos: {:?}", frame.head.position.as_textual());

        let editor = Editor {
            cursor: Cursor::new(Position::ZERO),

            grid_transform: TSTransform::default(),

        };

        Self {
            frame,
            editor,

            first_frame: true,
            inspect: false,
        }
    }
}

impl eframe::App for GralifferApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
                ui.add_space(16.0);

                egui::widgets::global_theme_preference_buttons(ui);
                ui.add_space(16.0);

                ui.checkbox(&mut self.inspect, "Inspect");

                if self.inspect {
                    let since_last_frame = std::time::Duration::from_secs_f32(frame.info().cpu_usage.unwrap());
                    dbg!(since_last_frame);
                    ui.label(format!("{:?}", since_last_frame));
                }

                if ui.button("Step").clicked() {
                    self.frame.step();
                }

                // ui.centered_and_justified(add_contents)

            });
        });

        if self.inspect {
            egui::Window::new("insection ouais").show(ctx, |ui| {
                ctx.inspection_ui(ui);
            });
            egui::Window::new("memory ouais").show(ctx, |ui| {
                ctx.memory_ui(ui);
            });
        }

        // egui::SidePanel::left("inspectors").show(ctx, |ui| {
        //     ui.separator();
        // });
        // });

        // egui::CentralPanel::default().show(ctx, |ui| {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.editor.show(ui, &mut self.frame);
        });
    }
}


struct GridWidget<'a> {
    cursor: Cursor,
    has_focus: bool,
    transform: TSTransform,
    head: Head,
    grid: &'a Grid,
}

impl<'a> GridWidget<'a> {
    const CELL_SIZE: f32 = 50.0;
    const CELL_PADDING: f32 = 1.5;
    const CELL_FULL_SIZE: f32 = Self::CELL_SIZE + Self::CELL_PADDING;
}

impl<'a> Widget for GridWidget<'a> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let container_id = ui.id();
        let container_rect = ui.max_rect();

        let response = ui.response();

        let (min_x, max_x, min_y, max_y) = {
            use crate::grid::PositionAxis;

            let rect_t = self.transform.inverse().mul_rect(container_rect);

            let min_x = PositionAxis::clamp_numeric((rect_t.min.x / GridWidget::CELL_FULL_SIZE).floor() as u32);
            let max_x = PositionAxis::clamp_numeric((rect_t.max.x / GridWidget::CELL_FULL_SIZE).ceil() as u32);
            let min_y = PositionAxis::clamp_numeric((rect_t.min.y / GridWidget::CELL_FULL_SIZE).floor() as u32);
            let max_y = PositionAxis::clamp_numeric((rect_t.max.y / GridWidget::CELL_FULL_SIZE).ceil() as u32);

            (min_x, max_x, min_y, max_y)
        };

        let painter = ui.painter_at(container_rect);

        for grid_pos_y in min_y..=max_y {
            for grid_pos_x in min_x..=max_x {

                let screen_pos = Pos2 {
                    x: GridWidget::CELL_FULL_SIZE * (grid_pos_x as f32),
                    y: GridWidget::CELL_FULL_SIZE * (grid_pos_y as f32),
                };

                let screen_rect = self.transform.mul_rect(Rect {
                    min: screen_pos + Vec2::splat(GridWidget::CELL_PADDING),
                    max: screen_pos + Vec2::splat(GridWidget::CELL_SIZE),
                });

                let grid_pos = Position::from_numeric(grid_pos_x, grid_pos_y).unwrap();

                let cell = self.grid.get(grid_pos);



                let bg_color = /*if self.has_focus && self.cursor.grid_position == grid_pos {
                    egui::Color32::from_gray(45)
                } else */ if self.head.position == grid_pos {
                    egui::Color32::from_hex("#445E93").unwrap()
                } else {
                    egui::Color32::from_gray(27)
                };

                let (stroke, stroke_kind) = if self.cursor.grid_position == grid_pos {
                    (
                        egui::Stroke::new(self.transform.scaling * 2.0, egui::Color32::from_gray(45)),
                        egui::StrokeKind::Outside,
                    )
                } else {
                    (
                        egui::Stroke::new(self.transform.scaling * 1.0, egui::Color32::from_gray(45)),
                        egui::StrokeKind::Inside,
                    )
                };

                let bg_corner_radius = self.transform.scaling * 3.0;

                // Draw background
                painter.rect(
                    screen_rect,
                    bg_corner_radius,
                    bg_color,
                    stroke,
                    stroke_kind,
                );

                painter.text(
                    screen_rect.center(),
                    egui::Align2::CENTER_CENTER,
                    cell.content(),
                    egui::FontId::monospace(self.transform.scaling * 12.0),
                    egui::Color32::WHITE
                );

                if self.cursor.grid_position == grid_pos && self.has_focus {
                    // painter.text(
                    //     screen_rect.left_top(),
                    //     egui::Align2::LEFT_TOP,
                    //     self.cursor.char_position,
                    //     egui::FontId::monospace(self.transform.scaling * 9.0),
                    //     egui::Color32::WHITE
                    // );

                    // Blocking
                    // (total, before_cursor)
                    let (content_total_width, content_pre_cursor_width) = ui.fonts(move |fonts| {
                        cell.content().chars().enumerate().map(|(index, char)| {
                            let width = fonts.glyph_width(
                                &egui::FontId::monospace(self.transform.scaling * 12.0),
                                char
                            );

                            if self.cursor.char_position > index {
                                (width, width)
                            } else {
                                (width, 0.0)
                            }
                        }).reduce(|acc, e| (acc.0 + e.0, acc.1 + e.1))
                    }).unwrap_or((0.0, 0.0));

                    let center_offset = Vec2 {
                        x: (content_total_width * 0.5).neg() + content_pre_cursor_width,
                        y: 0.0
                    };

                    painter.rect_filled(
                        Rect::from_center_size(screen_rect.center() + center_offset, Vec2 {
                            x: self.transform.scaling * 0.8,
                            y: self.transform.scaling * 13.0
                        }),
                        2.0,
                        egui::Color32::WHITE,
                    );
                }
            }
        }
        response
    }
}
