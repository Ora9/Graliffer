use std::{
    fmt::Debug,
    hash::Hash,
    sync::{Arc, Mutex},
};

use crate::{
    editor::{cursor, Cursor, InputContext, View, ViewsIds}, grid::{Position, PositionAxis}, Frame
};
use egui::{emath::TSTransform, Context, Id, Pos2, Rect, Response, Vec2, Widget};

#[derive(Default, Debug, Clone)]
pub struct GridWidgetState {
    pub cursor: Cursor,

    // grid transform relative to the egui grid's window
    pub grid_transform: TSTransform,
    // grid transform relative to the whole egui viewport
    screen_transform: TSTransform,
}

impl GridWidgetState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get(ctx: &Context, id: impl Hash) -> Option<Self> {
        ctx.data_mut(|d| d.get_persisted(Id::new(id)))
    }

    pub fn set(self, ctx: &Context, id: impl Hash) {
        ctx.data_mut(|d| d.insert_persisted(Id::new(id), self));
    }
}

pub struct GridWidget {
    frame: Arc<Mutex<Frame>>,
}

impl GridWidget {
    pub const CELL_SIZE: f32 = 50.0;
    pub const CELL_PADDING: f32 = 1.5;
    pub const CELL_FULL_SIZE: f32 = Self::CELL_SIZE + Self::CELL_PADDING;

    // pub fn screen_to_grid_position(pos: Pos2, rect: Rect, transform: TSTransform) -> Position {
    //     let rect_t = transform.inverse().mul_rect(rect);

    //     let grid_x = PositionAxis::clamp_numeric((pos.x / GridWidget::CELL_FULL_SIZE).floor() as u32);
    //     let grid_y = PositionAxis::clamp_numeric((rect_t.min.y / GridWidget::CELL_FULL_SIZE).floor() as u32);

    //     Position::from_numeric(grid_x, grid_y).unwrap()
    // }

    // pub fn grid_to_screen_position(pos: Position, rect: Rect, transform: TSTransform) {

    // }

    pub fn new(frame: Arc<Mutex<Frame>>) -> Self {
        Self { frame }
    }

    fn handle_inputs(&mut self, state: &mut GridWidgetState, ui: &mut egui::Ui) -> egui::Response {
        let container_rect = ui.max_rect();
        let container_id = ui.id();
        let response = ui.interact(container_rect, container_id, egui::Sense::click_and_drag());

        if let Some(pointer_pos) = ui.ctx().input(|i| i.pointer.hover_pos())
            && container_rect.contains(pointer_pos)
        {
            if response.clicked_by(egui::PointerButton::Primary) {
                response.request_focus();

                // from pointer position, figure out hovered cell rect and pos
                // *_t for translated, as in grid render coordinates
                let pointer_pos_t = state.screen_transform.inverse().mul_pos(pointer_pos);
                let hovered_cell_pos_t = Pos2 {
                    x: (pointer_pos_t.x / GridWidget::CELL_FULL_SIZE).clamp(
                        PositionAxis::MIN_NUMERIC as f32,
                        PositionAxis::MAX_NUMERIC as f32,
                    ),
                    y: (pointer_pos_t.y / GridWidget::CELL_FULL_SIZE).clamp(
                        PositionAxis::MIN_NUMERIC as f32,
                        PositionAxis::MAX_NUMERIC as f32,
                    ),
                };

                // Ceil implementation says in https://doc.rust-lang.org/std/primitive.f32.html#method.ceil :
                // « Returns the smallest integer greater than or equal to state. » wich mean that 62.0 is still 62.0 not 63.0
                // So we truncate and add 1.0 instead
                let hovered_cell_rect_t = Rect {
                    min: hovered_cell_pos_t.floor() * GridWidget::CELL_FULL_SIZE,
                    max: Pos2 {
                        x: (hovered_cell_pos_t.x.trunc() + 1.0) * GridWidget::CELL_FULL_SIZE,
                        y: (hovered_cell_pos_t.y.trunc() + 1.0) * GridWidget::CELL_FULL_SIZE,
                    },
                };

                let hovered_cell_x = hovered_cell_pos_t.x.floor() as u32;
                let hovered_cell_y = hovered_cell_pos_t.y.floor() as u32;
                // let hovered_cell_pos = state.screen_transform.mul_pos(hovered_cell_pos_t);
                let hovered_cell_rect = state.screen_transform.mul_rect(hovered_cell_rect_t);

                if hovered_cell_rect.contains(pointer_pos) {
                    // TODO: move the cursor to the right spot when clicking on text
                    // Should be possible if we work on Cursor with prefered position

                    if let Ok(frame_guard) = self.frame.try_lock()
                        && let Ok(grid_pos) = Position::from_numeric(hovered_cell_x, hovered_cell_y)
                    {
                        state.cursor.move_to(
                            cursor::PreferredGridPosition::At(grid_pos),
                            cursor::PreferredCharPosition::AtEnd,
                            &frame_guard.grid,
                        );
                    }
                }
            }

            let pointer_in_layer = state.screen_transform.inverse() * pointer_pos;
            let zoom_delta = ui.ctx().input(|i| i.zoom_delta());
            let pan_delta = ui.ctx().input(|i| i.smooth_scroll_delta * 1.5);
            // let multi_touch_info = ui.ctx().input(|i| i.multi_touch());

            // Zoom in on pointer:
            state.grid_transform = state.grid_transform
                * TSTransform::from_translation(pointer_in_layer.to_vec2())
                * TSTransform::from_scaling(zoom_delta)
                * TSTransform::from_translation(-pointer_in_layer.to_vec2());

            // Pan:
            state.grid_transform =
                TSTransform::from_translation(pan_delta * 2.0) * state.grid_transform;
        }

        response
    }
}

impl Widget for GridWidget {
    fn ui(mut self, ui: &mut egui::Ui) -> egui::Response {
        let (_container_id, container_rect) = ui.allocate_space(ui.available_size());

        let mut state = GridWidgetState::get(ui.ctx(), View::Grid).unwrap_or_default();

        let response = self.handle_inputs(&mut state, ui);

        ViewsIds::insert(ui.ctx(), View::Grid, ui.id());
        if response.gained_focus() {
            InputContext::set(ui.ctx(), InputContext::Grid);
        } else if response.lost_focus() {
            InputContext::set(ui.ctx(), InputContext::None);
        }

        state.screen_transform = TSTransform::from_translation(ui.min_rect().left_top().to_vec2())
            * state.grid_transform;

        let (min_x, max_x, min_y, max_y) = {
            use crate::grid::PositionAxis;

            let rect_t = state.screen_transform.inverse().mul_rect(container_rect);

            let min_x = PositionAxis::clamp_numeric(
                (rect_t.min.x / GridWidget::CELL_FULL_SIZE).floor() as u32,
            );
            let max_x = PositionAxis::clamp_numeric(
                (rect_t.max.x / GridWidget::CELL_FULL_SIZE).ceil() as u32,
            );
            let min_y = PositionAxis::clamp_numeric(
                (rect_t.min.y / GridWidget::CELL_FULL_SIZE).floor() as u32,
            );
            let max_y = PositionAxis::clamp_numeric(
                (rect_t.max.y / GridWidget::CELL_FULL_SIZE).ceil() as u32,
            );

            (min_x, max_x, min_y, max_y)
        };

        let painter = ui.painter_at(container_rect);

        for cell_grid_pos_y in min_y..=max_y {
            for cell_grid_pos_x in min_x..=max_x {
                let cell_screen_pos = Pos2 {
                    x: GridWidget::CELL_FULL_SIZE * (cell_grid_pos_x as f32),
                    y: GridWidget::CELL_FULL_SIZE * (cell_grid_pos_y as f32),
                };

                let cell_screen_rect = state.screen_transform.mul_rect(Rect {
                    min: cell_screen_pos + Vec2::splat(GridWidget::CELL_PADDING),
                    max: cell_screen_pos + Vec2::splat(GridWidget::CELL_SIZE),
                });

                let cell_grid_pos =
                    Position::from_numeric(cell_grid_pos_x, cell_grid_pos_y).unwrap();

                let (cell, head_pos) = {
                    let frame = self
                        .frame
                        .lock()
                        .expect("Frame should be available at this point");

                    (frame.grid.get(cell_grid_pos), frame.head.position)
                };

                let bg_color = /*if state.has_focus && state.cursor.grid_position == grid_pos {
                    egui::Color32::from_gray(45)
                } else */ if head_pos == cell_grid_pos {
                    egui::Color32::from_hex("#445E93").unwrap()
                } else {
                    egui::Color32::from_gray(27)
                };

                let (stroke, stroke_kind) = if state.cursor.grid_position() == cell_grid_pos {
                    (
                        egui::Stroke::new(
                            state.screen_transform.scaling * 2.0,
                            egui::Color32::from_gray(45),
                        ),
                        egui::StrokeKind::Outside,
                    )
                } else {
                    (
                        egui::Stroke::new(
                            state.screen_transform.scaling * 1.0,
                            egui::Color32::from_gray(45),
                        ),
                        egui::StrokeKind::Inside,
                    )
                };

                let bg_corner_radius = state.screen_transform.scaling * 3.0;

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
                    egui::FontId::monospace(state.screen_transform.scaling * 12.0),
                    egui::Color32::WHITE,
                );

                // dbg!(state.cursor.grid_position() == cell_grid_pos);

                if state.cursor.grid_position() == cell_grid_pos && response.has_focus() {
                    painter.text(
                        cell_screen_rect.left_top(),
                        egui::Align2::LEFT_TOP,
                        state.cursor.char_position(),
                        egui::FontId::monospace(state.screen_transform.scaling * 9.0),
                        egui::Color32::WHITE,
                    );

                    // Blocking
                    // (total, before_cursor)
                    let (content_total_width, content_pre_cursor_width) = ui
                        .fonts(move |fonts| {
                            cell.content()
                                .chars()
                                .enumerate()
                                .map(|(index, char)| {
                                    let width = fonts.glyph_width(
                                        &egui::FontId::monospace(
                                            state.screen_transform.scaling * 12.0,
                                        ),
                                        char,
                                    );

                                    if state.cursor.char_position() > index {
                                        (width, width)
                                    } else {
                                        (width, 0.0)
                                    }
                                })
                                .reduce(|acc, e| (acc.0 + e.0, acc.1 + e.1))
                        })
                        .unwrap_or((0.0, 0.0));

                    use std::ops::Neg;
                    let center_offset = Vec2 {
                        x: (content_total_width * 0.5).neg() + content_pre_cursor_width,
                        y: 0.0,
                    };

                    painter.rect_filled(
                        Rect::from_center_size(
                            cell_screen_rect.center() + center_offset,
                            Vec2 {
                                x: state.screen_transform.scaling * 0.8,
                                y: state.screen_transform.scaling * 13.0,
                            },
                        ),
                        2.0,
                        egui::Color32::WHITE,
                    );
                }
            }
        }

        state.set(ui.ctx(), View::Grid);

        response
    }
}
