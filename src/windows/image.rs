use eframe::egui::*;

pub struct ImageRef {
    path: String,
    title: String,
    pub open: bool,
}

impl ImageRef {
    pub fn new(path: String, title: String) -> Self {
        ImageRef {
            path,
            title,
            open: true,
        }
    }
    pub fn image(&mut self, ctx: &Context) {
        Window::new(&self.title)
            .open(&mut self.open)
            .show(ctx, |ui| {
                ui.image(&self.path);
            });
    }
}