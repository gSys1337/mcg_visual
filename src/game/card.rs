#[cfg(target_arch = "wasm32")]
use crate::log;
use egui;

#[derive(Clone, Copy)]
pub struct ConventionalCard {
    pub suit: Suit,
    pub rank: Rank,
    pub pos: egui::Pos2,
}

#[derive(Clone, PartialEq, Debug, Default, Copy)]
pub enum Suit {
    #[default]
    Heart,
    Diamond,
    Club,
    Spade,
}

#[derive(Clone, Default, Debug, PartialEq, PartialOrd, Copy)]
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

impl Suit {
    pub fn as_path_str(&self) -> &'static str {
        match self {
            Suit::Heart => "heart",
            Suit::Diamond => "diamond",
            Suit::Club => "club",
            Suit::Spade => "spade",
        }
    }

    pub fn name_str(&self) -> &'static str {
        match self {
            Suit::Heart => "Heart",
            Suit::Diamond => "Diamond",
            Suit::Club => "Club",
            Suit::Spade => "Spade",
        }
    }

    pub fn all_vec() -> Vec<Suit> {
        vec![Suit::Heart, Suit::Diamond, Suit::Club, Suit::Spade]
    }
}

impl Rank {
    pub fn as_path_str(&self) -> &'static str {
        match self {
            Rank::Ace => "1",
            Rank::Two => "2",
            Rank::Three => "3",
            Rank::Four => "4",
            Rank::Five => "5",
            Rank::Six => "6",
            Rank::Seven => "7",
            Rank::Eight => "8",
            Rank::Nine => "9",
            Rank::Ten => "10",
            Rank::Jack => "11",
            Rank::Queen => "12",
            Rank::King => "13",
        }
    }

    pub fn all_vec() -> Vec<Rank> {
        vec![
            Rank::Ace,
            Rank::Two,
            Rank::Three,
            Rank::Four,
            Rank::Five,
            Rank::Six,
            Rank::Seven,
            Rank::Eight,
            Rank::Nine,
            Rank::Ten,
            Rank::Jack,
            Rank::Queen,
            Rank::King,
        ]
    }
}

impl ConventionalCard {
    fn all_cards() -> Vec<ConventionalCard> {
        let mut cards: Vec<ConventionalCard> = Vec::new();
        let suits = Suit::all_vec();
        let ranks = Rank::all_vec();
        for suit in suits.iter() {
            for rank in ranks.iter() {
                cards.push(ConventionalCard {
                    rank: *rank,
                    suit: *suit,
                    pos: egui::pos2(100.0, 100.0),
                })
            }
        }
        cards
    }

    pub fn load_image_sources() -> Vec<Vec<egui::ImageSource<'static>>> {
        let mut sources = Vec::new();
        let mut hearts = Vec::new();
        let mut diamonds = Vec::new();
        let mut clubs = Vec::new();
        let mut spades = Vec::new();
        for card in ConventionalCard::all_cards() {
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
        let first = match self.suit {
            Suit::Heart => 0,
            Suit::Diamond => 1,
            Suit::Club => 2,
            Suit::Spade => 3,
        };
        let second = self
            .rank
            .as_path_str()
            .parse::<usize>()
            .expect("Every Rank member has a str which is correct")
            - 1;
        (first, second)
    }
}
