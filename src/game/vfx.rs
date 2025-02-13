use egui::containers::frame;
use crate::game;

pub struct HandLayout {
    pub cards: Vec<Box<dyn game::Card>>,
    pub pos: egui::Pos2,
    pub size: egui::Vec2,
}

impl HandLayout {
    pub fn add_card(&mut self, card: Box<dyn game::Card>) {
        self.cards.push(card);
    }
}

impl Default for HandLayout {
    fn default() -> Self {
        let cards = vec![];
        let pos = egui::Pos2::new(69.0, 420.0);
        let size = egui::Vec2::new(550.0, 144.0);
        Self { cards, pos, size }
    }
}

impl egui::Widget for &mut HandLayout {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let frame = frame::Frame::new()
            .inner_margin(egui::Margin::same(5))
            .outer_margin(egui::Margin::same(5))
            .stroke(egui::Stroke::new(2.0, egui::Color32::DEBUG_COLOR))
            .fill(egui::Color32::DARK_GREEN)
            .corner_radius(egui::CornerRadius::same(5));
        frame
            .show(ui, |ui| {
                ui.allocate_new_ui(
                    egui::UiBuilder::new()
                        //.max_rect(egui::Rect::from_min_size(self.pos, self.size))
                        .layout(egui::Layout::left_to_right(egui::Align::Center)),
                    |ui| {
                        // ui.set_max_size(self.size);
                        // ui.set_min_size(self.size);
                        ui.set_min_width(self.size.x);
                        ui.set_min_height(self.size.y);
                        for card in &self.cards {
                            ui.add(&**card);
                        }
                    },
                )
                .response
            })
            .response
    }
}
