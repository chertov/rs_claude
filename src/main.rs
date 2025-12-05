#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui;

#[derive(Default, serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct CounterApp {
    counter: i32,
}

impl CounterApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }
        Default::default()
    }
}

impl eframe::App for CounterApp {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(50.0);

                ui.heading("Counter Demo");
                ui.add_space(20.0);

                ui.label(
                    egui::RichText::new(format!("{}", self.counter))
                        .size(72.0)
                        .strong()
                );

                ui.add_space(20.0);

                ui.horizontal(|ui| {
                    ui.add_space(ui.available_width() / 2.0 - 100.0);

                    if ui.add_sized([60.0, 40.0], egui::Button::new("-")).clicked() {
                        self.counter -= 1;
                    }

                    ui.add_space(10.0);

                    if ui.add_sized([60.0, 40.0], egui::Button::new("Reset")).clicked() {
                        self.counter = 0;
                    }

                    ui.add_space(10.0);

                    if ui.add_sized([60.0, 40.0], egui::Button::new("+")).clicked() {
                        self.counter += 1;
                    }
                });

                ui.add_space(30.0);
                ui.separator();
                ui.add_space(10.0);
                ui.label("Built with Rust + egui + WASM");
            });
        });
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 300.0])
            .with_min_inner_size([300.0, 220.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Counter Demo",
        native_options,
        Box::new(|cc| Ok(Box::new(CounterApp::new(cc)))),
    )
}

#[cfg(target_arch = "wasm32")]
fn main() {
    use eframe::wasm_bindgen::JsCast as _;

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        let document = web_sys::window()
            .expect("No window")
            .document()
            .expect("No document");

        let canvas = document
            .get_element_by_id("the_canvas_id")
            .expect("Failed to find the_canvas_id")
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .expect("the_canvas_id was not a HtmlCanvasElement");

        let start_result = eframe::WebRunner::new()
            .start(
                canvas,
                web_options,
                Box::new(|cc| Ok(Box::new(CounterApp::new(cc)))),
            )
            .await;

        let loading_text = document.get_element_by_id("loading_text");
        if let Some(loading_text) = loading_text {
            match start_result {
                Ok(_) => {
                    loading_text.remove();
                }
                Err(e) => {
                    loading_text.set_inner_html(
                        &format!("<p>The app has crashed: {:?}</p>", e),
                    );
                }
            }
        }
    });
}
