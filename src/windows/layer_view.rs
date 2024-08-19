use eframe::egui::*;
use eframe::epaint::Hsva;
use crate::windows::trace::Trace;

const COLOUR_OFFSET: f32 = 0.15;

pub struct LayerView {
    trace_index: f32,
}

impl LayerView {
    pub fn new() -> Self {
        Self {
            trace_index: 0.0
        }
    }

    pub fn show(&mut self, ctx: &Context, traces: &mut Vec<Trace>) {
        Window::new("Layers").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("Add Trace").clicked() {
                    traces.push(
                        Trace::new(format!("Trace {}", self.trace_index),
                                   Color32::from(
                                       Hsva::new(
                                           (self.trace_index * COLOUR_OFFSET) % 1.0,
                                           1.0,
                                           1.0,
                                           1.0
                                       )
                                   )
                        )
                    );
                    self.trace_index += 1.0;
                }
            });

            ui.vertical(|ui| {
                traces.iter_mut().for_each(|trace| {
                    if ui.button(&trace.title).clicked() {
                        trace.enabled = !trace.enabled;
                    }
                });
            });
        });
    }
}