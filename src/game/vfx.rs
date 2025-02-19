use crate::game;
use egui::containers::frame;
use std::ops::Add;

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
            (100.0 + self.inner_margin as f32) * max_cards as f32 - self.inner_margin as f32,
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
            (self.size.x - 100.0) * (idx as f32) / (cards - 1) as f32
        };
        egui::Vec2::new(x, 0.0)
    }
}

impl Default for HandLayout {
    fn default() -> Self {
        let mut x = Self {
            cards: vec![],
            pos: egui::Pos2::new(69.0, 420.0),
            size: Default::default(),
            inner_margin: 5,
            max_cards: 0,
        };
        x.max_cards(5);
        x
    }
}

impl egui::Widget for &mut HandLayout {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        egui::Area::new(ui.next_auto_id())
            .current_pos(self.pos)
            .sense(egui::Sense::empty())
            .show(ui.ctx(), |ui| {
                let pointer = ui.input(|state| state.pointer.clone());
                let mut selected = None;
                if pointer.latest_pos().is_some()
                    && ui.max_rect().contains(pointer.latest_pos().unwrap())
                {
                    let left = ui.max_rect().left();
                    let right = ui.max_rect().right();
                    let selector = self.cards.len() as f32
                        * (pointer
                            .latest_pos()
                            .unwrap_or_else(|| egui::pos2(left, 0.0))
                            .x
                            - left)
                        / (right - left);
                    selected = Some(selector as usize);
                }
                frame::Frame::new()
                    .inner_margin(egui::Margin::same(self.inner_margin))
                    .outer_margin(egui::Margin::same(5))
                    .stroke(egui::Stroke::new(2.0, egui::Color32::DEBUG_COLOR))
                    .fill(egui::Color32::DARK_GREEN)
                    .corner_radius(egui::CornerRadius::same(5))
                    .show(ui, |ui| {
                        let next_pos = ui.next_widget_position();
                        ui.allocate_new_ui(
                            egui::UiBuilder::new()
                                .max_rect(egui::Rect::from_min_size(next_pos, self.size))
                                .layer_id(egui::LayerId::background()),
                            |ui| {
                                ui.set_max_size(self.size);
                                ui.set_min_size(self.size);
                                for (idx, card) in self.cards.iter().enumerate() {
                                    let card_pos = next_pos.add(self.card_pos(idx));
                                    if selected.is_some() && idx == selected.unwrap() {
                                        continue;
                                    }
                                    egui::Area::new(ui.next_auto_id())
                                        .order(egui::Order::Foreground)
                                        .sense(egui::Sense::all())
                                        .current_pos(card_pos)
                                        .show(ui.ctx(), |ui| {
                                            ui.add(&**card);
                                        });
                                }
                                if selected.is_some() {
                                    self.cards.get(selected.unwrap()).map(|card| {
                                        let card_pos = next_pos
                                            .add(self.card_pos(selected.unwrap()))
                                            .add(egui::vec2(0.0, -10.0));
                                        egui::Area::new(ui.next_auto_id())
                                            .order(egui::Order::Foreground)
                                            .sense(egui::Sense::all())
                                            .current_pos(card_pos)
                                            .show(ui.ctx(), |ui| {
                                                egui::Frame::new()
                                                    .stroke(egui::Stroke::new(
                                                        2.0,
                                                        egui::Color32::RED,
                                                    ))
                                                    .corner_radius(egui::CornerRadius::same(2))
                                                    .show(ui, |ui| {
                                                        ui.allocate_new_ui(
                                                            egui::UiBuilder::new(),
                                                            |ui| {
                                                                ui.add(card.as_ref());
                                                            },
                                                        );
                                                    });
                                            });
                                    });
                                }
                            },
                        )
                        .response
                    })
                    .inner
            })
            .inner
    }
}

pub struct Stack {
    pub cards: Vec<Box<dyn game::Card>>,
    pub pos: egui::Pos2,
    size: egui::Vec2,
    pub inner_margin: i8,
    max_cards: usize,
}
impl Stack {
    pub fn add_card(&mut self, card: Box<dyn game::Card>) {
        self.cards.push(card);
    }
    fn card_pos(&self, idx: usize) -> egui::Vec2 {
        let x = if idx <= self.max_cards {
            idx as f32
        } else {
            self.max_cards as f32
        };
        egui::Vec2::new(x, -x + self.inner_margin as f32)
    }
    pub fn max_cards(&mut self, max_cards: usize) {
        let size = egui::Vec2::new(100.0 + max_cards as f32, 144.0 + max_cards as f32);
        self.size = size;
        self.max_cards = max_cards;
    }
}

impl Default for Stack {
    fn default() -> Self {
        let mut x = Self {
            cards: vec![],
            pos: egui::pos2(314.15, 217.18),
            size: Default::default(),
            inner_margin: 5,
            max_cards: 0,
        };
        x.max_cards(5);
        x
    }
}

impl egui::Widget for &mut Stack {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        egui::Area::new(ui.next_auto_id())
            .current_pos(self.pos)
            .sense(egui::Sense::empty())
            .show(ui.ctx(), |ui| {
                frame::Frame::new()
                    .inner_margin(egui::Margin::same(self.inner_margin))
                    .outer_margin(egui::Margin::same(5))
                    .stroke(egui::Stroke::new(2.0, egui::Color32::DEBUG_COLOR))
                    .fill(egui::Color32::DARK_GREEN)
                    .corner_radius(egui::CornerRadius::same(5))
                    .show(ui, |ui| {
                        let next_pos = ui.next_widget_position();
                        ui.allocate_new_ui(
                            egui::UiBuilder::new()
                                .max_rect(egui::Rect::from_min_size(next_pos, self.size))
                                .layer_id(egui::LayerId::background()),
                            |ui| {
                                ui.set_max_size(self.size);
                                ui.set_min_size(self.size);
                                for (idx, card) in self.cards.iter().enumerate() {
                                    let card_pos = next_pos.add(self.card_pos(idx));
                                    egui::Area::new(ui.next_auto_id())
                                        .order(egui::Order::Foreground)
                                        .sense(egui::Sense::all())
                                        .current_pos(card_pos)
                                        .show(ui.ctx(), |ui| {
                                            ui.add(&**card);
                                        });
                                }
                            },
                        )
                        .response
                    })
                    .inner
            })
            .inner
    }
}
