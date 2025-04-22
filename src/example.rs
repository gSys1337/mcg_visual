use crate::game;
use crate::game::{card, Field};
// #[cfg(target_arch = "wasm32")]
#[allow(unused_imports)]
use crate::log;
use egui;
use egui::frame;
use game::card::Drawable;
use rand::Rng;
use std::fmt;
use std::ops::Add;

#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
pub struct ConventionalCard {
    pub suit: Suit,
    pub rank: Rank,
    pub pos: egui::Pos2,
}

impl ConventionalCard {
    pub fn _iter() -> ConventionalCardIter {
        Default::default()
    }

    pub fn new_random() -> Self {
        let mut rng = rand::thread_rng();
        let rank: Rank = rng.gen_range(0..Rank::len()).into();
        let suit: Suit = rng.gen_range(0..Suit::len()).into();
        let x = rng.gen_range(0..1000) as f32;
        let y = rng.gen_range(0..1000) as f32;
        let pos = egui::Pos2::new(x, y);
        ConventionalCard { suit, rank, pos }
    }
}

impl card::Card for ConventionalCard {
    fn img_path(&self) -> String {
        format!(
            "http://127.0.0.1:8080/media/img_cards/{}_{}.png",
            self.rank as usize + 1,
            self.suit.to_string().to_lowercase()
        )
    }

    fn pos(&self) -> egui::Pos2 {
        self.pos
    }

    fn set_pos(&mut self, pos: egui::Pos2) {
        self.pos = pos;
    }

    fn translate(&mut self, amt: egui::Vec2) {
        self.pos += amt;
    }
}

#[derive(Default)]
pub struct ConventionalCardIter {
    suit: SuitIter,
    rank: RankIter,
}


impl Iterator for ConventionalCardIter {
    type Item = ConventionalCard;

    fn next(&mut self) -> Option<Self::Item> {
        let rank = self.rank.value;
        let suit = self.suit.value;
        if rank.is_none() {
            if suit.is_none() {
                return None;
            } else {
                self.rank = Default::default();
                self.suit.next();
            }
        }
        Some(ConventionalCard {
            rank: self.rank.next()?,
            suit: self.suit.value?,
            pos: Default::default(),
        })
    }
}

#[derive(Clone, Copy, Default, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum Suit {
    #[default]
    Heart,
    Diamond,
    Club,
    Spade,
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl Suit {
    #[allow(unused)]
    pub fn iter() -> SuitIter {
        Default::default()
    }

    pub const fn len() -> usize {
        4
    }
}

impl From<usize> for Suit {
    fn from(index: usize) -> Self {
        match index {
            0 => Suit::Heart,
            1 => Suit::Diamond,
            2 => Suit::Club,
            3 => Suit::Spade,
            _ => {
                panic!("Invalid index: {}", index)
            }
        }
    }
}

pub struct SuitIter {
    value: Option<Suit>,
}

impl Default for SuitIter {
    fn default() -> Self {
        Self {
            value: Some(Default::default()),
        }
    }
}

impl Iterator for SuitIter {
    type Item = Suit;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.value;
        match self.value? {
            Suit::Heart => self.value = Some(Suit::Diamond),
            Suit::Diamond => self.value = Some(Suit::Club),
            Suit::Club => self.value = Some(Suit::Spade),
            Suit::Spade => self.value = None,
        };
        current
    }
}

#[derive(Clone, Copy, Default, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum Rank {
    #[default]
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
}

impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl Rank {
    #[allow(unused)]
    pub fn iter() -> RankIter {
        Default::default()
    }

    pub const fn len() -> usize {
        13
    }
}

impl From<usize> for Rank {
    fn from(index: usize) -> Self {
        match index {
            0 => Rank::Ace,
            1 => Rank::Two,
            2 => Rank::Three,
            3 => Rank::Four,
            4 => Rank::Five,
            5 => Rank::Six,
            6 => Rank::Seven,
            7 => Rank::Eight,
            8 => Rank::Nine,
            9 => Rank::Ten,
            10 => Rank::Jack,
            11 => Rank::Queen,
            12 => Rank::King,
            _ => {
                panic!("Invalid index: {}", index)
            }
        }
    }
}

pub struct RankIter {
    value: Option<Rank>,
}

impl Default for RankIter {
    fn default() -> Self {
        Self {
            value: Some(Default::default()),
        }
    }
}

impl Iterator for RankIter {
    type Item = Rank;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.value;
        match self.value? {
            Rank::Ace => self.value = Some(Rank::Two),
            Rank::Two => self.value = Some(Rank::Three),
            Rank::Three => self.value = Some(Rank::Four),
            Rank::Four => self.value = Some(Rank::Five),
            Rank::Five => self.value = Some(Rank::Six),
            Rank::Six => self.value = Some(Rank::Seven),
            Rank::Seven => self.value = Some(Rank::Eight),
            Rank::Eight => self.value = Some(Rank::Nine),
            Rank::Nine => self.value = Some(Rank::Ten),
            Rank::Ten => self.value = Some(Rank::Jack),
            Rank::Jack => self.value = Some(Rank::Queen),
            Rank::Queen => self.value = Some(Rank::King),
            Rank::King => self.value = None,
        };
        current
    }
}

pub struct Backside {}

impl Default for Backside {
    fn default() -> Self {
        Self::new()
    }
}

impl Backside {
    pub fn new() -> Self {
        Self {}
    }
}

impl card::Card for Backside {
    fn img_path(&self) -> String {
        "https://placehold.co/100x144/png?text=Hello World!".to_string()
    }

    fn pos(&self) -> egui::Pos2 {
        egui::pos2(500.0, 500.0)
    }

    fn set_pos(&mut self, _pos: egui::Pos2) {}

    fn translate(&mut self, _amt: egui::Vec2) {}
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
            pos: egui::pos2(314.15, 271.828),
            size: Default::default(),
            inner_margin: 5,
            max_cards: 0,
        };
        x.max_cards(5);
        x
    }
}

impl Field for Stack {
    fn ui(&self, ui: &mut egui::Ui) -> egui::Response {
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
                            card.draw(
                                ui,
                                Some(card_pos),
                                Some(egui::Sense::all()),
                                Some(egui::Order::Foreground),
                                true,
                            );
                        }
                    },
                )
                .response
            })
            .inner
    }
    fn pos(&self) -> egui::Pos2 {
        self.pos
    }
    fn set_pos(&mut self, pos: egui::Pos2) {
        self.pos = pos;
    }
}

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

impl Field for HandLayout {
    fn ui(&self, ui: &mut egui::Ui) -> egui::Response {
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
                        for (idx, card) in self.cards.iter().enumerate() {
                            let card_pos = next_pos.add(self.card_pos(idx));
                            if selected.is_some() && idx == selected.unwrap() {
                                continue;
                            }
                            card.draw(
                                ui,
                                Some(card_pos),
                                Some(egui::Sense::all()),
                                Some(egui::Order::Foreground),
                                true,
                            );
                        }
                        if selected.is_some() {
                            if let Some(card) = self.cards.get(selected.unwrap()) {
                                let card_pos = next_pos
                                    .add(self.card_pos(selected.unwrap()))
                                    .add(egui::vec2(0.0, -10.0));
                                egui::Area::new(ui.next_auto_id())
                                    .order(egui::Order::Foreground)
                                    .sense(egui::Sense::all())
                                    .current_pos(card_pos)
                                    .show(ui.ctx(), |ui| {
                                        egui::Frame::new()
                                            .stroke(egui::Stroke::new(2.0, egui::Color32::RED))
                                            .corner_radius(egui::CornerRadius::same(2))
                                            .show(ui, |ui| {
                                                ui.allocate_new_ui(egui::UiBuilder::new(), |ui| {
                                                    ui.add(card.as_ref());
                                                });
                                            });
                                    });
                            }
                        }
                    },
                )
                .response
            })
            .inner
    }
    fn pos(&self) -> egui::Pos2 {
        self.pos
    }
    fn set_pos(&mut self, pos: egui::Pos2) {
        self.pos = pos;
    }
}
