#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;
use qt_simple_crypt::{simple_crypt::SimpleCrypt, *};

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    );
}

struct MyApp {
    name: String,
    age: u32,
    plain_text: String,
    cypher_text: String,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            name: "Arthur".to_owned(),
            age: 42,
            plain_text: "plain text here".to_owned(),
            cypher_text: "cypher text here".to_owned(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("My egui Application");
            ui.horizontal(|ui| {
                ui.label("Your name: ");
                ui.text_edit_singleline(&mut self.name);
            });
            ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));
            if ui.button("Click each year").clicked() {
                self.age += 1;
            }
            ui.label(format!("Hello '{}', age {}", self.name, self.age));
            ui.text_edit_multiline(&mut self.plain_text);
            ui.text_edit_multiline(&mut self.cypher_text);
            if ui.button("Encrypt").clicked() {
                let mut crypto = SimpleCrypt::new(0x1234);
                crypto.set_compression_mode(simple_crypt::CompressionMode::CompressionNever);
                crypto.set_integrity_protection_mode(
                    simple_crypt::IntegrityProtectionMode::ProtectionNone,
                );
                let cypher_text = crypto.encrypt_to_string(&self.plain_text[..]).unwrap();
                println!("encrypted: {}", &cypher_text);
                self.cypher_text = String::from(cypher_text);
            }

            if ui.button("Decrypt").clicked() {
                let mut crypto = SimpleCrypt::new(0x1234);
                crypto.set_compression_mode(simple_crypt::CompressionMode::CompressionNever);
                crypto.set_integrity_protection_mode(
                    simple_crypt::IntegrityProtectionMode::ProtectionNone,
                );
                let plain_text = crypto.decrypt_to_string(&self.cypher_text[..]).unwrap();
                println!("decrypted: {}", &plain_text);
                self.plain_text = String::from(plain_text);
            }
        });
    }
}
