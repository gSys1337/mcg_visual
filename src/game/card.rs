#[cfg(target_arch = "wasm32")]
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
    pub fn iter() -> ConventionalCardIter {
        Default::default()
    }

    pub fn load_image_sources() -> Vec<Vec<egui::ImageSource<'static>>> {
        let mut sources = Vec::new();
        let mut hearts = Vec::new();
        let mut diamonds = Vec::new();
        let mut clubs = Vec::new();
        let mut spades = Vec::new();
        for card in ConventionalCard::iter() {
            match card.suit {
                Suit::Heart => match card.rank {
                    Rank::Ace => {
                        hearts.push(egui::include_image!("../../media/img_cards/1_heart.png"))
                    }
                    Rank::Two => {
                        hearts.push(egui::include_image!("../../media/img_cards/2_heart.png"))
                    }
                    Rank::Three => {
                        hearts.push(egui::include_image!("../../media/img_cards/3_heart.png"))
                    }
                    Rank::Four => {
                        hearts.push(egui::include_image!("../../media/img_cards/4_heart.png"))
                    }
                    Rank::Five => {
                        hearts.push(egui::include_image!("../../media/img_cards/5_heart.png"))
                    }
                    Rank::Six => {
                        hearts.push(egui::include_image!("../../media/img_cards/6_heart.png"))
                    }
                    Rank::Seven => {
                        hearts.push(egui::include_image!("../../media/img_cards/7_heart.png"))
                    }
                    Rank::Eight => {
                        hearts.push(egui::include_image!("../../media/img_cards/8_heart.png"))
                    }
                    Rank::Nine => {
                        hearts.push(egui::include_image!("../../media/img_cards/9_heart.png"))
                    }
                    Rank::Ten => {
                        hearts.push(egui::include_image!("../../media/img_cards/10_heart.png"))
                    }
                    Rank::Jack => {
                        hearts.push(egui::include_image!("../../media/img_cards/11_heart.png"))
                    }
                    Rank::Queen => {
                        hearts.push(egui::include_image!("../../media/img_cards/12_heart.png"))
                    }
                    Rank::King => {
                        hearts.push(egui::include_image!("../../media/img_cards/13_heart.png"))
                    }
                },
                Suit::Diamond => {
                    match card.rank {
                        Rank::Ace => diamonds
                            .push(egui::include_image!("../../media/img_cards/1_diamond.png")),
                        Rank::Two => diamonds
                            .push(egui::include_image!("../../media/img_cards/2_diamond.png")),
                        Rank::Three => diamonds
                            .push(egui::include_image!("../../media/img_cards/3_diamond.png")),
                        Rank::Four => diamonds
                            .push(egui::include_image!("../../media/img_cards/4_diamond.png")),
                        Rank::Five => diamonds
                            .push(egui::include_image!("../../media/img_cards/5_diamond.png")),
                        Rank::Six => diamonds
                            .push(egui::include_image!("../../media/img_cards/6_diamond.png")),
                        Rank::Seven => diamonds
                            .push(egui::include_image!("../../media/img_cards/7_diamond.png")),
                        Rank::Eight => diamonds
                            .push(egui::include_image!("../../media/img_cards/8_diamond.png")),
                        Rank::Nine => diamonds
                            .push(egui::include_image!("../../media/img_cards/9_diamond.png")),
                        Rank::Ten => diamonds
                            .push(egui::include_image!("../../media/img_cards/10_diamond.png")),
                        Rank::Jack => diamonds
                            .push(egui::include_image!("../../media/img_cards/11_diamond.png")),
                        Rank::Queen => diamonds
                            .push(egui::include_image!("../../media/img_cards/12_diamond.png")),
                        Rank::King => diamonds
                            .push(egui::include_image!("../../media/img_cards/13_diamond.png")),
                    }
                }
                Suit::Club => match card.rank {
                    Rank::Ace => {
                        clubs.push(egui::include_image!("../../media/img_cards/1_club.png"))
                    }
                    Rank::Two => {
                        clubs.push(egui::include_image!("../../media/img_cards/2_club.png"))
                    }
                    Rank::Three => {
                        clubs.push(egui::include_image!("../../media/img_cards/3_club.png"))
                    }
                    Rank::Four => {
                        clubs.push(egui::include_image!("../../media/img_cards/4_club.png"))
                    }
                    Rank::Five => {
                        clubs.push(egui::include_image!("../../media/img_cards/5_club.png"))
                    }
                    Rank::Six => {
                        clubs.push(egui::include_image!("../../media/img_cards/6_club.png"))
                    }
                    Rank::Seven => {
                        clubs.push(egui::include_image!("../../media/img_cards/7_club.png"))
                    }
                    Rank::Eight => {
                        clubs.push(egui::include_image!("../../media/img_cards/8_club.png"))
                    }
                    Rank::Nine => {
                        clubs.push(egui::include_image!("../../media/img_cards/9_club.png"))
                    }
                    Rank::Ten => {
                        clubs.push(egui::include_image!("../../media/img_cards/10_club.png"))
                    }
                    Rank::Jack => {
                        clubs.push(egui::include_image!("../../media/img_cards/11_club.png"))
                    }
                    Rank::Queen => {
                        clubs.push(egui::include_image!("../../media/img_cards/12_club.png"))
                    }
                    Rank::King => {
                        clubs.push(egui::include_image!("../../media/img_cards/13_club.png"))
                    }
                },
                Suit::Spade => match card.rank {
                    Rank::Ace => {
                        spades.push(egui::include_image!("../../media/img_cards/1_spade.png"))
                    }
                    Rank::Two => {
                        spades.push(egui::include_image!("../../media/img_cards/2_spade.png"))
                    }
                    Rank::Three => {
                        spades.push(egui::include_image!("../../media/img_cards/3_spade.png"))
                    }
                    Rank::Four => {
                        spades.push(egui::include_image!("../../media/img_cards/4_spade.png"))
                    }
                    Rank::Five => {
                        spades.push(egui::include_image!("../../media/img_cards/5_spade.png"))
                    }
                    Rank::Six => {
                        spades.push(egui::include_image!("../../media/img_cards/6_spade.png"))
                    }
                    Rank::Seven => {
                        spades.push(egui::include_image!("../../media/img_cards/7_spade.png"))
                    }
                    Rank::Eight => {
                        spades.push(egui::include_image!("../../media/img_cards/8_spade.png"))
                    }
                    Rank::Nine => {
                        spades.push(egui::include_image!("../../media/img_cards/9_spade.png"))
                    }
                    Rank::Ten => {
                        spades.push(egui::include_image!("../../media/img_cards/10_spade.png"))
                    }
                    Rank::Jack => {
                        spades.push(egui::include_image!("../../media/img_cards/11_spade.png"))
                    }
                    Rank::Queen => {
                        spades.push(egui::include_image!("../../media/img_cards/12_spade.png"))
                    }
                    Rank::King => {
                        spades.push(egui::include_image!("../../media/img_cards/13_spade.png"))
                    }
                },
            }
        }
        sources.push(hearts);
        sources.push(diamonds);
        sources.push(clubs);
        sources.push(spades);
        sources
    }

    pub fn get_source_index(&self) -> (usize, usize) {
        let first = self.suit as usize;
        let second = self.rank as usize;
        (first, second)
    }
}

pub struct ConventionalCardIter {
    suit: SuitIter,
    rank: RankIter,
}

impl Default for ConventionalCardIter {
    fn default() -> Self {
        Self { suit: Default::default(), rank: Default::default() }
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
        Self { value: Some(Default::default()) }
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
        Self { value: Some(Default::default()) }
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
