pub mod example;

#[cfg(target_arch = "wasm32")]
use crate::log;
use egui::{load, Sense};

pub trait Card {
    fn draw(&mut self, ui: &mut egui::Ui) -> egui::Response;
}

pub struct Backside {
    pub(crate) printed: bool,
    bytes: Option<load::Bytes>,
}

impl Backside {
    pub fn new() -> Self {
        Self {
            printed: false,
            bytes: None,
        }
    }
}

impl Card for Backside {
    fn draw(&mut self, ui: &mut egui::Ui) -> egui::Response {
        let img = egui::Image::new(
            format!("https://placehold.co/100x144/png?text=Hello World!").to_string(),
        )
        .show_loading_spinner(true)
        .sense(Sense::click_and_drag());
        ui.add(img)
    }
}
