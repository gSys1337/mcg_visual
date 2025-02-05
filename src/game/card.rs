pub mod example;

#[cfg(target_arch = "wasm32")]
use crate::log;
use egui::{load, Sense};
use std::ops::Add;

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

impl Card for example::ConventionalCard {
    fn draw(&mut self, ui: &mut egui::Ui) -> egui::Response {
        let path = format!(
            "http://127.0.0.1:8080/media/img_cards/{}_{}.png",
            self.rank as usize + 1,
            self.suit.to_string().to_lowercase()
        );
        #[cfg(target_arch = "wasm32")]
        log(path.as_str());
        let img = egui::Image::new(path)
            .show_loading_spinner(true)
            .fit_to_original_size(1.0)
            .sense(Sense::click_and_drag());
        let r = ui.add(img);
        if r.is_pointer_button_down_on() {
            let new = self.pos.add(r.drag_delta());
            self.pos = new;
        }
        r
    }
}
