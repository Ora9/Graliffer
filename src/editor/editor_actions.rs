use egui::{Event, Key};

use crate::{
    Editor, Frame, FrameAction,
    editor::{
        View,
        cursor::{PreferredCharPosition, PreferredGridPosition},
        grid_widget::GridWidgetState,
    },
    grid::{Cell, Position},
    utils::Direction,
};

/// Helper function to move the cursor when said action is FrameAction::GridSet
/// To make the cursor follow undo/redo manipulations
fn move_cursor_back_to_action(editor: &Editor, frame: &Frame, action: FrameAction) {
    if let FrameAction::GridSet(grid_pos, _) = action {
        let mut grid_state = GridWidgetState::get(&editor.egui_ctx, View::Grid).unwrap_or_default();

        if let Ok(cursor) = grid_state.cursor.with_position(
            PreferredGridPosition::At(grid_pos),
            PreferredCharPosition::AtEnd,
            &frame.grid,
        ) {
            grid_state.cursor = cursor
        }

        grid_state.set(&editor.egui_ctx, View::Grid);
    }
}

#[derive(Debug, Clone)]
pub enum EditorAction {
    /// Undo the last editor action
    Undo,
    /// Redo the last thing that was undone
    Redo,

    /// The default behavior when pressing an arrow key, stepping to the next
    /// character in a cell, or to the next cell if the cursor is at the border
    /// of a cell
    CursorStepIn(Direction),

    /// A small leap to the next cell in the direction, ignoring the current
    /// cursor char position, used for the tab, enter and space key
    CursorLeapIn(Direction),

    /// A dash to either the start/end of a cell's content, or to the next
    /// non-empty cell in the given direction
    CursorDashIn(Direction),

    /// Move the cursor to a given position in the grid
    CursorMoveTo(Position),

    GridDeleteOrCursorStepBackward,
    GridDeleteForeward,
    GridDeleteCell,
    GridDeleteCellOrCursorStepBackward,

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
                key:
                    arrow @ (Key::ArrowUp
                    | Key::ArrowRight
                    | Key::ArrowDown
                    | Key::ArrowLeft
                    | Key::Space
                    | Key::Tab
                    | Key::Enter),
                pressed: true,
                modifiers,
                ..
            } => {
                let direction = match arrow {
                    Key::ArrowUp => Direction::Up,

                    Key::ArrowRight => Direction::Right,

                    Key::Space | Key::Tab if modifiers.shift => Direction::Left,
                    Key::Space | Key::Tab => Direction::Right,

                    Key::Enter if modifiers.shift => Direction::Up,
                    Key::Enter => Direction::Down,

                    Key::ArrowDown => Direction::Down,
                    Key::ArrowLeft => Direction::Left,

                    _ => unreachable!(),
                };

                if matches!(arrow, Key::Tab | Key::Space | Key::Enter) {
                    Some(Self::CursorLeapIn(direction))
                } else if modifiers.command {
                    Some(Self::CursorDashIn(direction))
                } else {
                    Some(Self::CursorStepIn(direction))
                }
            }

            Event::Key {
                key: Key::Backspace,
                pressed: true,
                modifiers,
                ..
            } => {
                if modifiers.command {
                    Some(Self::GridDeleteCellOrCursorStepBackward)
                } else {
                    Some(Self::GridDeleteOrCursorStepBackward)
                }
            }

            Event::Key {
                key: Key::Delete,
                pressed: true,
                modifiers,
                ..
            } => {
                if modifiers.command {
                    Some(Self::GridDeleteCell)
                } else {
                    Some(Self::GridDeleteForeward)
                }
            }

            Event::Text(string) if string != " " => Some(Self::GridInsertAtCursor(string.clone())),

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
                let artifact = editor.history.redo(&mut frame);

                if let Some(action) = artifact.last_redo_action() {
                    move_cursor_back_to_action(editor, &frame, action);
                }

                editor.history_merge.cancel_all_merge();
            }
            Undo => {
                let artifact = editor.history.undo(&mut frame);

                if let Some(action) = artifact.last_undo_action() {
                    move_cursor_back_to_action(editor, &frame, action);
                }

                editor.history_merge.cancel_all_merge();
            }

            CursorDashIn(direction) => {
                let mut grid_state =
                    GridWidgetState::get(&editor.egui_ctx, View::Grid).unwrap_or_default();
                let char_pos = grid_state.cursor.char_position();
                let grid_pos = grid_state.cursor.grid_position();

                let (preferred_grid_pos, preferred_char_pos) = match direction {
                    Direction::Up | Direction::Down => (
                        PreferredGridPosition::InDirectionUntilNonEmpty(*direction),
                        PreferredCharPosition::AtEnd,
                    ),
                    Direction::Right => {
                        if char_pos >= frame.grid.get(grid_pos).len() {
                            editor.history_merge.cancel_all_merge();
                            (
                                PreferredGridPosition::InDirectionUntilNonEmpty(*direction),
                                PreferredCharPosition::AtStart,
                            )
                        } else {
                            (
                                PreferredGridPosition::Unchanged,
                                PreferredCharPosition::AtEnd,
                            )
                        }
                    }
                    Direction::Left => {
                        if char_pos == 0 {
                            editor.history_merge.cancel_all_merge();
                            (
                                PreferredGridPosition::InDirectionUntilNonEmpty(*direction),
                                PreferredCharPosition::AtEnd,
                            )
                        } else {
                            (
                                PreferredGridPosition::Unchanged,
                                PreferredCharPosition::AtStart,
                            )
                        }
                    }
                };

                if let Ok(cursor) = grid_state.cursor.with_position(
                    preferred_grid_pos,
                    preferred_char_pos,
                    &frame.grid,
                ) {
                    grid_state.cursor = cursor;
                }

                grid_state.set(&editor.egui_ctx, View::Grid);
            }

            CursorLeapIn(direction) => {
                let mut grid_state =
                    GridWidgetState::get(&editor.egui_ctx, View::Grid).unwrap_or_default();

                if let Ok(cursor) = grid_state.cursor.with_position(
                    PreferredGridPosition::InDirectionByOffset(*direction, 1),
                    PreferredCharPosition::AtEnd,
                    &frame.grid,
                ) {
                    grid_state.cursor = cursor;
                }

                editor.history_merge.cancel_all_merge();
                grid_state.set(&editor.egui_ctx, View::Grid);
            }

            CursorStepIn(direction) => {
                let mut grid_state =
                    GridWidgetState::get(&editor.egui_ctx, View::Grid).unwrap_or_default();
                let grid_pos = grid_state.cursor.grid_position();
                let char_pos = grid_state.cursor.char_position();

                let (preferred_grid_pos, preferred_char_pos) = match direction {
                    Direction::Down | Direction::Up => (
                        PreferredGridPosition::InDirectionByOffset(*direction, 1),
                        PreferredCharPosition::At(grid_state.cursor.char_position()),
                    ),
                    Direction::Right => {
                        if char_pos >= frame.grid.get(grid_pos).len() {
                            editor.history_merge.cancel_all_merge();
                            (
                                PreferredGridPosition::InDirectionByOffset(*direction, 1),
                                PreferredCharPosition::AtStart,
                            )
                        } else {
                            (
                                PreferredGridPosition::Unchanged,
                                PreferredCharPosition::ForwardBy(1),
                            )
                        }
                    }
                    Direction::Left => {
                        if char_pos == 0 {
                            editor.history_merge.cancel_all_merge();
                            (
                                PreferredGridPosition::InDirectionByOffset(*direction, 1),
                                PreferredCharPosition::AtEnd,
                            )
                        } else {
                            (
                                PreferredGridPosition::Unchanged,
                                PreferredCharPosition::BackwardBy(1),
                            )
                        }
                    }
                };

                if let Ok(cursor) = grid_state.cursor.with_position(
                    preferred_grid_pos,
                    preferred_char_pos,
                    &frame.grid,
                ) {
                    grid_state.cursor = cursor;
                }

                grid_state.set(&editor.egui_ctx, View::Grid);
            }

            CursorMoveTo(position) => {
                let mut grid_state =
                    GridWidgetState::get(&editor.egui_ctx, View::Grid).unwrap_or_default();

                if let Ok(cursor) = grid_state.cursor.with_position(
                    PreferredGridPosition::At(*position),
                    PreferredCharPosition::AtEnd,
                    &frame.grid,
                ) {
                    grid_state.cursor = cursor;
                }

                editor.history_merge.cancel_all_merge();
                grid_state.set(&editor.egui_ctx, View::Grid);
            }

            GridDeleteCell => {
                let mut grid_state =
                    GridWidgetState::get(&editor.egui_ctx, View::Grid).unwrap_or_default();

                let grid_pos = grid_state.cursor.grid_position();

                let artifact = frame.act(FrameAction::GridSet(grid_pos, Cell::new_trim("")));

                if let Ok(cursor) = grid_state
                    .cursor
                    .char_with(PreferredCharPosition::AtStart, &frame.grid)
                {
                    grid_state.cursor = cursor;
                }

                if editor.history_merge.should_merge_deletion() {
                    editor.history.merge_with_last(artifact);
                } else {
                    editor.history.append(artifact);
                }

                editor.history_merge.update_deletion_timeout();
                editor.history_merge.cancel_insertion_merge();

                frame.grid.set(grid_pos, Cell::new_trim(""));

                grid_state.set(&editor.egui_ctx, View::Grid);
            }

            GridDeleteCellOrCursorStepBackward => {
                let mut grid_state =
                    GridWidgetState::get(&editor.egui_ctx, View::Grid).unwrap_or_default();

                let grid_pos = grid_state.cursor.grid_position();
                let char_pos = grid_state.cursor.char_position();

                if char_pos > 0 {
                    let artifact = frame.act(FrameAction::GridSet(grid_pos, Cell::new_trim("")));

                    if let Ok(cursor) = grid_state
                        .cursor
                        .char_with(PreferredCharPosition::AtStart, &frame.grid)
                    {
                        grid_state.cursor = cursor;
                    }

                    if editor.history_merge.should_merge_deletion() {
                        editor.history.merge_with_last(artifact);
                    } else {
                        editor.history.append(artifact);
                    }

                    editor.history_merge.update_deletion_timeout();
                    editor.history_merge.cancel_insertion_merge();
                } else {
                    if let Ok(cursor) = grid_state.cursor.with_position(
                        PreferredGridPosition::InDirectionByOffset(Direction::Left, 1),
                        PreferredCharPosition::AtEnd,
                        &frame.grid,
                    ) {
                        grid_state.cursor = cursor;
                    }

                    editor.history_merge.cancel_all_merge();
                }

                grid_state.set(&editor.egui_ctx, View::Grid);
            }

            GridDeleteOrCursorStepBackward => {
                let mut grid_state =
                    GridWidgetState::get(&editor.egui_ctx, View::Grid).unwrap_or_default();

                let grid_pos = grid_state.cursor.grid_position();
                let char_pos = grid_state.cursor.char_position();

                if char_pos > 0 {
                    let mut cell = frame.grid.get(grid_pos);

                    let char_deleted = cell.delete_char_range(char_pos - 1..char_pos).unwrap_or(0);
                    let artifact = frame.act(FrameAction::GridSet(grid_pos, cell));

                    if let Ok(cursor) = grid_state
                        .cursor
                        .char_with(PreferredCharPosition::BackwardBy(char_deleted), &frame.grid)
                    {
                        grid_state.cursor = cursor;
                    }

                    if editor.history_merge.should_merge_deletion() {
                        editor.history.merge_with_last(artifact);
                    } else {
                        editor.history.append(artifact);
                    }

                    editor.history_merge.update_deletion_timeout();
                    editor.history_merge.cancel_insertion_merge();
                } else {
                    if let Ok(cursor) = grid_state.cursor.with_position(
                        PreferredGridPosition::InDirectionByOffset(Direction::Left, 1),
                        PreferredCharPosition::AtEnd,
                        &frame.grid,
                    ) {
                        grid_state.cursor = cursor;
                    }

                    editor.history_merge.cancel_all_merge();
                }

                grid_state.set(&editor.egui_ctx, View::Grid);
            }

            GridDeleteForeward => {
                let grid_state =
                    GridWidgetState::get(&editor.egui_ctx, View::Grid).unwrap_or_default();

                let grid_pos = grid_state.cursor.grid_position();
                let char_pos = grid_state.cursor.char_position();

                let mut cell = frame.grid.get(grid_pos);

                let _ = cell.delete_char_range(char_pos..char_pos + 1).unwrap_or(0);
                let artifact = frame.act(FrameAction::GridSet(grid_pos, cell));

                if editor.history_merge.should_merge_deletion() {
                    editor.history.merge_with_last(artifact);
                } else {
                    editor.history.append(artifact);
                }

                editor.history_merge.update_deletion_timeout();
                editor.history_merge.cancel_insertion_merge();

                grid_state.set(&editor.egui_ctx, View::Grid);
            }

            GridInsertAtCursor(string) => {
                let mut grid_state =
                    GridWidgetState::get(&editor.egui_ctx, View::Grid).unwrap_or_default();

                let grid_pos = grid_state.cursor.grid_position();

                let mut cell = frame.grid.get(grid_pos);

                let char_inserted = cell
                    .insert_at(string, grid_state.cursor.char_position())
                    .unwrap_or(0);

                if char_inserted > 0 {
                    let artifact = frame.act(FrameAction::GridSet(grid_pos, cell));

                    if let Ok(cursor) = grid_state.cursor.with_position(
                        PreferredGridPosition::At(grid_pos),
                        PreferredCharPosition::ForwardBy(char_inserted),
                        &frame.grid,
                    ) {
                        grid_state.cursor = cursor;
                    }

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
