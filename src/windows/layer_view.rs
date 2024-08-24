use copypasta::{ClipboardContext, ClipboardProvider};
use eframe::egui::*;
use eframe::epaint::Hsva;
use log::{log, Level};
use crate::windows::add_ref_url::AddRefUrlWin;
use crate::windows::image::ImageRef;
use crate::windows::trace::Trace;

const COLOUR_OFFSET: f32 = 0.15;

pub struct LayerView {
    trace_index: f32,
    ref_index: u32,
    #[cfg(not(target_arch = "wasm32"))]
    clipboard_context: ClipboardContext,
    add_url_win: AddRefUrlWin
}

impl LayerView {
    pub fn new() -> Self {
        Self {
            trace_index: 0.0,
            ref_index: 0,
            #[cfg(not(target_arch = "wasm32"))]
            clipboard_context: ClipboardContext::new().unwrap(),
            add_url_win: AddRefUrlWin {
                active: false,
                url: String::new()
            }
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

                #[cfg(not(target_arch = "wasm32"))]
                if ui.button("Add URL").clicked() {
                    let url = self.clipboard_context.get_contents().unwrap();

                    log!(Level::Error, "Image url: {}", url);

                    refs.push(ImageRef::new(url, format!("Ref: {}", self.ref_index)));
                    self.ref_index += 1;
                }

                #[cfg(target_arch = "wasm32")]
                if ui.button("Add URL").clicked() {
                    self.add_url_win.active = true;
                }

                if self.add_url_win.active {
                    Window::new("Add reference from URL").show(ctx, |ui| {
                        ui.horizontal_centered(|ui| {
                            ui.text_edit_singleline(&mut self.add_url_win.url);
                            if ui.button("Add").clicked() {
                                log!(Level::Error, "Image url: {}", self.add_url_win.url);

                                refs.push(ImageRef::new(self.add_url_win.url.clone(), format!("Ref: {}", self.ref_index)));
                                self.ref_index += 1;

                                self.add_url_win.active = false;
                            }
                        });
                    });
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