pub mod example;

#[cfg(target_arch = "wasm32")]
#[allow(unused_imports)]
use crate::log;
use egui::Sense;

pub trait Card {
    fn img_path(&self) -> String;
    fn pos(&self) -> egui::Pos2;
    fn translate(&mut self, amt: egui::Vec2);
}

impl egui::Widget for &dyn Card {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let path = self.img_path();
        let img = egui::Image::new(path)
            .show_loading_spinner(true)
            .fit_to_original_size(1.0)
            .sense(Sense::click_and_drag());
        ui.add(img)
    }
}

pub struct Backside {}

impl Backside {
    pub fn new() -> Self {
        Self {}
    }
}

impl Card for Backside {
    fn img_path(&self) -> String {
        "https://placehold.co/100x144/png?text=Hello World!".to_string()
    }

    fn pos(&self) -> egui::Pos2 {
        egui::pos2(500.0, 500.0)
    }

    fn translate(&mut self, _amt: egui::Vec2) {}
}
