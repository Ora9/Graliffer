use std::{sync::{Arc, Mutex}, time::{Duration, Instant}};

use anyhow::Context;
use egui::{emath::TSTransform, Color32, FontFamily, Pos2, Rect, RichText, Widget};

mod cursor;
use cursor::Cursor;

mod grid_widget;
use grid_widget::GridWidget;

mod console_widget;
use console_widget::ConsoleWidget;

mod stack_widget;
use stack_widget::StackWidget;

use serde::{Deserialize, Serialize};
use strum_macros::AsRefStr;

use crate::{artifact::{History}, editor::cursor::{PreferredCharPosition, PreferredGridPosition}, grid::{GridAction, Position, PositionAxis}, utils::Direction, Frame};

#[derive(Debug, Default)]
pub struct Editor {
    pub cursor: Cursor,
    pub history: History,

    // A timeout for the next acceptable text input that would be
    // merged in undo history. This is used to merge close
    // text input (timewise), and make undo/redo a bit less granular
    // `None` or any already passed timestamp would mean to create a new
    // history entry
    history_merge: HistoryMerge,
}

#[derive(Debug, Default)]
struct HistoryMerge {
    input_timeout: Option<Instant>,
    deletion_timeout: Option<Instant>,
}

impl HistoryMerge {
    const MERGE_TIMEOUT: Duration = Duration::from_secs(3);

    fn should_merge_input(&self) -> bool {
        self.input_timeout.is_some_and(|timeout| {
            Instant::now().checked_duration_since(timeout).is_none()
        })
    }

    fn should_merge_deletion(&self) -> bool {
        self.deletion_timeout.is_some_and(|timeout| {
            Instant::now().checked_duration_since(timeout).is_none()
        })
    }

    fn update_input_timeout(&mut self) {
        self.input_timeout = Instant::now().checked_add(HistoryMerge::MERGE_TIMEOUT);
    }

    fn update_deletion_timeout(&mut self) {
        self.deletion_timeout = Instant::now().checked_add(HistoryMerge::MERGE_TIMEOUT);
    }

    fn cancel_input_merge(&mut self) {
        self.input_timeout = None;
    }

    fn cancel_deletion_merge(&mut self) {
        self.deletion_timeout = None;
    }

    fn cancel_all_merge(&mut self) {
        self.cancel_input_merge();
        self.cancel_deletion_merge();
    }
}

impl Editor {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn grid_ui(&mut self, ui: &mut egui::Ui, frame: Arc<Mutex<Frame>>) {
        GridWidget::new(frame).ui(ui);
    }

    pub fn console_ui(&self, ui: &mut egui::Ui, frame: Arc<Mutex<Frame>>) {
        ConsoleWidget::new(frame).ui(ui);
    }

    pub fn stack_ui(&mut self, ui: &mut egui::Ui, frame: Arc<Mutex<Frame>>) {
        StackWidget::new(frame).ui(ui);
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
