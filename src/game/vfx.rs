use egui::containers::frame;
use std::ops::Add;

use crate::game;

pub struct HandLayout {
    pub cards: Vec<Box<dyn game::Card>>,
    pub pos: egui::Pos2,
    size: egui::Vec2,
    pub inner_margin: i8,
    max_cards: usize,
}

impl HandLayout {
    pub fn add_card(&mut self, card: Box<dyn game::Card>) {
        self.cards.push(card);
    }
    pub fn max_cards(&mut self, max_cards: usize) {
        let size = egui::Vec2::new(
            (100.0 + self.inner_margin as f32) * max_cards as f32 - max_cards as f32,
            144.0,
        );
        self.size = size;
        self.max_cards = max_cards;
    }
    fn card_pos(&self, idx: usize) -> egui::Vec2 {
        let cards = self.cards.len();
        let x = if cards <= self.max_cards {
            (100.0 + self.inner_margin as f32) * (idx as f32)
        } else {
            (self.size.x - 100.0) * (idx as f32) / (cards-1) as f32
        };
        egui::Vec2::new(x, 0.0)
    }
}

impl Default for HandLayout {
    fn default() -> Self {
        let cards = vec![];
        let inner_margin = 5;
        let max_cards = 5;
        let pos = egui::Pos2::new(69.0, 420.0);
        let size = egui::Vec2::new(
            (100.0 + inner_margin as f32) * max_cards as f32 - max_cards as f32,
            144.0,
        );
        Self {
            cards,
            pos,
            size,
            inner_margin,
            max_cards,
        }
    }
}

impl egui::Widget for &mut HandLayout {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        ui.set_max_size(self.size);
        ui.set_min_size(self.size);
        let inner_margin = 5.0;
        let frame = frame::Frame::new()
            .inner_margin(egui::Margin::same(inner_margin as i8))
            .outer_margin(egui::Margin::same(5))
            .stroke(egui::Stroke::new(2.0, egui::Color32::DEBUG_COLOR))
            .fill(egui::Color32::DARK_GREEN)
            .corner_radius(egui::CornerRadius::same(5));
        frame
            .show(ui, |ui| {
                let next_pos = ui.next_widget_position();
                ui.allocate_new_ui(
                    egui::UiBuilder::new().max_rect(egui::Rect::from_min_size(next_pos, self.size)),
                    |ui| {
                        ui.set_max_size(self.size);
                        ui.set_min_size(self.size);
                        for (idx, card) in self.cards.iter().enumerate() {
                            egui::Area::new(format!("{}", idx + 100).into())
                                .interactable(false)
                                .sense(egui::Sense::all())
                                .current_pos(ui.next_widget_position().add(self.card_pos(idx)))
                                .show(ui.ctx(), |ui| {
                                    ui.add(&**card);
                                });
                        }
                    },
                )
                .response
            })
            .response
    }
}
