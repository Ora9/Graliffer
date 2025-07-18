use egui::{epaint, NumExt, Response, Sense, TextStyle, Ui, Widget, WidgetInfo, WidgetText};

pub struct SubtleButton {
    text: WidgetText,
    selected: bool,
}

impl SubtleButton {
    pub fn new(text: impl Into<WidgetText>) -> Self {
        Self {
            text: text.into(),
            selected: false,
        }
    }
}

impl Widget for SubtleButton {
    fn ui(self, ui: &mut Ui) -> Response {
        let Self { text, selected } = self;

        let button_padding = ui.spacing().button_padding;
        let total_padding = button_padding + button_padding;

        let wrap_width = ui.available_width() - total_padding.x;
        let galley = text.into_galley(ui, None, wrap_width, TextStyle::Button);

        let mut desired_size = total_padding + galley.size();

        desired_size.y = desired_size.y.at_least(ui.spacing().interact_size.y);
        let (rect, response) = ui.allocate_at_least(desired_size, Sense::click());

        response.widget_info(|| {
            WidgetInfo::labeled(
                egui::WidgetType::Button,
                ui.is_enabled(),
                galley.text()
            )
        });

        if ui.is_rect_visible(response.rect) {
            let text_pos = ui
                .layout()
                .align_size_within_rect(galley.size(), rect.shrink2(button_padding))
                .min;

            let visuals = ui.style().interact_selectable(&response, selected);

            if selected || response.hovered() || response.highlighted() || response.has_focus() {
                let rect = rect.expand(visuals.expansion);

                ui.painter().rect(
                    rect,
                    visuals.corner_radius,
                    visuals.weak_bg_fill,
                    visuals.bg_stroke,
                    epaint::StrokeKind::Inside
                );
            }

            ui.painter().galley(text_pos, galley, visuals.text_color());
        }

        response
    }
}
