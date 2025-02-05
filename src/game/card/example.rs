use crate::game::card;
#[cfg(target_arch = "wasm32")]
#[allow(unused_imports)]
use crate::log;
use egui;
use std::fmt;

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

    fn translate(&mut self, amt: egui::Vec2) {
        self.pos += amt;
    }
}

pub struct ConventionalCardIter {
    suit: SuitIter,
    rank: RankIter,
}

impl Default for ConventionalCardIter {
    fn default() -> Self {
        Self {
            suit: Default::default(),
            rank: Default::default(),
        }
    }
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
    pub fn iter() -> SuitIter {
        Default::default()
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
    pub fn iter() -> RankIter {
        Default::default()
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
