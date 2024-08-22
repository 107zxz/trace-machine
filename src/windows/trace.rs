use std::hash::{Hash, Hasher};
use eframe::egui::*;

pub struct Trace {
    pub window_title: String,
    lines: Vec<Vec<Pos2>>,
    pub stroke: Stroke,
    pub open: bool,
    pub enabled: bool,
    window_id: Id,
    pub layer_id: LayerId
}

impl Trace {
    pub fn new(title: impl Into<String>, color: Color32) -> Self {

        let window_title = title.into();
        let window_id = Id::new(window_title.clone());
        let layer_id = LayerId::new(Order::Foreground, window_id);

        Trace {
            window_title,
            lines: Vec::new(),
            stroke: Stroke::new(3.0, color),
            open: true,
            enabled: true,
            window_id,
            layer_id
        }
    }

    pub fn trace(&mut self, ctx: &Context) {

        // Workaround: close button
        Window::new(&self.window_title)
            .frame(Frame::none())
            .order(Order::Foreground)
            .id(self.window_id)
            .open(&mut self.open)
            .enabled(self.enabled)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.add(&mut self.stroke);
                    if ui.button("clear").clicked() {
                        self.lines.clear();
                    }
                });

                let response = Frame::canvas(ui.style()).fill(Color32::TRANSPARENT).show(ui, |ui| {
                    let (mut response, painter) = ui.allocate_painter(ui.available_size(), Sense::drag());

                    let to_screen = emath::RectTransform::from_to(
                        Rect::from_min_size(Pos2::ZERO, response.rect.square_proportions()),
                        response.rect,
                    );

                    let from_screen = to_screen.inverse();

                    if self.lines.is_empty() {
                        self.lines.push(vec![]);
                    }

                    let current_line = self.lines.last_mut().unwrap();

                    if let Some(pointer_pos) = response.interact_pointer_pos() {
                        let canvas_pos = from_screen * pointer_pos;
                        if current_line.last() != Some(&canvas_pos) {
                            current_line.push(canvas_pos);
                            response.mark_changed();
                        }
                    } else if !current_line.is_empty() {
                        self.lines.push(vec![]);
                        response.mark_changed();
                    }

                    let shapes = self
                        .lines
                        .iter()
                        .filter(|line| line.len() >= 2)
                        .map(|line| {
                            let points: Vec<Pos2> = line.iter().map(|p| to_screen * *p).collect();
                            Shape::line(points, self.stroke)
                        });

                    painter.extend(shapes);
                });

                response.response.on_hover_cursor(CursorIcon::Crosshair);
            });
    }
}

impl Hash for Trace {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.window_title.hash(state);
    }
}