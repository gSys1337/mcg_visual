use egui::Sense;
// #[cfg(target_arch = "wasm32")]
#[allow(unused_imports)]
use crate::log;

pub trait Card {
    // TODO replace img_path(...) with img(...)
    fn img_path(&self) -> String;
    fn pos(&self) -> egui::Pos2;
    fn set_pos(&mut self, pos: egui::Pos2);
    fn translate(&mut self, amt: egui::Vec2);
}

impl dyn Card {
    pub fn draw(
        &self,
        ui: &mut egui::Ui,
        pos: Option<egui::Pos2>,
        sense: Option<Sense>,
        order: Option<egui::Order>,
    ) -> egui::InnerResponse<egui::Response> {
        let mut area = egui::Area::new(ui.next_auto_id());
        match pos {
            Some(pos) => area = area.current_pos(pos),
            None => area = area.current_pos(self.pos()),
        }
        sense.iter().for_each(|sense| {
            area = area.sense(*sense);
        });
        order.iter().for_each(|order| {
            area = area.order(*order);
        });
        // important to use ``&*card`` because rust gets it somehow wrong ¯\_(ツ)_/¯
        // i assume this is an "issue" with deref coercion because
        // Rust can include `*`s at compile-time but no `&`
        area.show(ui.ctx(), |ui| ui.add(&*self))
    }
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
