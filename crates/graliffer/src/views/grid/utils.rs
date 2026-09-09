use ratatui::layout::{Offset, Position, Rect};

use super::{GridInput, GridOffset};

pub const CELL_WIDTH: u16 = 3;
pub const CELL_HEIGHT: u16 = 1;
pub const CELL_BORDER: u16 = 1;

pub fn terminal_to_grid_position(
    terminal_position: Position,
    area: Rect,
    offset: GridOffset,
) -> Option<grai::Position> {
    grai::Position::new(
        terminal_position
            .x
            .checked_add(offset.x as u16)?
            .saturating_sub(area.x)
            .saturating_sub(CELL_BORDER)
            .checked_div(CELL_WIDTH + CELL_BORDER)? as u32,
        terminal_position
            .y
            .checked_add(offset.y as u16)?
            .saturating_sub(area.y)
            .saturating_sub(CELL_BORDER)
            .checked_div(CELL_HEIGHT + CELL_BORDER)? as u32,
    )
    .ok()
}

pub fn cursor_to_terminal_position(
    grid_input: &GridInput,
    area: Rect,
    grid_offset: GridOffset,
) -> Position {
    let cursor_term_origin = grid_to_terminal_position(grid_input.grid_cursor(), area, grid_offset);

    Position::new(cursor_term_origin.x, cursor_term_origin.y)
        .offset(Offset::new(CELL_BORDER as i32, CELL_BORDER as i32))
        .offset(Offset::new(grid_input.char_cursor() as i32, 0))
}

pub fn grid_to_terminal_position(
    grid_position: grai::Position,
    area: Rect,
    offset: GridOffset,
) -> Position {
    Position {
        x: area
            .x
            .strict_add(grid_position.x() as u16 * (CELL_WIDTH + CELL_BORDER))
            .saturating_sub(offset.x as u16),
        y: area
            .y
            .strict_add(grid_position.y() as u16 * (CELL_HEIGHT + CELL_BORDER))
            .saturating_sub(offset.y as u16),
    }
}
