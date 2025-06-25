use egui::{emath::TSTransform, Pos2, Rect, Vec2, Widget};
use crate::grid::{Grid, Head, Position, PositionAxis};

use super::cursor::Cursor;

pub struct GridWidget<'a> {
    pub cursor: Cursor,
    pub head: Head,
    pub grid: &'a Grid,

    pub transform: TSTransform,

    pub has_focus: bool,
}

impl<'a> GridWidget<'a> {
    pub const CELL_SIZE: f32 = 50.0;
    pub const CELL_PADDING: f32 = 1.5;
    pub const CELL_FULL_SIZE: f32 = Self::CELL_SIZE + Self::CELL_PADDING;

    pub fn screen_to_grid_position(pos: Pos2, rect: Rect, transform: TSTransform) -> Position {
        let rect_t = transform.inverse().mul_rect(rect);

        let grid_x = PositionAxis::clamp_numeric((pos.x / GridWidget::CELL_FULL_SIZE).floor() as u32);
        let grid_y = PositionAxis::clamp_numeric((rect_t.min.y / GridWidget::CELL_FULL_SIZE).floor() as u32);

        Position::from_numeric(grid_x, grid_y).unwrap()
    }

    pub fn grid_to_screen_position(pos: Position, rect: Rect, transform: TSTransform) {

    }
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

        for cell_grid_pos_y in min_y..=max_y {
            for cell_grid_pos_x in min_x..=max_x {
                let cell_screen_pos = Pos2 {
                    x: GridWidget::CELL_FULL_SIZE * (cell_grid_pos_x as f32),
                    y: GridWidget::CELL_FULL_SIZE * (cell_grid_pos_y as f32),
                };

                let cell_screen_rect = self.transform.mul_rect(Rect {
                    min: cell_screen_pos + Vec2::splat(GridWidget::CELL_PADDING),
                    max: cell_screen_pos + Vec2::splat(GridWidget::CELL_SIZE),
                });

                let cell_grid_pos = Position::from_numeric(cell_grid_pos_x, cell_grid_pos_y).unwrap();

                let cell = self.grid.get(cell_grid_pos);

                let bg_color = /*if self.has_focus && self.cursor.grid_position == grid_pos {
                    egui::Color32::from_gray(45)
                } else */ if self.head.position == cell_grid_pos {
                    egui::Color32::from_hex("#445E93").unwrap()
                } else {
                    egui::Color32::from_gray(27)
                };

                let (stroke, stroke_kind) = if self.cursor.grid_position() == cell_grid_pos {
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

                painter.rect(
                    cell_screen_rect,
                    bg_corner_radius,
                    bg_color,
                    stroke,
                    stroke_kind,
                );

                painter.text(
                    cell_screen_rect.center(),
                    egui::Align2::CENTER_CENTER,
                    cell.content(),
                    egui::FontId::monospace(self.transform.scaling * 12.0),
                    egui::Color32::WHITE
                );

                // dbg!(self.cursor.grid_position() == cell_grid_pos);

                if self.cursor.grid_position() == cell_grid_pos && self.has_focus {
                    painter.text(
                        cell_screen_rect.left_top(),
                        egui::Align2::LEFT_TOP,
                        self.cursor.char_position(),
                        egui::FontId::monospace(self.transform.scaling * 9.0),
                        egui::Color32::WHITE
                    );

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

                    use std::ops::Neg;
                    let center_offset = Vec2 {
                        x: (content_total_width * 0.5).neg() + content_pre_cursor_width,
                        y: 0.0
                    };

                    painter.rect_filled(
                        Rect::from_center_size(cell_screen_rect.center() + center_offset, Vec2 {
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
