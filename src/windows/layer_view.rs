use copypasta::{ClipboardContext, ClipboardProvider};
use eframe::egui::*;
use eframe::epaint::Hsva;
use crate::windows::image::ImageRef;
use crate::windows::trace::Trace;

const COLOUR_OFFSET: f32 = 0.15;

pub struct LayerView {
    trace_index: f32,
    ref_index: u32,
    clipboard_context: ClipboardContext
}

impl LayerView {
    pub fn new() -> Self {
        Self {
            trace_index: 0.0,
            ref_index: 0,
            clipboard_context: ClipboardContext::new().unwrap()
        }
    }

    pub fn show(&mut self, ctx: &Context, traces: &mut Vec<Trace>, refs: &mut Vec<ImageRef>) {
        Window::new("Layers")
            .fixed_pos([0f32, 0f32])
            .collapsible(false)
            .resizable(false)
            .show(ctx, |ui| {
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

                if ui.button("Add URL").clicked() {
                    let url = self.clipboard_context.get_contents().unwrap();
                    refs.push(ImageRef::new(url, format!("Ref: {}", self.ref_index)));
                    self.ref_index += 1;
                }
            });

            // List of existing layers
            ui.vertical(|ui| {
                traces.iter_mut().for_each(|trace| {
                    ui.horizontal(|ui| {
                        ui.label(&trace.window_title);
                        if ui.button("D").clicked() {
                            trace.enabled = !trace.enabled;
                        }
                        if ui.button("T").clicked() {
                            ctx.move_to_top(trace.layer_id);
                        }

                        ui.add(&mut trace.stroke);
                    });
                });
            });
        });
    }
}