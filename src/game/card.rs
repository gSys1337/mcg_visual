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
