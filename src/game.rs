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
            egui::Area::new(egui::Id::new("myArea"))
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
