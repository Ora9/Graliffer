use std::ops::{Div, Neg};

use ratatui::layout::{Margin, Offset, Rect};

use super::{
    CELL_BORDER, CELL_HEIGHT, CELL_WIDTH, FollowCursorConfig, FollowCursorMode, GridInput,
    cursor_to_terminal_position, grid_to_terminal_position,
};

#[derive(Debug, Default, Clone, Copy)]
pub struct GridOffset {
    pub x: u32,
    pub y: u32,
}

impl GridOffset {
    pub fn with(&mut self, x: u32, y: u32, area: Rect) {
        let max = grid_to_terminal_position(grai::Position::MAX, area, GridOffset::default())
            .offset(Offset {
                x: (area
                    .width
                    .saturating_sub(5)
                    .saturating_sub(CELL_WIDTH + CELL_BORDER) as i32)
                    .neg(),
                y: (area
                    .height
                    .saturating_sub(2)
                    .saturating_sub(CELL_HEIGHT + CELL_BORDER) as i32)
                    .neg(),
            });

        self.x = x.clamp(0, max.x as u32);
        self.y = y.clamp(0, max.y as u32);
    }

    pub fn follow_cursor(
        &mut self,
        grid_input: &GridInput,
        area: Rect,
        config: FollowCursorConfig,
    ) {
        let cursor_term_pos = cursor_to_terminal_position(grid_input, area, GridOffset::default())
            .offset(Offset {
                x: (area.x as i32).neg(),
                y: (area.y as i32).neg(),
            });

        match config.follow_cursor_mode {
            FollowCursorMode::Centered => {
                self.with(
                    cursor_term_pos.x.saturating_sub(area.width.div(2)) as u32,
                    cursor_term_pos.y.saturating_sub(area.height.div(2)) as u32,
                    area,
                );
            }
            FollowCursorMode::Sticky => {
                let config_margin = config
                    .follow_cursor_sticky_margin
                    .try_into()
                    .unwrap_or(u16::MAX);

                let margin_x = (CELL_WIDTH + CELL_BORDER)
                    .saturating_mul(config_margin)
                    .max(1);

                let margin_y = (CELL_HEIGHT + CELL_BORDER)
                    .saturating_mul(config_margin)
                    .max(1);

                let cursor_box = Rect::new(
                    self.x as u16,
                    self.y as u16,
                    area.width.saturating_sub(1),
                    area.height.saturating_sub(1),
                )
                .inner(Margin::new(margin_x, margin_y));

                let offset_x = if cursor_box.width <= (CELL_WIDTH + CELL_BORDER * 2) {
                    // default to Centered mode
                    cursor_term_pos.x.saturating_sub(area.width.div(2)) as u32
                } else {
                    let right = cursor_term_pos
                        .x
                        .saturating_add(CELL_WIDTH)
                        .saturating_sub(cursor_box.right());

                    let left = cursor_box
                        .left()
                        .saturating_sub(cursor_term_pos.x.saturating_sub(CELL_BORDER));

                    self.x
                        .saturating_add(right as u32)
                        .saturating_sub(left as u32)
                };

                let offset_y = if cursor_box.height <= (CELL_HEIGHT + CELL_BORDER * 2) {
                    // default to Centered mode
                    cursor_term_pos.y.saturating_sub(area.height.div(2)) as u32
                } else {
                    let top = cursor_box
                        .top()
                        .saturating_sub(cursor_term_pos.y.saturating_sub(CELL_BORDER));

                    let bottom = cursor_term_pos
                        .y
                        .saturating_add(CELL_HEIGHT)
                        .saturating_sub(cursor_box.bottom());

                    self.y
                        .saturating_add(bottom as u32)
                        .saturating_sub(top as u32)
                };

                self.with(offset_x, offset_y, area);
            }
        }
    }
}
