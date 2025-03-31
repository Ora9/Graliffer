use std::ops::Neg;

use crate::{
	artifact::{Action, Artifact, History}, grid::{Cell, Direction, Grid, GridAction, Head, Position, PositionAxis}, Frame, RunDescriptor
};

use cursor::CursorAction;
use egui::{emath::TSTransform, Pos2, Rect, Vec2, Widget};

mod cursor;
pub use cursor::Cursor;

#[derive(Debug, Default)]
pub struct Editor {
    cursor: Cursor,
    grid_transform: TSTransform,
}

impl Editor {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn show(ui: &mut egui::Ui, frame: &mut Frame) -> Artifact {
        let (container_id, container_rect) = ui.allocate_space(ui.available_size());
        let response = ui.interact(container_rect, container_id, egui::Sense::click_and_drag());


        let transform =
            TSTransform::from_translation(ui.min_rect().left_top().to_vec2()) * frame.editor.grid_transform;

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
                        if let Ok(grid_pos) = Position::from_numeric(hovered_cell_x, hovered_cell_y) {
                            frame.act(Box::new(CursorAction::MoveTo(grid_pos, cursor::PreferredCharPosition::AtEnd)));
                        }
                    }
                }

                let pointer_in_layer = transform.inverse() * pointer;
                let zoom_delta = ui.ctx().input(|i| i.zoom_delta());
                let pan_delta = ui.ctx().input(|i| i.smooth_scroll_delta * 1.5);
                // let multi_touch_info = ui.ctx().input(|i| i.multi_touch());

                // Zoom in on pointer:
                frame.editor.grid_transform = frame.editor.grid_transform
                    * TSTransform::from_translation(pointer_in_layer.to_vec2())
                    * TSTransform::from_scaling(zoom_delta)
                    * TSTransform::from_translation(-pointer_in_layer.to_vec2());

                // Pan:
                frame.editor.grid_transform = TSTransform::from_translation(pan_delta * 2.0) * frame.editor.grid_transform;
            }
        }

        let event_filter = egui::EventFilter {
            horizontal_arrows: true,
            vertical_arrows: true,
            escape: true,
            tab: true,
            ..Default::default()
        };

        let mut artifact = Artifact::EMPTY;

        if response.has_focus() {
            ui.memory_mut(|mem| mem.set_focus_lock_filter(container_id, event_filter));
            let events = ui.input(|i| i.filtered_events(&event_filter));

            for event in &events {
                use {egui::Event, egui::Key};
                match event {
                    Event::Key {
                        key: egui::Key::N,
                        modifiers: egui::Modifiers::CTRL,
                        pressed: true,
                        ..
                    } => {
                        frame.step();
                    }

                    Event::Copy => {
                        let cell = frame.grid.get(frame.editor.cursor.grid_position());
                        if !cell.is_empty() {
                            ui.ctx().copy_text(cell.content());
                        }
                    }

                    Event::Cut => {
                        let pos = frame.editor.cursor.grid_position();
                        let mut cell = frame.grid.get(pos);

                        if !cell.is_empty() {
                            ui.ctx().copy_text(cell.content());

                            cell.clear();

                            artifact.append_last(frame.act(Box::new(GridAction::Set(pos, cell))));
                            artifact.append_last(frame.act(Box::new(CursorAction::CharMoveTo(cursor::PreferredCharPosition::AtStart))));;
                        }
                    }

                    Event::Paste(text) => {
                        let pos = frame.editor.cursor.grid_position();
                        let mut cell = frame.grid.get(pos);

                        let char_inserted = cell.insert_at(text, frame.editor.cursor.char_position()).unwrap_or(0);

                        artifact.append_last(frame.act(Box::new(GridAction::Set(pos, cell))));
                        artifact.append_last(frame.act(Box::new(CursorAction::CharMoveTo(cursor::PreferredCharPosition::ForwardBy(char_inserted)))));
                    }

                    // TODO: better check to avoid whitespaces
                    Event::Text(text) if text != " " => {
                        let pos = frame.editor.cursor.grid_position();
                        let mut cell = frame.grid.get(pos);

                        let char_inserted = cell.insert_at(text, frame.editor.cursor.char_position()).unwrap_or(0);

                        artifact.append_last(frame.act(Box::new(GridAction::Set(pos, cell))));
                        artifact.append_last(frame.act(Box::new(CursorAction::CharMoveTo(cursor::PreferredCharPosition::ForwardBy(char_inserted)))));
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
                                let char_pos = cursor::PreferredCharPosition::AtEnd;
                                let action = CursorAction::GridStepInDirection(Direction::Up, char_pos);

                                artifact.append_last(frame.act(Box::new(action)));
                            },
                            Key::ArrowDown | Key::Enter => {
                                let char_pos = cursor::PreferredCharPosition::AtEnd;
                                let action = CursorAction::GridStepInDirection(Direction::Down, char_pos);

                                artifact.append_last(frame.act(Box::new(action)));
                            }
                            Key::Tab | Key::Space => {
                                let char_pos = cursor::PreferredCharPosition::AtEnd;
                                let action = CursorAction::GridStepInDirection(Direction::Right, char_pos);

                                artifact.append_last(frame.act(Box::new(action)));
                            }
                            Key::ArrowRight => {
                                let current_cell_len = frame.grid.get(frame.editor.cursor.grid_position()).len();

                                // if we are already at the end of this cell
                                let action = if frame.editor.cursor.char_position() == current_cell_len {
                                    let char_pos = cursor::PreferredCharPosition::AtStart;
                                    CursorAction::GridStepInDirection(Direction::Right, char_pos)
                                } else {
                                    let char_pos = cursor::PreferredCharPosition::ForwardBy(1);
                                    CursorAction::CharMoveTo(char_pos)
                                };

                                artifact.append_last(frame.act(Box::new(action)));
                            },
                            Key::ArrowLeft => {
                                let action = if frame.editor.cursor.char_position() == 0 {
                                    let char_pos = cursor::PreferredCharPosition::AtEnd;
                                    CursorAction::GridStepInDirection(Direction::Left, char_pos)
                                } else {
                                    let char_pos = cursor::PreferredCharPosition::BackwardBy(1);
                                    CursorAction::CharMoveTo(char_pos)
                                };

                                artifact.append_last(frame.act(Box::new(action)));
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
                        let grid_pos = frame.editor.cursor.grid_position();
                        let char_pos = frame.editor.cursor.char_position();
                        let mut cell = frame.grid.get(grid_pos);

                        if char_pos > 0 {
                            let char_range = if modifiers.ctrl {
                                0..char_pos
                            } else {
                                (char_pos - 1)..char_pos
                            };

                            let chars_deleted = cell.delete_char_range(char_range).unwrap_or(0);

                            artifact.append_last(frame.act(Box::new(GridAction::Set(grid_pos, cell))));

                            let preferred_char_position = cursor::PreferredCharPosition::BackwardBy(chars_deleted);
                            artifact.append_last(frame.act(Box::new(CursorAction::CharMoveTo(preferred_char_position))));
                        } else {
                            let action = CursorAction::GridStepInDirection(
                                Direction::Left,
                                cursor::PreferredCharPosition::AtEnd
                            );
                            artifact.append_last(frame.act(Box::new(action)));
                        }
                    }

                    Event::Key {
                        key: Key::Delete,
                        pressed: true,
                        modifiers,
                        ..
                    } => {
                        let grid_pos = frame.editor.cursor.grid_position();
                        let char_pos = frame.editor.cursor.char_position();
                        let mut cell = frame.grid.get(grid_pos);

                        if char_pos < cell.len() {
                            let range = if modifiers.ctrl {
                                char_pos..cell.len()
                            } else {
                                char_pos..(char_pos + 1)
                            };

                            let _ = cell.delete_char_range(range);
                            artifact.append_last(frame.act(Box::new(GridAction::Set(grid_pos, cell))));
                        }
                    }
                    _ => {}
                }
            }
        }

        ui.put(container_rect, GridWidget {
            transform,
            has_focus: response.has_focus(),
            cursor: frame.editor.cursor,
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
