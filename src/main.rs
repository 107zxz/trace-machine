mod windows;

use eframe::egui;

fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([640.0, 480.0]),
        ..Default::default()
    };
    eframe::run_native(
        "My egui App",
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
    refs: Vec<windows::image::Image>,
    layer_view: windows::layer_view::LayerView
}

impl Default for MyApp {
    fn default() -> Self {
        // let refs = fs::read_dir("assets/").unwrap().map(|entry| {
        //     windows::image::Image::new(entry.unwrap().path().to_str().unwrap().into())
        // }).collect();

        // let refs = vec![windows::image::Image::new("assets/rammy.png".into())];
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
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("My egui Application");
        });

        // Paint window
        self.traces.iter_mut().for_each(|f| f.trace(ctx));
        self.traces.retain(|t| t.open);

        self.refs.iter_mut().for_each(|f| f.image(ctx));
        self.layer_view.show(ctx, &mut self.traces);

        if !ctx.input(|i| i.raw.dropped_files.is_empty()) {
            for file in ctx.input(|i| i.raw.dropped_files.clone()) {
                self.refs.push(windows::image::Image::new(
                    file.path.unwrap().to_str().unwrap().to_owned(),
                    format!("Ref {}", self.refs.len())
                ));
            }
        }
    }
}

impl MyApp {
    fn handle_drops() {

    }
}