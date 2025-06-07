use std::ops::Neg;

use crate::{
    artifact::{Action, Artifact, History}, editor::cursor::{PreferredCharPosition, PreferredGridPosition}, grid::{Cell, Grid, GridAction, Head, Position, PositionAxis}, utils::Direction, Frame, RunDescriptor
};

use egui::{emath::TSTransform, KeyboardShortcut, Pos2, Rect, Vec2, Widget};

mod cursor;
pub use cursor::Cursor;

#[derive(Debug, Default)]
pub struct Editor {
    cursor: Cursor,
    // grid transform relative to the egui grid's window
    grid_transform: TSTransform,
    // grid transform relative to the whole egui viewport
    screen_transform: TSTransform,

    last_text_artifact_merge: Option<std::time::Instant>,
}

impl Editor {
    pub fn new() -> Self {
        Self::default()
    }

    fn keyboard_shortcuts(self) -> Vec<(KeyboardShortcut, Box<dyn Action>)> {
        vec![
            (KeyboardShortcut::new(egui::Modifiers::CTRL, egui::Key::N), Box::new(GridAction::Set(Position::from_numeric(0, 0).unwrap(), Cell::new("oui").unwrap()))),
            (KeyboardShortcut::new(egui::Modifiers::CTRL, egui::Key::B), Box::new(GridAction::Set(Position::from_numeric(0, 0).unwrap(), Cell::new("mbé").unwrap())))
        ]
    }

    fn handle_inputs(&mut self, ui: &egui::Ui, frame: &Frame) -> Artifact {
        self.screen_transform =
            TSTransform::from_translation(ui.min_rect().left_top().to_vec2()) * self.grid_transform;

        let container_rect = ui.max_rect();
        let response = ui.response();

        let mut artifact = Artifact::EMPTY;

        // Handle pointer (drag, zoom ..)
        if let Some(pointer) = ui.ctx().input(|i| i.pointer.hover_pos()) {
            if container_rect.contains(pointer) {
                if response.clicked_by(egui::PointerButton::Primary) {
                    response.request_focus();

                    // from pointer position, figure out hovered cell rect and pos
                    // *_t for translated, as in grid render coordinates
                    let pointer_pos_t = self.screen_transform.inverse().mul_pos(pointer);
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
                    // let hovered_cell_pos = self.screen_transform.mul_pos(hovered_cell_pos_t);
                    let hovered_cell_rect = self.screen_transform.mul_rect(hovered_cell_rect_t);


                    if hovered_cell_rect.contains(pointer) {
                        // TODO: move the cursor to the right spot when clicking on text
                        // Should be possible if we work on Cursor with prefered position
                        if let Ok(grid_pos) = Position::from_numeric(hovered_cell_x, hovered_cell_y) {
                            self.cursor.move_to(grid_pos, cursor::PreferredCharPosition::AtEnd, frame);
                        }
                    }
                }

                let pointer_in_layer = self.screen_transform.inverse() * pointer;
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

        artifact
    }

    pub fn show(&mut self, ui: &mut egui::Ui, frame: &mut Frame) -> Artifact {
        let (container_id, container_rect) = ui.allocate_space(ui.available_size());
        let response = ui.interact(container_rect, container_id, egui::Sense::click_and_drag());

        let event_filter = egui::EventFilter {
            horizontal_arrows: true,
            vertical_arrows: true,
            escape: true,
            tab: true,
            ..Default::default()
        };

        let should_merge = false;
        let mut artifact = Artifact::EMPTY;

        self.handle_inputs(ui, frame);

        if response.has_focus() {
            ui.memory_mut(|mem| mem.set_focus_lock_filter(container_id, event_filter));
            let events = ui.input(|i| i.filtered_events(&event_filter));

            for event in &events {
                use {egui::Event, egui::Key};

                dbg!(event);

                artifact.push(match event {
                    Event::Copy => {
                        let cell = frame.grid.get(self.cursor.grid_position());
                        if !cell.is_empty() {
                            ui.ctx().copy_text(cell.content());
                        }

                        Artifact::EMPTY
                    }

                    Event::Cut => {
                        let pos = self.cursor.grid_position();
                        let mut cell = frame.grid.get(pos);

                        if !cell.is_empty() {
                            ui.ctx().copy_text(cell.content());

                            cell.clear();

                            let artifact = frame.act(Box::new(GridAction::Set(pos, cell)));
                            self.cursor.move_to(pos, cursor::PreferredCharPosition::AtEnd, &frame);

                            artifact
                        } else {
                            Artifact::EMPTY
                        }
                    }

                    Event::Paste(text) => {
                        let pos = self.cursor.grid_position();
                        let mut cell = frame.grid.get(pos);

                        let char_inserted = cell.insert_at(text, self.cursor.char_position()).unwrap_or(0);

                        if char_inserted > 0 {
                            let artifact = frame.act(Box::new(GridAction::Set(pos, cell)));
                            // artifact.push(frame.act(Box::new(CursorAction::CharMoveTo(cursor::PreferredCharPosition::ForwardBy(char_inserted)))));
                            self.cursor.move_to(pos, cursor::PreferredCharPosition::ForwardBy(char_inserted), &frame);


                            artifact
                        } else {
                            Artifact::EMPTY
                        }
                    }

                    // TODO: better check to avoid whitespaces
                    Event::Text(text) if text != " " => {
                        let pos = self.cursor.grid_position();
                        let mut cell = frame.grid.get(pos);

                        let char_inserted = cell.insert_at(text, self.cursor.char_position()).unwrap_or(0);

                        if char_inserted > 0 {
                            // should_merge = self.last_text_artifact_merge
                            //     .is_some_and(|timestamp| {
                            //         timestamp.elapsed()
                            //             .as_secs_f32() > 3.0
                            //     });

                            // if should_merge {
                            //     self.last_text_artifact_merge = Some(std::time::Instant::now());
                            //     dbg!("should merge");
                            // }

                            let artifact = frame.act(Box::new(GridAction::Set(pos, cell)));
                            // artifact.push(frame.act(Box::new(CursorAction::CharMoveTo(cursor::PreferredCharPosition::ForwardBy(char_inserted)))));
                            self.cursor.move_to(pos, cursor::PreferredCharPosition::ForwardBy(char_inserted), &frame);

                            artifact
                        } else {
                            Artifact::EMPTY
                        }
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
                                // let action = CursorAction::GridStepInDirection(Direction::Up, char_pos);
                                // frame.act(Box::new(action))
                                self.cursor.grid_move_to(
                                    PreferredGridPosition::InDirectionByOffset(Direction::Up, 1),
                                    &frame
                                );
                            },
                            Key::ArrowDown | Key::Enter => {
                                self.cursor.grid_move_to(
                                    PreferredGridPosition::InDirectionByOffset(Direction::Down, 1),
                                    &frame
                                );
                            }
                            Key::Tab | Key::Space => {
                                self.cursor.grid_move_to(
                                    PreferredGridPosition::InDirectionByOffset(Direction::Right, 1),
                                    &frame
                                );
                            }
                            Key::ArrowRight => {
                                let current_cell_len = frame.grid.get(self.cursor.grid_position()).len();

                                // if we are already at the end of this cell
                                if self.cursor.char_position() == current_cell_len {
                                    self.cursor.grid_move_to(
                                        PreferredGridPosition::InDirectionByOffset(Direction::Right, 1),
                                        &frame
                                    );
                                } else {
                                    self.cursor.char_move_to(
                                        PreferredCharPosition::ForwardBy(1),
                                        &frame
                                    );
                                };
                            },
                            Key::ArrowLeft => {
                                if self.cursor.char_position() == 0 {
                                    self.cursor.grid_move_to(
                                        PreferredGridPosition::InDirectionByOffset(Direction::Left, 1),
                                        &frame
                                    );
                                } else {
                                    self.cursor.char_move_to(
                                        PreferredCharPosition::BackwardBy(1),
                                        &frame
                                    );
                                }
                            },
                            _ => unreachable!(),
                        }

                        Artifact::EMPTY
                    },

                    Event::Key {
                        key: Key::Backspace,
                        pressed: true,
                        modifiers,
                        ..
                    } => {
                        let grid_pos = self.cursor.grid_position();
                        let char_pos = self.cursor.char_position();
                        let mut cell = frame.grid.get(grid_pos);

                        if char_pos > 0 {
                            let char_range = if modifiers.ctrl {
                                0..char_pos
                            } else {
                                (char_pos - 1)..char_pos
                            };

                            let chars_deleted = cell.delete_char_range(char_range).unwrap_or(0);

                            let artifact = frame.act(Box::new(GridAction::Set(grid_pos, cell)));

                            self.cursor.char_move_to(
                                PreferredCharPosition::BackwardBy(chars_deleted),
                                &frame
                            );

                            artifact
                        } else {
                            self.cursor.grid_move_to(
                                PreferredGridPosition::InDirectionByOffset(Direction::Left, 1),
                                &frame
                            );

                            Artifact::EMPTY
                        }
                    }

                    Event::Key {
                        key: Key::Delete,
                        pressed: true,
                        modifiers,
                        ..
                    } => {
                        let grid_pos = self.cursor.grid_position();
                        let char_pos = self.cursor.char_position();
                        let mut cell = frame.grid.get(grid_pos);

                        if char_pos < cell.len() {
                            let range = if modifiers.ctrl {
                                char_pos..cell.len()
                            } else {
                                char_pos..(char_pos + 1)
                            };

                            let _ = cell.delete_char_range(range);
                            frame.act(Box::new(GridAction::Set(grid_pos, cell)))
                        } else {
                            Artifact::EMPTY
                        }
                    }
                    _ => {
                        Artifact::EMPTY
                    }
                });
            }
        };

        ui.put(container_rect, GridWidget {
            transform: self.screen_transform,
            has_focus: response.has_focus(),
            cursor: self.cursor,
            head: frame.head,
            grid: &frame.grid
        });

        artifact
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

                let (stroke, stroke_kind) = if self.cursor.grid_position() == grid_pos {
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

                if self.cursor.grid_position() == grid_pos && self.has_focus {
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

                            if self.cursor.char_position() > index {
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
