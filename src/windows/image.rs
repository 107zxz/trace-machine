use eframe::egui::*;

pub struct Image {
    path: String,
    title: String
}

impl Image {
    pub fn new(path: String, title: String) -> Self {
        Image {
            path,
            title
        }
    }
    pub fn image(&mut self, ctx: &Context) {
        Window::new(&self.title).show(ctx, |ui| {
            ui.image(format!("file://{}", &self.path));
        });
    }
}