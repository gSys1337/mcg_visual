use egui;

#[derive(Clone)]
struct ConventionalCard {
    suit: Suit,
    rank: Rank,
}

#[derive(Clone, PartialEq, Debug, Default)]
enum Suit {
    #[default]
    Heart,
    Diamond,
    Club,
    Spade,
}

#[derive(Clone, Default, Debug, PartialEq, PartialOrd)]
enum Rank {
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
    pub fn as_str(&self) -> &'static str {
        match self {
            Suit::Heart => {"heart"}
            Suit::Diamond => {"diamond"}
            Suit::Club => {"club"}
            Suit::Spade => {"spade"}
        }
    }
}

impl Rank {
    pub fn as_str(&self) -> &'static str {
        match self {
            Rank::Ace => {"1"}
            Rank::Two => {"2"}
            Rank::Three => {"3"}
            Rank::Four => {"4"}
            Rank::Five => {"5"}
            Rank::Six => {"6"}
            Rank::Seven => {"7"}
            Rank::Eight => {"8"}
            Rank::Nine => {"9"}
            Rank::Ten => {"10"}
            Rank::Jack => {"11"}
            Rank::Queen => {"12"}
            Rank::King => {"13"}
        }
    }
}

impl egui::Widget for &mut ConventionalCard {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        todo!()
    }
}
