use egui::{Event, Key};

use crate::{editor::{cursor::{PreferredCharPosition, PreferredGridPosition}, grid_widget::GridWidgetState, View}, grid::{Cell, Position}, utils::Direction, Editor, FrameAction};

#[derive(Debug, Clone)]
pub enum EditorAction {
    Undo,
    Redo,

    CursorStepIn(Direction),
    CursorLeapIn(Direction),
    CursorMoveTo(Position),

    GridDeleteOrCursorStepBackward,
    GridDeleteForeward,
    GridDeleteCell,

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
            } if modifiers.command => Some(Self::Undo),
            Event::Key {
                key: Key::Y,
                modifiers,
                pressed: true,
                ..
            } if modifiers.command => Some(Self::Redo),

            Event::Key {
                key: arrow @ (Key::ArrowUp | Key::ArrowRight | Key::ArrowDown | Key::ArrowLeft),
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
                    _ => {
                        unreachable!();
                    }
                };

                if modifiers.command {
                    Some(Self::CursorLeapIn(direction))
                } else {
                    Some(Self::CursorStepIn(direction))
                }
            }

            Event::Key {
                key: key @ (Key::Backspace | Key::Delete),
                pressed: true,
                modifiers,
                ..
            } => {
                if modifiers.command {
                    Some(Self::GridDeleteCell)
                } else {
                    match key {
                        Key::Backspace => Some(Self::GridDeleteOrCursorStepBackward),
                        Key::Delete => Some(Self::GridDeleteForeward),
                        _ => unreachable!(),
                    }
                }
            }

            Event::Text(string) => Some(Self::GridInsertAtCursor(string.clone())),

            _ => None,
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

            CursorLeapIn(direction) => {}

            CursorStepIn(direction) => {
                let mut grid_state =
                    GridWidgetState::get(&editor.egui_ctx, View::Grid).unwrap_or_default();
                let grid_pos = grid_state.cursor.grid_position();
                let char_pos = grid_state.cursor.char_position();

                match direction {
                    Direction::Down | Direction::Up => grid_state.cursor.move_to(
                        PreferredGridPosition::InDirectionByOffset(*direction, 1),
                        PreferredCharPosition::At(grid_state.cursor.char_position()),
                        &frame.grid,
                    ),
                    Direction::Right => {
                        if char_pos >= frame.grid.get(grid_pos).len() {
                            grid_state.cursor.move_to(
                                PreferredGridPosition::InDirectionByOffset(*direction, 1),
                                PreferredCharPosition::AtEnd,
                                &frame.grid,
                            )
                        } else {
                            grid_state.cursor.move_to(
                                PreferredGridPosition::Unchanged,
                                PreferredCharPosition::ForwardBy(1),
                                &frame.grid,
                            )
                        }
                    }
                    Direction::Left => {
                        if char_pos == 0 {
                            grid_state.cursor.move_to(
                                PreferredGridPosition::InDirectionByOffset(*direction, 1),
                                PreferredCharPosition::AtStart,
                                &frame.grid,
                            )
                        } else {
                            grid_state.cursor.move_to(
                                PreferredGridPosition::Unchanged,
                                PreferredCharPosition::BackwardBy(1),
                                &frame.grid,
                            )
                        }
                    }
                };

                grid_state.set(&editor.egui_ctx, View::Grid);
            }

            CursorMoveTo(position) => {
                let mut grid_state =
                    GridWidgetState::get(&editor.egui_ctx, View::Grid).unwrap_or_default();

                grid_state.cursor.move_to(
                    PreferredGridPosition::At(*position),
                    PreferredCharPosition::AtEnd,
                    &frame.grid,
                );
                grid_state.set(&editor.egui_ctx, View::Grid);
            }

            GridDeleteCell => {
                let grid_state =
                    GridWidgetState::get(&editor.egui_ctx, View::Grid).unwrap_or_default();

                let pos = grid_state.cursor.grid_position();
                frame.grid.set(pos, Cell::new_trim(""));

                grid_state.set(&editor.egui_ctx, View::Grid);
            }

            GridDeleteOrCursorStepBackward => {
                let grid_state =
                    GridWidgetState::get(&editor.egui_ctx, View::Grid).unwrap_or_default();

                let pos = grid_state.cursor.grid_position();

                grid_state.set(&editor.egui_ctx, View::Grid);
            }

            GridDeleteForeward => {}

            GridInsertAtCursor(string) => {
                let mut grid_state =
                    GridWidgetState::get(&editor.egui_ctx, View::Grid).unwrap_or_default();

                let pos = grid_state.cursor.grid_position();

                let mut cell = frame.grid.get(pos);

                dbg!(pos, string);

                let char_inserted = cell
                    .insert_at(string, grid_state.cursor.char_position())
                    .unwrap_or(0);

                if char_inserted > 0 {
                    let artifact = frame.act(FrameAction::GridSet(pos, cell));
                    grid_state.cursor.move_to(
                        PreferredGridPosition::At(pos),
                        PreferredCharPosition::ForwardBy(char_inserted),
                        &frame.grid,
                    );

                    if editor.history_merge.should_merge_insertion() {
                        dbg!("Merging !");
                        editor.history.merge_with_last(artifact);
                    } else {
                        editor.history.append(artifact);
                    }

                    editor.history_merge.update_insertion_timeout();
                    editor.history_merge.cancel_deletion_merge();
                }

                grid_state.set(&editor.egui_ctx, View::Grid);
            }
        }
    }
}
