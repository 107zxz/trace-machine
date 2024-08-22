mod windows;

use eframe::egui;
use eframe::egui::{Key, ViewportCommand};

fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([640.0, 480.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Tracemaker",
        options,
        Box::new(|cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);

            Ok(Box::<MyApp>::default())
        }),
    )
}

struct MyApp {
    traces: Vec<windows::trace::Trace>,
    refs: Vec<windows::image::ImageRef>,
    layer_view: windows::layer_view::LayerView
}

impl Default for MyApp {
    fn default() -> Self {
        let refs = vec![];

        Self {
            traces: Vec::new(),
            refs,
            layer_view: windows::layer_view::LayerView::new()
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.traces.iter_mut().for_each(|f| f.trace(ctx));
        self.traces.retain(|t| t.open);

        self.refs.iter_mut().for_each(|f| f.image(ctx));
        self.refs.retain(|r| r.open);

        self.layer_view.show(ctx, &mut self.traces, &mut self.refs);

        // Drag and drop
        if !ctx.input(|i| i.raw.dropped_files.is_empty()) {
            for file in ctx.input(|i| i.raw.dropped_files.clone()) {
                self.refs.push(windows::image::ImageRef::new(
                    format!("file://{}", file.path.unwrap().to_str().unwrap().to_owned()),
                    format!("Ref {}", self.refs.len())
                ));
            }
        }

        // Fullscreen
        if ctx.input(|i| i.key_pressed(Key::F11)) {
            ctx.send_viewport_cmd(ViewportCommand::Fullscreen(!ctx.input(|i| i.viewport().fullscreen.unwrap())));
        }

        // Escape
        if ctx.input(|i| i.key_pressed(Key::Escape)) {
            ctx.send_viewport_cmd(ViewportCommand::Close);
        }
    }
}