use anyhow::Context;
use crate::{
	grid::{Grid, Cell, Position},
	Frame,
	RunDescriptor,
};

use eframe::glow::ZERO;
use egui::{emath::TSTransform, Label, Pos2, Rect, Scene, TextWrapMode, Vec2, Widget};

/// A cursor wandering around a [`Grid`]
/// For now the cursor has only one [`Position`], but will probably have two in the future to represent a selection
#[derive(Debug, Clone, Copy)]
struct Cursor {
    pub position: Position,
}

impl Cursor {
    fn new(position: Position) -> Self {
        Self {
            position
        }
    }

    pub fn move_to(&mut self, position: Position) {
        self.position = position;
    }
}

pub struct GralifferApp {
    frame: Frame,
    cursor: Cursor,
    transform: TSTransform,
}

impl GralifferApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.

       	let mut initial_grid = Grid::new();
    	initial_grid.set(Position::from_textual('A', 'A').unwrap(), Cell::new("100").unwrap());
    	initial_grid.set(Position::from_textual('B', 'A').unwrap(), Cell::new("&BB").unwrap());
    	initial_grid.set(Position::from_textual('C', 'A').unwrap(), Cell::new("div").unwrap());
    	initial_grid.set(Position::from_textual('B', 'B').unwrap(), Cell::new("@CB").unwrap());
    	initial_grid.set(Position::from_textual('C', 'B').unwrap(), Cell::new("3").unwrap());
    	// initial_grid.set(Position::from_textual('D', 'A').unwrap(), Cell::new("").unwrap());
    	initial_grid.set(Position::from_textual('E', 'A').unwrap(), Cell::new("20").unwrap());
    	initial_grid.set(Position::from_textual('F', 'A').unwrap(), Cell::new("sub").unwrap());

    	let mut frame = Frame::new(RunDescriptor {
    		grid: initial_grid,
    		..Default::default()
    	});

    	for _ in 0..20 {
    		frame.step();
    	}

    	println!("last pos: {:?}", frame.head.position.as_textual());

        Self {
            frame: frame,
            transform: TSTransform::default(),
            cursor: Cursor::new(Position::ZERO),
        }
    }
}

impl eframe::App for GralifferApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
                ui.add_space(16.0);

                egui::widgets::global_theme_preference_buttons(ui);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::Window::new("Inspect ouais").show(ctx, |ui| {
                ctx.inspection_ui(ui);
            });

            egui::Window::new("Memory ouais").show(ctx, |ui| {
                ctx.memory_ui(ui);
            });

        });

        egui::Window::new("graliffer ouais").show(ctx, |ui| {

            let (container_id, container_rect) = ui.allocate_space(ui.available_size());
            let container_layer = ui.layer_id();

            let response = ui.interact(container_rect, container_id, egui::Sense::click_and_drag());

            let transform =
                TSTransform::from_translation(ui.min_rect().left_top().to_vec2()) * self.transform;

            if response.clicked() {
                response.request_focus();
            }

            // Handle pointer (drag, zoom ..)
            if let Some(pointer) = ui.ctx().input(|i| i.pointer.hover_pos()) {
                if container_rect.contains(pointer) {
                    let pointer_in_layer = transform.inverse() * pointer;
                    let zoom_delta = ui.ctx().input(|i| i.zoom_delta());
                    let pan_delta = ui.ctx().input(|i| i.smooth_scroll_delta * 1.5);
                    // let multi_touch_info = ui.ctx().input(|i| i.multi_touch());

                    // Zoom in on pointer:
                    self.transform = self.transform
                        * TSTransform::from_translation(pointer_in_layer.to_vec2())
                        * TSTransform::from_scaling(zoom_delta)
                        * TSTransform::from_translation(-pointer_in_layer.to_vec2());

                    // Pan:
                    self.transform = TSTransform::from_translation(pan_delta * 2.0) * self.transform;
                }
            }

            let event_filter = egui::EventFilter {
                horizontal_arrows: true,
                vertical_arrows: true,
                escape: true,
                ..Default::default()
            };

            ui.memory_mut(|mem| mem.set_focus_lock_filter(container_id, event_filter));
            let events = ui.input(|i| i.filtered_events(&event_filter));

            if response.has_focus() {
                for event in &events {
                    use {egui::Event, egui::Key};
                    match event {
                        Event::Key {
                            key: key @ (Key::ArrowRight | Key::ArrowDown | Key::ArrowLeft | Key::ArrowUp),
                            pressed: true,
                            ..
                        } => {
                            let pos_result = match key {
                                Key::ArrowRight => self.cursor.position.checked_increment_x(1),
                                Key::ArrowDown => self.cursor.position.checked_increment_y(1),
                                Key::ArrowLeft => self.cursor.position.checked_decrement_x(1),
                                Key::ArrowUp => self.cursor.position.checked_decrement_y(1),
                                _ => unreachable!(),
                            };

                            if let Ok(pos) = pos_result {
                                self.cursor.position = pos
                            }
                        },
                        Event::Text(text) => {
                            dbg!(text);
                        }
                        _ => {}
                    }
                }
            }

            let grid_widget = GridWidget {
                transform,
                cursor: self.cursor,
            };

            ui.put(container_rect, grid_widget);
        });
    }
}


struct GridWidget {
    cursor: Cursor,
    transform: TSTransform,
}

impl GridWidget {
    const CELL_SIZE: f32 = 50.0;
    const CELL_PADDING: f32 = 1.5;
    const CELL_FULL_SIZE: f32 = Self::CELL_SIZE + Self::CELL_PADDING;
}

impl Widget for GridWidget {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let container_id = ui.id();
        let container_rect = ui.max_rect();

        let response = ui.response();

        let (min_x, max_x, min_y, max_y) = {
            use crate::grid::PositionAxis;

            let trans_rect = self.transform.inverse().mul_rect(container_rect);

            let min_x = ((trans_rect.min.x / GridWidget::CELL_FULL_SIZE).floor() as u32).clamp(PositionAxis::MIN_NUMERIC, PositionAxis::MAX_NUMERIC);
            let max_x = ((trans_rect.max.x / GridWidget::CELL_FULL_SIZE).ceil() as u32).clamp(PositionAxis::MIN_NUMERIC, PositionAxis::MAX_NUMERIC);
            let min_y = ((trans_rect.min.y / GridWidget::CELL_FULL_SIZE).floor() as u32).clamp(PositionAxis::MIN_NUMERIC, PositionAxis::MAX_NUMERIC);
            let max_y = ((trans_rect.max.y / GridWidget::CELL_FULL_SIZE).ceil() as u32).clamp(PositionAxis::MIN_NUMERIC, PositionAxis::MAX_NUMERIC);

            (min_x, max_x, min_y, max_y)
        };

        let painter = ui.painter_at(container_rect);

        for grid_pos_y in min_y..max_y {
            for grid_pos_x in min_x..max_x {

                let screen_pos = Pos2 {
                    x: GridWidget::CELL_FULL_SIZE * (grid_pos_x as f32),
                    y: GridWidget::CELL_FULL_SIZE * (grid_pos_y as f32),
                };

                let screen_rect = self.transform.mul_rect(Rect {
                    min: screen_pos + Vec2::splat(GridWidget::CELL_PADDING),
                    max: screen_pos + Vec2::splat(GridWidget::CELL_SIZE),
                });

                let grid_pos = Position::from_numeric(grid_pos_x, grid_pos_y).unwrap();

                // let cell = self.frame.grid.get(grid_pos);

                let bg_color = if self.cursor.position == grid_pos {
                    egui::Color32::from_gray(45)
                } else {
                    egui::Color32::from_gray(27)
                };

                let bg_corner_radius = self.transform.scaling * 3.0;

                // Draw background
                painter.rect(
                    screen_rect,
                    bg_corner_radius,
                    bg_color,
                    egui::Stroke::new(self.transform.scaling * 1.0, egui::Color32::from_gray(50)),
                    egui::StrokeKind::Inside,
                );

                painter.text(
                    screen_rect.center(),
                    egui::Align2::CENTER_CENTER,
                    "oui",
                    egui::FontId::monospace(self.transform.scaling * 12.0),
                    egui::Color32::WHITE
                );
            }
        }
        response
    }
}
