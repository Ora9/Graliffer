use std::{fmt::format, ops::Neg};

use anyhow::Context;
use serde_json::map::Keys;
use crate::{
	grid::{Cell, Direction, Grid, Position},
	Frame,
	RunDescriptor,
};

use eframe::glow::ZERO;
use egui::{emath::TSTransform, Label, Pos2, Rect, Scene, TextBuffer, TextWrapMode, Vec2, Widget};

/// A cursor wandering around a [`Grid`]
/// For now the cursor has only one [`Position`], but will probably have two in the future to represent a selection
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

    // pub fn move_to(&mut self, grid_position: Position, char_position: usize) {
    //     self.grid_position = grid_position;
    //     self.char_position = char_position
    // }

    /// Move the cursor one cell in the given [`Direction`],
    /// setting `self.char_position` to 0 in the process,
    /// and returning the new [`Position`] on success
    ///
    /// # Errors
    /// Returns an error if [`Head`] could not step further in that direction
    /// because it could not go outside of the [`Grid`]'s limits
    pub fn move_in_direction(&mut self, direction: Direction) -> Result<Position, anyhow::Error> {
        use Direction::*;
        self.grid_position = match direction {
            Right => self.grid_position.checked_increment_x(1),
            Down => self.grid_position.checked_increment_y(1),
            Left => self.grid_position.checked_decrement_x(1),
            Up => self.grid_position.checked_decrement_y(1),
        }.context("could not step into darkness, the position is invalid")?;

        self.char_position = 0;

        Ok(self.grid_position)
    }

    // pub fn move_char_position(&mut self, char_position: usize) {
    //     self.char_position = char_position;
    // }
}

pub struct GralifferApp {
    frame: Frame,
    cursor: Cursor,
    transform: TSTransform,
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

    	for _ in 0..20 {
    		frame.step();
    	}

    	println!("last pos: {:?}", frame.head.position.as_textual());

        Self {
            frame: frame,
            transform: TSTransform::default(),
            cursor: Cursor::new(Position::ZERO),
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

        egui::CentralPanel::default().show(ctx, |ui| {
        // });
        // egui::Window::new("graliffer ouais").show(ctx, |ui| {
            let (container_id, container_rect) = ui.allocate_space(ui.available_size());
            let response = ui.interact(container_rect, container_id, egui::Sense::click_and_drag());

            let transform =
                TSTransform::from_translation(ui.min_rect().left_top().to_vec2()) * self.transform;

            if response.clicked() || self.first_frame {
                response.request_focus();
            }

            // Handle pointer (drag, zoom ..)
            if let Some(pointer) = ui.ctx().input(|i| i.pointer.hover_pos()) {
                if container_rect.contains(pointer) {
                    let pointer_in_layer = transform.inverse() * pointer;
                    let zoom_delta = ui.ctx().input(|i| i.zoom_delta());
                    let pan_delta = ui.ctx().input(|i| i.smooth_scroll_delta * 1.5);
                    // let multi_touch_info = ui.ctx().input(|i| i.multi_touch());

                    // Zoom in on pointer:
                    self.transform = self.transform
                        * TSTransform::from_translation(pointer_in_layer.to_vec2())
                        * TSTransform::from_scaling(zoom_delta)
                        * TSTransform::from_translation(-pointer_in_layer.to_vec2());

                    // Pan:
                    self.transform = TSTransform::from_translation(pan_delta * 2.0) * self.transform;
                }
            }

            let event_filter = egui::EventFilter {
                horizontal_arrows: true,
                vertical_arrows: true,
                escape: true,
                tab: true,
                ..Default::default()
            };

            ui.memory_mut(|mem| mem.set_focus_lock_filter(container_id, event_filter));
            let events = ui.input(|i| i.filtered_events(&event_filter));

            if response.has_focus() {
                for event in &events {
                    use {egui::Event, egui::Key};
                    match event {
                        Event::Key {
                            key: key @ (
                                Key::ArrowRight
                                | Key::Tab
                                | Key::Space
                                | Key::ArrowDown
                                | Key::ArrowLeft
                                | Key::ArrowUp),
                            pressed: true,
                            ..
                        } => {

                            // If Up or Down, move grid_position
                            // If Left
                            // - and char_position == 0, move grid_position
                            // - else, decrement char_position
                            // If Right
                            // - and char_position == current_cell.len, move grid_position
                            // - else, increment char_position

                            match key {
                                Key::ArrowUp => {
                                    if let Ok(new_pos) = self.cursor.move_in_direction(Direction::Up) {
                                        let new_pos_cell_length = self.frame.grid.get(new_pos).len();
                                        self.cursor.char_position = new_pos_cell_length;
                                    }
                                },
                                Key::ArrowDown => {
                                    if let Ok(new_pos) = self.cursor.move_in_direction(Direction::Down) {
                                        let new_pos_cell_length = self.frame.grid.get(new_pos).len();
                                        self.cursor.char_position = new_pos_cell_length;
                                    }
                                }
                                Key::Tab | Key::Space => {
                                    if let Ok(new_pos) = self.cursor.move_in_direction(Direction::Right) {
                                        let new_pos_cell_length = self.frame.grid.get(new_pos).len();
                                        self.cursor.char_position = new_pos_cell_length;
                                    }
                                }
                                Key::ArrowRight => {
                                    let current_cell_len = self.frame.grid.get(self.cursor.grid_position).len();

                                    if self.cursor.char_position == current_cell_len {
                                        if let Ok(new_pos) = self.cursor.move_in_direction(Direction::Right) {
                                            let new_pos_cell_length = self.frame.grid.get(new_pos).len();
                                            self.cursor.char_position = new_pos_cell_length;
                                        }
                                    } else {
                                        self.cursor.char_position += 1;
                                    }
                                },
                                Key::ArrowLeft => {
                                    if self.cursor.char_position == 0 {
                                        if let Ok(new_pos) = self.cursor.move_in_direction(Direction::Left) {
                                            let new_pos_cell_length = self.frame.grid.get(new_pos).len();
                                            self.cursor.char_position = new_pos_cell_length;
                                        }
                                    } else {
                                        self.cursor.char_position -= 1;
                                    }
                                },
                                _ => unreachable!(),
                            }


                            // let direction = match key {
                            //     Key::ArrowRight => Direction::Right,
                            //     Key::ArrowDown => Direction::Down,
                            //     Key::ArrowLeft => Direction::Left,
                            //     Key::ArrowUp => Direction::Up,
                            // };

                            // if let Ok(new_pos) = self.cursor.move_in_direction(direction) {
                            //     let new_pos_cell_length = self.frame.grid.get(new_pos).len();
                            //     self.cursor.char_position = new_pos_cell_length;
                            // }
                        },

                        // Simple text input
                        Event::Key {
                            key: Key::Enter,
                            pressed: true,
                            ..
                        } => {

                        }

                        Event::Key {
                            key: Key::Backspace,
                            pressed: true,
                            ..
                        } => {

                            dbg!("Backspace!");

                            let cell_mut = self.frame.grid.get_mut(self.cursor.grid_position);

                            let char_pos = self.cursor.char_position;

                            if char_pos > 0 {
                                let range_start = char_pos - 1;
                                let range_end = char_pos;

                                let char_deleted = cell_mut.delete_char_range(range_start..range_end).unwrap_or(0);
                                self.cursor.char_position -= char_deleted;
                            }
                        }

                        Event::Text(text) if text != " " => {
                            dbg!(text);

                            let cell_mut = self.frame.grid.get_mut(self.cursor.grid_position);
                            let char_inserted = cell_mut.insert_at(text, self.cursor.char_position).unwrap_or(0);
                            dbg!(char_inserted);
                            self.cursor.char_position += char_inserted;
                        }
                        _ => {}
                    }
                }
            }

            ui.put(container_rect, GridWidget {
                transform,
                cursor: self.cursor,
                grid: &self.frame.grid
            });
        });
    }
}


struct GridWidget<'a> {
    cursor: Cursor,
    transform: TSTransform,
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

            let trans_rect = self.transform.inverse().mul_rect(container_rect);

            let min_x = ((trans_rect.min.x / GridWidget::CELL_FULL_SIZE).floor() as u32).clamp(PositionAxis::MIN_NUMERIC, PositionAxis::MAX_NUMERIC);
            let max_x = ((trans_rect.max.x / GridWidget::CELL_FULL_SIZE).ceil() as u32).clamp(PositionAxis::MIN_NUMERIC, PositionAxis::MAX_NUMERIC);
            let min_y = ((trans_rect.min.y / GridWidget::CELL_FULL_SIZE).floor() as u32).clamp(PositionAxis::MIN_NUMERIC, PositionAxis::MAX_NUMERIC);
            let max_y = ((trans_rect.max.y / GridWidget::CELL_FULL_SIZE).ceil() as u32).clamp(PositionAxis::MIN_NUMERIC, PositionAxis::MAX_NUMERIC);

            (min_x, max_x, min_y, max_y)
        };

        let painter = ui.painter_at(container_rect);

        for grid_pos_y in min_y..max_y {
            for grid_pos_x in min_x..max_x {

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

                let bg_color = if self.cursor.grid_position == grid_pos {
                    egui::Color32::from_gray(45)
                } else {
                    egui::Color32::from_gray(27)
                };

                let bg_corner_radius = self.transform.scaling * 3.0;

                // Draw background
                painter.rect(
                    screen_rect,
                    bg_corner_radius,
                    bg_color,
                    egui::Stroke::new(self.transform.scaling * 1.0, egui::Color32::from_gray(50)),
                    egui::StrokeKind::Inside,
                );

                painter.text(
                    screen_rect.center(),
                    egui::Align2::CENTER_CENTER,
                    cell.content(),
                    egui::FontId::monospace(self.transform.scaling * 12.0),
                    egui::Color32::WHITE
                );

                if self.cursor.grid_position == grid_pos {
                    // painter.text(
                    //     screen_rect.left_top(),
                    //     egui::Align2::LEFT_TOP,
                    //     self.cursor.char_position,
                    //     egui::FontId::monospace(self.transform.scaling * 9.0),
                    //     egui::Color32::WHITE
                    // );

                    // Blocking
                    let text_widths = ui.fonts(move |fonts| {
                        // (total, before)
                        cell.content().chars().enumerate().map(|(index, char)| {
                            let width = fonts.glyph_width(
                                &egui::FontId::monospace(self.transform.scaling * 12.0),
                                char
                            );

                            (
                                width,
                                if self.cursor.char_position > index {
                                    width
                                } else {
                                    0.0
                                }
                            )
                        }).reduce(|acc, e| (acc.0 + e.0, acc.1 + e.1))
                    });

                    let (content_total_width, content_pre_cursor_width) = text_widths.unwrap_or((0.0, 0.0));

                    let center_offset = Vec2 {
                        x: (content_total_width * 0.5).neg() + content_pre_cursor_width,
                        y: 0.0
                    };

                    let cursor_pos = screen_rect.center() + center_offset;

                    dbg!(cursor_pos);

                    painter.rect_filled(
                        Rect::from_center_size(cursor_pos, Vec2 {
                            x: self.transform.scaling * 1.0,
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
