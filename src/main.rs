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
fn log_to_page(msg: &str, log_type: &str) {
    use eframe::wasm_bindgen::JsCast;
    if let Some(window) = web_sys::window() {
        if let Some(document) = window.document() {
            if let Some(log_div) = document.get_element_by_id("log") {
                if let Ok(line) = document.create_element("div") {
                    line.set_class_name(log_type);
                    line.set_text_content(Some(msg));
                    let _ = log_div.append_child(&line);
                }
            }
        }
    }
}

#[cfg(target_arch = "wasm32")]
fn set_status(msg: &str) {
    if let Some(window) = web_sys::window() {
        if let Some(document) = window.document() {
            if let Some(status) = document.get_element_by_id("status") {
                status.set_text_content(Some(msg));
            }
        }
    }
}

#[cfg(target_arch = "wasm32")]
fn main() {
    use eframe::wasm_bindgen::JsCast as _;

    log_to_page("WASM module loaded", "ok");
    set_status("WASM loaded, initializing...");

    let web_options = eframe::WebOptions::default();
    log_to_page("WebOptions created", "ok");

    wasm_bindgen_futures::spawn_local(async {
        log_to_page("Async task started", "ok");

        let window = match web_sys::window() {
            Some(w) => {
                log_to_page("Got window object", "ok");
                w
            }
            None => {
                log_to_page("Failed to get window!", "err");
                set_status("Error: No window object");
                return;
            }
        };

        let document = match window.document() {
            Some(d) => {
                log_to_page("Got document object", "ok");
                d
            }
            None => {
                log_to_page("Failed to get document!", "err");
                set_status("Error: No document object");
                return;
            }
        };

        let canvas = match document.get_element_by_id("the_canvas_id") {
            Some(c) => {
                log_to_page("Found canvas element", "ok");
                match c.dyn_into::<web_sys::HtmlCanvasElement>() {
                    Ok(canvas) => {
                        log_to_page("Canvas cast successful", "ok");
                        canvas
                    }
                    Err(e) => {
                        log_to_page(&format!("Canvas cast failed: {:?}", e), "err");
                        set_status("Error: Canvas cast failed");
                        return;
                    }
                }
            }
            None => {
                log_to_page("Canvas element not found!", "err");
                set_status("Error: Canvas not found");
                return;
            }
        };

        log_to_page("Starting eframe WebRunner...", "info");
        set_status("Starting egui...");

        let start_result = eframe::WebRunner::new()
            .start(
                canvas,
                web_options,
                Box::new(|cc| {
                    log_to_page("App creation callback called", "ok");
                    Ok(Box::new(CounterApp::new(cc)))
                }),
            )
            .await;

        match start_result {
            Ok(_) => {
                log_to_page("eframe started successfully!", "ok");
                if let Some(loading_text) = document.get_element_by_id("loading_text") {
                    loading_text.remove();
                }
            }
            Err(e) => {
                let err_msg = format!("eframe start failed: {:?}", e);
                log_to_page(&err_msg, "err");
                set_status("Error starting app!");
            }
        }
    });
}
