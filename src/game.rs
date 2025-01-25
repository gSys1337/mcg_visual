use eframe::emath::Align;
use eframe::Frame;
use egui::{Context, Direction};
use crate::log;

pub struct App {
    text: String,
}

impl eframe::App for App {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {

        egui::CentralPanel::default().show(ctx, |ui| {
            let layout = egui::Layout::from_main_dir_and_cross_align(Direction::TopDown, Align::Center);
            ui.with_layout(layout, |ui| {
                let start: egui::Button = egui::Button::new("Start Game");
                ui.add(start);
                ui.text_edit_singleline(&mut self.text);
                if ui.button("Log to console").clicked() {
                    log(self.text.as_str());
                }
                egui::Area::new(egui::Id::new("myArea"))
            });
        });
    }
}

impl App {
    pub fn new() -> Self {
        crate::utils::set_panic_hook();
        log("New App created.");
        Self { text: String::new() }
    }
}
