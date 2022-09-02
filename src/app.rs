#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
use wasm_bindgen::prelude::*;
#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
use wasm_bindgen::JsCast;
#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
use web_sys::{ErrorEvent, MessageEvent, WebSocket};

#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

use egui_extras::{Size, StripBuilder};
use qt_simple_crypt::{simple_crypt::SimpleCrypt, *};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    // Example stuff:
    label: String,

    // this how you opt-out of serialization of a member
    #[serde(skip)]
    value: f32,
    cypher_key: String,
    plain_text: String,
    cypher_text: String,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            value: 2.7,
            cypher_key: "01020304".to_owned(),
            plain_text: "plain text here".to_owned(),
            cypher_text: "cypher text here".to_owned(),
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customized the look at feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let Self {
            label,
            value,
            cypher_key: _,
            plain_text: _,
            cypher_text: _,
        } = self;

        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        frame.quit();
                    }
                });
            });
        });

        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.heading("Side Panel ___hello");

            ui.horizontal(|ui| {
                ui.label("Write something: 123123 ");
                ui.text_edit_singleline(label);
            });

            ui.add(egui::Slider::new(value, 0.0..=10.0).text("value"));
            if ui.button("Increment").clicked() {
                *value += 1.0;
            }

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;
                    ui.label("powered by ");
                    ui.hyperlink_to("egui", "https://github.com/emilk/egui");
                    ui.label(" and ");
                    ui.hyperlink_to("eframe", "https://github.com/emilk/egui/tree/master/eframe");
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's

            ui.heading("eframe template");
            ui.hyperlink("https://github.com/emilk/eframe_template");
            ui.add(egui::github_link_file!(
                "https://github.com/emilk/eframe_template/blob/master/",
                "Source code."
            ));

            ui.text_edit_singleline(&mut self.cypher_key);

            StripBuilder::new(ui)
                .size(Size::exact(100.0))
                .size(Size::remainder())
                .vertical(|mut strip| {
                    strip.cell(|ui| {
                        ui.horizontal(|ui| {
                            if ui.button("http").clicked() {
                                println!("http request");
                                start_websocket();
                            }
                            if ui.button("Encrypt").clicked() {
                                let mut crypto = SimpleCrypt::new(
                                    u64::from_str_radix(&self.cypher_key, 16).unwrap(),
                                );
                                crypto.set_compression_mode(
                                    simple_crypt::CompressionMode::CompressionNever,
                                );
                                crypto.set_integrity_protection_mode(
                                    simple_crypt::IntegrityProtectionMode::ProtectionNone,
                                );
                                let cypher_text =
                                    crypto.encrypt_to_string(&self.plain_text[..]).unwrap();
                                println!("encrypted: {}", &cypher_text);
                                self.cypher_text = String::from(cypher_text);
                            }

                            if ui.button("Decrypt").clicked() {
                                let mut crypto = SimpleCrypt::new(
                                    u64::from_str_radix(&self.cypher_key, 16).unwrap(),
                                );
                                crypto.set_compression_mode(
                                    simple_crypt::CompressionMode::CompressionNever,
                                );
                                crypto.set_integrity_protection_mode(
                                    simple_crypt::IntegrityProtectionMode::ProtectionNone,
                                );
                                let plain_text =
                                    crypto.decrypt_to_string(&self.cypher_text[..]).unwrap();
                                println!("decrypted: {}", &plain_text);
                                self.plain_text = String::from(plain_text);
                            }
                            egui::warn_if_debug_build(ui);
                        });
                    });
                    strip.strip(|builder| {
                        builder.sizes(Size::remainder(), 2).horizontal(|mut strip| {
                            strip.cell(|ui| {
                                // left side
                                ui.push_id("plain_text", |ui| {
                                    egui::ScrollArea::vertical().show(ui, |ui| {
                                        ui.text_edit_multiline(&mut self.plain_text);
                                    });
                                });
                            });
                            strip.cell(|ui| {
                                // right side
                                ui.push_id("cypher_text", |ui| {
                                    egui::ScrollArea::vertical().show(ui, |ui| {
                                        ui.text_edit_multiline(&mut self.cypher_text);
                                    });
                                });
                            });
                        });
                    });
                });
        });

        if false {
            egui::Window::new("Window").show(ctx, |ui| {
                ui.label("Windows can be moved by dragging them.");
                ui.label("They are automatically sized based on contents.");
                ui.label("You can turn on resizing and scrolling if you like.");
                ui.label("You would normally chose either panels OR windows.");
            });
        }
    }
}

#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
pub fn start_websocket() {
    println!("not implemented yet");
}

// NOTE If uncomment below line, it will be executed when wasm module is instanciated.
// See https://rustwasm.github.io/wasm-bindgen/reference/attributes/on-rust-exports/start.html
// #[wasm_bindgen(start)]
#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
pub fn start_websocket() -> Result<(), JsValue> {
    // Connect to an echo server
    let ws = WebSocket::new("wss://echo.websocket.events")?;
    // For small binary messages, like CBOR, Arraybuffer is more efficient than Blob handling
    ws.set_binary_type(web_sys::BinaryType::Arraybuffer);
    // create callback
    let cloned_ws = ws.clone();
    let onmessage_callback = Closure::<dyn FnMut(_)>::new(move |e: MessageEvent| {
        // Handle difference Text/Binary,...
        if let Ok(abuf) = e.data().dyn_into::<js_sys::ArrayBuffer>() {
            console_log!("message event, received arraybuffer: {:?}", abuf);
            let array = js_sys::Uint8Array::new(&abuf);
            let len = array.byte_length() as usize;
            console_log!("Arraybuffer received {}bytes: {:?}", len, array.to_vec());
            // here you can for example use Serde Deserialize decode the message
            // for demo purposes we switch back to Blob-type and send off another binary message
            cloned_ws.set_binary_type(web_sys::BinaryType::Blob);
            match cloned_ws.send_with_u8_array(&vec![5, 6, 7, 8]) {
                Ok(_) => console_log!("binary message successfully sent"),
                Err(err) => console_log!("error sending message: {:?}", err),
            }
        } else if let Ok(blob) = e.data().dyn_into::<web_sys::Blob>() {
            console_log!("message event, received blob: {:?}", blob);
            // better alternative to juggling with FileReader is to use https://crates.io/crates/gloo-file
            let fr = web_sys::FileReader::new().unwrap();
            let fr_c = fr.clone();
            // create onLoadEnd callback
            let onloadend_cb = Closure::<dyn FnMut(_)>::new(move |_e: web_sys::ProgressEvent| {
                let array = js_sys::Uint8Array::new(&fr_c.result().unwrap());
                let len = array.byte_length() as usize;
                console_log!("Blob received {}bytes: {:?}", len, array.to_vec());
                // here you can for example use the received image/png data
            });
            fr.set_onloadend(Some(onloadend_cb.as_ref().unchecked_ref()));
            fr.read_as_array_buffer(&blob).expect("blob not readable");
            onloadend_cb.forget();
        } else if let Ok(txt) = e.data().dyn_into::<js_sys::JsString>() {
            console_log!("message event, received Text: {:?}", txt);
        } else {
            console_log!("message event, received Unknown: {:?}", e.data());
        }
    });
    // set message event handler on WebSocket
    ws.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
    // forget the callback to keep it alive
    onmessage_callback.forget();

    let onerror_callback = Closure::<dyn FnMut(_)>::new(move |e: ErrorEvent| {
        console_log!("error event: {:?}", e);
    });
    ws.set_onerror(Some(onerror_callback.as_ref().unchecked_ref()));
    onerror_callback.forget();

    let cloned_ws = ws.clone();
    let onopen_callback = Closure::<dyn FnMut()>::new(move || {
        console_log!("socket opened");
        match cloned_ws.send_with_str("ping") {
            Ok(_) => console_log!("message successfully sent"),
            Err(err) => console_log!("error sending message: {:?}", err),
        }
        // send off binary message
        match cloned_ws.send_with_u8_array(&vec![0, 1, 2, 3]) {
            Ok(_) => console_log!("binary message successfully sent"),
            Err(err) => console_log!("error sending message: {:?}", err),
        }
    });
    ws.set_onopen(Some(onopen_callback.as_ref().unchecked_ref()));
    onopen_callback.forget();

    Ok(())
}
