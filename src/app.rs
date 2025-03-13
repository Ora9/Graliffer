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

        // });

        // egui::Window::new("graliffer ouais").show(ctx, |ui| {

            let (container_id, container_rect) = ui.allocate_space(ui.available_size());
            let container_layer = ui.layer_id();


            let response = ui.interact(container_rect, container_id, egui::Sense::click_and_drag());

            let transform =
                TSTransform::from_translation(ui.min_rect().left_top().to_vec2()) * self.transform;

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

            // if response.has_focus() {
                for event in &events {
                    use {egui::Event, egui::Key};
                    match event {
                        // Event::MouseMoved(pointer) => {
                        //     dbg!("Pointer default!", pointer);
                        // },

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
                            }.context("could not step into darkness, the position is invalid");

                            if let Ok(pos) = pos_result {
                                self.cursor.position = pos
                            }
                        },
                        Event::Text(text) => {
                            dbg!(text);
                        }
                        _ => {}
                    }
                // }
            }

            let grid_widget = GridWidget {
                transform,
                cursor: self.cursor,
            };

            let grid_area = egui::Area::new(container_id.with("grid"))
                .fixed_pos(container_rect.min)
                .order(container_layer.order)
                .show(ui.ctx(), |ui| {
                    ui.put(container_rect, grid_widget);
                })
                .response;
            // ui.ctx().set_transform_layer(grid_area.layer_id, transform);
            ui.ctx().set_sublayer(container_layer, grid_area.layer_id);


        // egui::Window::new("Grid ouais").show(ctx, |ui| {
            // let (container_id, container_rect) = ui.allocate_space(ui.available_size());
            // let container_layer = ui.layer_id();

            // // // if response.dragged() {
            // // //     self.transform.translation += response.drag_delta();
            // // // }

            // // if response.double_clicked() {
            // //     self.transform = TSTransform::default();
            // // }
            // //

            // let transform =
            //     TSTransform::from_translation(ui.min_rect().left_top().to_vec2()) * self.transform;


            // let event_filter = egui::EventFilter {
            //     horizontal_arrows: true,
            //     vertical_arrows: true,
            //     escape: true,
            //     ..Default::default()
            // };


            // let size = ui.spacing().interact_size.y * egui::vec2(2.0, 2.0);
            // let padding = 2.5;
            // let full_size = size.x + padding;

            // let (min_x, max_x, min_y, max_y) = {
            //     use crate::grid::PositionAxis;

            //     let container_size = Rect { min: Pos2::ZERO, max: container_rect.size().to_pos2() };
            //     let a = self.transform.inverse().mul_rect(container_size);

            //     let min_x = ((a.min.x / full_size).floor() as u32).clamp(PositionAxis::MIN_NUMERIC, PositionAxis::MAX_NUMERIC);
            //     let max_x = ((a.max.x / full_size).ceil() as u32).clamp(PositionAxis::MIN_NUMERIC, PositionAxis::MAX_NUMERIC);
            //     let min_y = ((a.min.y / full_size).floor() as u32).clamp(PositionAxis::MIN_NUMERIC, PositionAxis::MAX_NUMERIC);
            //     let max_y = ((a.max.y / full_size).ceil() as u32).clamp(PositionAxis::MIN_NUMERIC, PositionAxis::MAX_NUMERIC);

            //     (min_x, max_x, min_y, max_y)
            // };

            // let painter = ui.painter_at(container_rect);

            // for y in min_y..max_y {
            //     for x in min_x..max_x {
            //         let cell_pos = Pos2 {
            //             x: (size.x + padding) * x as f32,
            //             y: (size.y + padding) * y as f32,
            //         };

            //         let cell_rect = Rect {
            //             min: cell_pos,
            //             max: cell_pos + Vec2::splat(full_size),
            //         };
            //         let grid_pos = Position::from_numeric(x, y).unwrap();

            //         // Focus on click
            //         if ui.rect_contains_pointer(transform.mul_rect(cell_rect)) && response.clicked_by(egui::PointerButton::Primary) {
            //             response.request_focus();
            //             self.cursor.position = grid_pos;
            //         }

            //         let cell = self.frame.grid.get(grid_pos);

            //         let bg_color = if self.cursor.position == grid_pos {
            //             egui::Color32::from_gray(45)
            //         } else {
            //             egui::Color32::from_gray(27)
            //         };

            //         let corner_radius = 0;
            //         painter.rect(
            //             transform.mul_rect(cell_rect),
            //             corner_radius,
            //             bg_color,
            //             egui::Stroke::new(1.0, egui::Color32::from_gray(50)),
            //             egui::StrokeKind::Inside,
            //         );

            //         // dbg!(container_rect);

            //         if !cell.is_empty() {
            //             let subarea = egui::Area::new(container_id.with(("cell", x, y)))
            //                 .fixed_pos(cell_pos)
            //                 .constrain(false)
            //                 .order(container_layer.order)
            //                 .show(ui.ctx(), |ui| {
            //                     ui.set_clip_rect(transform.inverse().mul_rect(container_rect));
            //                     let label = Label::new(cell.content())
            //                         .selectable(false);

            //                     ui.put(cell_rect, label);
            //                 })
            //                 .response;
            //             ui.ctx().set_transform_layer(subarea.layer_id, transform);
            //             ui.ctx().set_sublayer(container_layer, subarea.layer_id);
            //         }
            //     }
            // }
        });
    }
}


struct GridWidget {
    cursor: Cursor,
    transform: TSTransform,
}

impl GridWidget {
    const CELL_SIZE: f32 = 50.0;
    const CELL_PADDING: f32 = 2.5;
    const CELL_FULL_SIZE: f32 = Self::CELL_SIZE + Self::CELL_PADDING;
}

impl Widget for GridWidget {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        // let (container_id, container_rect) = ui.allocate_space(ui.available_size());
        let container_id = ui.id();
        let container_rect = ui.max_rect();

        let response = ui.interact(container_rect, container_id, egui::Sense::click_and_drag());

        let (min_x, max_x, min_y, max_y) = {
            use crate::grid::PositionAxis;

            let container_size = Rect { min: Pos2::ZERO, max: container_rect.size().to_pos2() };
            let a = self.transform.inverse().mul_rect(container_rect);
            dbg!(container_size, self.transform);

            let min_x = ((a.min.x / GridWidget::CELL_FULL_SIZE).floor() as u32).clamp(PositionAxis::MIN_NUMERIC, PositionAxis::MAX_NUMERIC);
            let max_x = ((a.max.x / GridWidget::CELL_FULL_SIZE).ceil() as u32).clamp(PositionAxis::MIN_NUMERIC, PositionAxis::MAX_NUMERIC);
            let min_y = ((a.min.y / GridWidget::CELL_FULL_SIZE).floor() as u32).clamp(PositionAxis::MIN_NUMERIC, PositionAxis::MAX_NUMERIC);
            let max_y = ((a.max.y / GridWidget::CELL_FULL_SIZE).ceil() as u32).clamp(PositionAxis::MIN_NUMERIC, PositionAxis::MAX_NUMERIC);

            (min_x, max_x, min_y, max_y)
        };

        dbg!((min_x, max_x, min_y, max_y));

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

        // ui.ctx().set_transform_layer(ui.layer_id(), self.transform);

        response
    }
}

// struct CellWidget<'a> {
//     cell: &'a mut Cell,
//     focused: bool
// }

// impl<'a> CellWidget<'a> {
//     fn size_for_ui(ui: &egui::Ui) -> Vec2 {
//         ui.spacing().interact_size.y * egui::vec2(2.0, 2.0)
//     }
// }

// impl<'a> Widget for CellWidget<'a> {
//     fn ui(self, ui: &mut egui::Ui) -> egui::Response {
//         let desired_size = Self::size_for_ui(&ui);

//         let response = ui.response();
//         let (id, rect) = ui.allocate_space(desired_size);

//         if ui.is_rect_visible(rect) {

//             // let visuals = ui.style().interact(&response);

//             // let = ui.style().noninteractive();

//             let bg_color = if self.focused {
//                 egui::Color32::from_gray(45)
//             } else {
//                 egui::Color32::from_gray(27)
//             };

//             let corner_radius = 0.1 * rect.height();
//             ui.painter().rect(
//                 transform,
//                 corner_radius,
//                 bg_color,
//                 egui::Stroke::new(1.0, egui::Color32::from_gray(50)),
//                 egui::StrokeKind::Inside,
//             );

//             // self.cell.set("oui").unwrap();

//             // if self.focused {
//             //     ui.put(rect, egui::TextEdit::singleline(self.cell));
//             // }
//             if !self.cell.is_empty() {
//                 ui.put(rect, Label::new(self.cell.content()).selectable(false));
//             }
//         }

//         response

//     }
// }

// // pub fn cell_widget(ui: &mut egui::Ui) -> egui::Response {
// //     // Widget code can be broken up in four steps:
// //     //  1. Decide a size for the widget
// //     //  2. Allocate space for it
// //     //  3. Handle interactions with the widget (if any)
// //     //  4. Paint the widget

// //     // 1. Deciding widget size:
// //     // You can query the `ui` how much space is available,
// //     // but in this example we have a fixed size widget based on the height of a standard button:
// //     let desired_size = ui.spacing().interact_size.y * egui::vec2(2.0, 2.0);
// //     // let desired_size = Vec2 { x: 5.0, y: 5.0 };

// //     // 2. Allocating space:
// //     // This is where we get a region of the screen assigned.
// //     // We also tell the Ui to sense clicks in the allocated region.
// //     let (rect, mut response) = ui.allocate_exact_size(desired_size, egui::Sense::click());

// //     // 3. Interact: Time to check for clicks!
// //     if response.clicked() {
// //         *on = !*on;
// //         response.mark_changed(); // report back that the value changed
// //     }

// //     // Attach some meta-data to the response which can be used by screen readers:
// //     response.widget_info(|| {
// //         egui::WidgetInfo::selected(egui::WidgetType::Checkbox, ui.is_enabled(), *on, "")
// //     });

// //     // 4. Paint!
// //     // Make sure we need to paint:
// //     if ui.is_rect_visible(rect) {
// //         // We will follow the current style by asking
// //         // "how should something that is being interacted with be painted?".
// //         // This will, for instance, give us different colors when the widget is hovered or clicked.
// //         let visuals = ui.style().interact(&response);
// //         // All coordinates are in absolute screen coordinates so we use `rect` to place the elements.
// //         let rect = rect.expand(visuals.expansion);
// //         let radius = 0.1 * rect.height();
// //         ui.painter().rect(
// //             rect,
// //             radius,
// //             visuals.bg_fill,
// //             visuals.bg_stroke,
// //             egui::StrokeKind::Inside,
// //         );
// //         // Paint the circle, animating it from left to right with `how_on`:
// //         // let circle_x = egui::lerp((rect.left() + radius)..=(rect.right() - radius), how_on);
// //         // let center = egui::pos2(circle_x, rect.center().y);
// //         // ui.painter()
// //         //     .circle(center, 0.75 * radius, visuals.bg_fill, visuals.fg_stroke);

// //         ui.put(rect, Label::new("hlt").selectable(false));
// //     }

// //     // All done! Return the interaction response so the user can check what happened
// //     // (hovered, clicked, ...) and maybe show a tooltip:
// //     response
// // }
