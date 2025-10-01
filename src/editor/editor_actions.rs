use egui::{Event, Key};

use crate::{
    Editor, Frame, FrameAction,
    editor::{
        View,
        cursor::{PreferredCharPosition, PreferredGridPosition},
        grid_widget::GridWidgetState,
    },
    grid::Position,
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

#[derive(Debug, Clone, Copy)]
pub enum GridDeleteRange {
    Foreward,
    Backward,
    WholeCell,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GridDeleteIfEmpty {
    StepBackward,
    StayInPlace,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CursorMovement {
    /// The default behavior when pressing an arrow key, stepping to the next
    /// character in a cell, or to the next cell if the cursor is at the border
    /// of a cell
    StepCharThenGrid(Direction),

    /// A step to the next cell in the direction, ignoring the current
    /// cursor char position, used for the tab, enter and space key
    StepGrid(Direction),

    /// A dash to either the cell's bound (start or end) or to the next non-empty
    /// cell in the direction
    DashUntilBoudsOrNonEmpty(Direction),

    /// Move the cursor to a given position in the grid
    Jump(Position)
}

#[derive(Debug, Clone)]
pub enum EditorAction {
    /// Undo the last editor action
    Undo,
    /// Redo the last thing that was undone
    Redo,

    Copy,
    Cut,
    Paste(String),

    CursorMove(CursorMovement),

    /// Delete a range of the cell under the cursor
    GridDelete(GridDeleteRange, GridDeleteIfEmpty),

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

            Event::Copy => Some(Self::Copy),
            Event::Cut => Some(Self::Cut),
            Event::Paste(string) => Some(Self::Paste(string.to_owned())),

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
                    Some(Self::CursorMove(CursorMovement::StepGrid(direction)))
                } else if modifiers.command {
                    Some(Self::CursorMove(CursorMovement::DashUntilBoudsOrNonEmpty(direction)))
                } else {
                    Some(Self::CursorMove(CursorMovement::StepCharThenGrid(direction)))
                }
            }

            Event::Key {
                key: Key::Backspace,
                pressed: true,
                modifiers,
                ..
            } => {
                if modifiers.command {
                    Some(Self::GridDelete(
                        GridDeleteRange::WholeCell,
                        GridDeleteIfEmpty::StepBackward,
                    ))
                } else {
                    Some(Self::GridDelete(
                        GridDeleteRange::Backward,
                        GridDeleteIfEmpty::StepBackward,
                    ))
                }
            }

            Event::Key {
                key: Key::Delete,
                pressed: true,
                modifiers,
                ..
            } => {
                if modifiers.command {
                    Some(Self::GridDelete(
                        GridDeleteRange::WholeCell,
                        GridDeleteIfEmpty::StayInPlace,
                    ))
                } else {
                    Some(Self::GridDelete(
                        GridDeleteRange::Foreward,
                        GridDeleteIfEmpty::StayInPlace,
                    ))
                }
            }

            Event::Text(string) if string != " " => Some(Self::GridInsertAtCursor(string.clone())),

            _ => None,
        }
    }

    pub fn act(&self, editor: &mut Editor) {
        use EditorAction::*;
        match self {
            Redo | Undo => {
                let mut frame = editor
                    .frame
                    .lock()
                    .expect("Should be able to get the frame");

                let action_opt = match self {
                    Redo => {
                        let artifact = editor.history.redo(&mut frame);
                        artifact.last_redo_action()
                    }
                    Undo => {
                        let artifact = editor.history.undo(&mut frame);
                        artifact.last_undo_action()
                    }
                    _ => unreachable!()
                };

                if let Some(action) = action_opt {
                    move_cursor_back_to_action(editor, &frame, action);
                }

                editor.history_merge.cancel_all_merge();
            }

            Copy => {
                let frame = editor
                    .frame
                    .lock()
                    .expect("Should be able to get the frame");

                let grid_state =
                    GridWidgetState::get(&editor.egui_ctx, View::Grid).unwrap_or_default();

                let grid_pos = grid_state.cursor.grid_position();
                let cell = frame.grid.get(grid_pos);

                if !cell.is_empty() {
                    editor.egui_ctx.copy_text(cell.content());
                }
            }

            Cut => {
            }

            Paste(_text) => {

            }

            CursorMove(movement) => {
                let frame = editor
                    .frame
                    .lock()
                    .expect("Should be able to get the frame");

                let mut grid_state =
                    GridWidgetState::get(&editor.egui_ctx, View::Grid).unwrap_or_default();
                let char_pos = grid_state.cursor.char_position();
                let grid_pos = grid_state.cursor.grid_position();

                let at_end = char_pos >= frame.grid.get(grid_pos).len();
                let at_start = char_pos == 0;

                let (preferred_grid_pos, preferred_char_pos) = match movement {
                    CursorMovement::Jump(position) => (
                        PreferredGridPosition::At(*position),
                        PreferredCharPosition::AtEnd,
                    ),
                    CursorMovement::StepGrid(direction) => (
                        PreferredGridPosition::InDirectionByOffset(*direction, 1),
                        PreferredCharPosition::AtEnd,
                    ),
                    CursorMovement::StepCharThenGrid(direction) => {
                        match direction {
                            Direction::Down | Direction::Up => (
                                PreferredGridPosition::InDirectionByOffset(*direction, 1),
                                PreferredCharPosition::AtMost(grid_state.cursor.char_position()),
                            ),

                            Direction::Right if at_end => (
                                PreferredGridPosition::InDirectionByOffset(*direction, 1),
                                PreferredCharPosition::AtStart,
                            ),
                            Direction::Right => (
                                PreferredGridPosition::Unchanged,
                                PreferredCharPosition::ForwardBy(1),
                            ),

                            Direction::Left if at_start => (
                                PreferredGridPosition::InDirectionByOffset(*direction, 1),
                                PreferredCharPosition::AtEnd,
                            ),
                            Direction::Left => (
                                PreferredGridPosition::Unchanged,
                                PreferredCharPosition::BackwardBy(1),
                            ),
                        }
                    },
                    CursorMovement::DashUntilBoudsOrNonEmpty(direction) => {
                        match direction {
                            Direction::Up | Direction::Down => (
                                PreferredGridPosition::InDirectionUntilNonEmpty(*direction),
                                PreferredCharPosition::AtEnd,
                            ),

                            Direction::Right if at_end => (
                                PreferredGridPosition::InDirectionUntilNonEmpty(*direction),
                                PreferredCharPosition::AtStart,
                            ),
                            Direction::Right => (
                                PreferredGridPosition::Unchanged,
                                PreferredCharPosition::AtEnd,
                            ),

                            Direction::Left if at_start => (
                                PreferredGridPosition::InDirectionUntilNonEmpty(*direction),
                                PreferredCharPosition::AtEnd,
                            ),
                            Direction::Left => (
                                PreferredGridPosition::Unchanged,
                                PreferredCharPosition::AtStart,
                            )
                        }
                    }
                };

                if preferred_char_pos != PreferredCharPosition::Unchanged {
                    editor.history_merge.cancel_all_merge();
                }

                if let Ok(cursor) = grid_state.cursor.with_position(
                    preferred_grid_pos,
                    preferred_char_pos,
                    &frame.grid,
                ) {
                    grid_state.cursor = cursor;
                    grid_state.set(&editor.egui_ctx, View::Grid);
                }
            }

            GridDelete(grid_delete_range, grid_delete_if_empty) => {
                let mut grid_state =
                    GridWidgetState::get(&editor.egui_ctx, View::Grid).unwrap_or_default();

                let mut frame = editor
                    .frame
                    .lock()
                    .expect("Should be able to get the frame");

                let grid_pos = grid_state.cursor.grid_position();
                let char_pos = grid_state.cursor.char_position();

                if *grid_delete_if_empty == GridDeleteIfEmpty::StepBackward && char_pos == 0 {
                    if let Ok(cursor) = grid_state.cursor.with_position(
                        PreferredGridPosition::InDirectionByOffset(Direction::Left, 1),
                        PreferredCharPosition::AtEnd,
                        &frame.grid,
                    ) {
                        grid_state.cursor = cursor;
                    }

                    editor.history_merge.cancel_all_merge();
                } else {
                    let mut cell = frame.grid.get(grid_pos);

                    let range = match grid_delete_range {
                        GridDeleteRange::Backward => char_pos - 1 .. char_pos,
                        GridDeleteRange::Foreward => char_pos .. char_pos + 1,
                        GridDeleteRange::WholeCell => 0 .. cell.len(),
                    };

                    let char_deleted = cell.delete_char_range(range).unwrap_or(0);

                    let preferred_char_pos = match grid_delete_range {
                        GridDeleteRange::Backward => PreferredCharPosition::BackwardBy(char_deleted),
                        GridDeleteRange::Foreward => PreferredCharPosition::ForwardBy(char_deleted),
                        GridDeleteRange::WholeCell => PreferredCharPosition::AtEnd,
                    };

                    let artifact = frame.act(FrameAction::GridSet(grid_pos, cell));

                    if let Ok(cursor) = grid_state
                        .cursor
                        .char_with(preferred_char_pos, &frame.grid)
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
                }

                grid_state.set(&editor.egui_ctx, View::Grid);
            }

            GridInsertAtCursor(string) => {
                let mut frame = editor
                    .frame
                    .lock()
                    .expect("Should be able to get the frame");

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
