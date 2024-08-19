pub mod trace;
pub mod image;
pub mod layer_view;

pub trait Panel {
    fn show(&mut self, ctx: &eframe::egui::Context);
}