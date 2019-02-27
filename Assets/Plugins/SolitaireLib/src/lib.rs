use std;
use rand;
use core::iter::Zip;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn standard_deck_has_52_cards() {
        let deck = create_standard_deck();
        assert_eq!(deck.len(), 52);
    }

    #[test]
    fn standard_deck_has_unique_cards() {
        let mut deck = create_standard_deck();
        deck.sort();
        deck.dedup();
        assert_eq!(deck.len(), 52);
    }

    #[test]
    fn shuffled_deck_has_same_number_of_cards() {
        let deck = create_standard_deck();
        let shuffled_deck = shuffle_deck(&deck);

        assert_eq!(deck.len(), shuffled_deck.len());
    }

    #[test]
    fn new_game_has_unique_cards() {}

    #[test]
    fn new_game_has_correct_layout() {
        let game = GameState::new();
        assert_eq!(game.tableau[0].face_up.len(), 1);
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
enum Color {
    Black,
    Red,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
enum Rank {
    Ace = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    Ten = 10,
    Jack = 11,
    Queen = 12,
    King = 13,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
struct Card {
    rank: Rank,
    suit: Suit,
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
struct TableauPile {
    pub face_down: Vec<Card>,
    pub face_up: Vec<Card>,
}

#[derive(Debug, Clone)]
struct GameState {
    tableau: [TableauPile; 7],
    foundation: [Vec<Card>; 4],
    hand: Vec<Card>,
    waste: Vec<Card>,
}

fn create_standard_deck() -> Vec<Card> {
    [
        Suit::Clubs,
        Suit::Diamonds,
        Suit::Hearts,
        Suit::Spades,
    ]
        .iter()
        .flat_map(|&suit| -> Vec<Card> {
            [
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
                .iter()
                .map(|&rank| -> Card{ Card { rank, suit } })
                .collect()
        }
        )
        .collect()
}

fn get_suit_color(suit: Suit) -> Color {
    match suit {
        Suit::Clubs => Color::Black,
        Suit::Diamonds => Color::Red,
        Suit::Hearts => Color::Red,
        Suit::Spades => Color::Black,
    }
}

fn parse_suit(suit: &str) -> Result<Suit, &'static str> {
    match suit {
        "clubs" => Ok(Suit::Clubs),
        "diamonds" => Ok(Suit::Diamonds),
        "hearts" => Ok(Suit::Hearts),
        "spades" => Ok(Suit::Spades),
        _ => Err("invalid suit"),
    }
}

//impl std::iter::FromIterator<(&u64, &Card> for (u64, &Card) {
//}

fn shuffle_deck(deck: &Vec<Card>) -> Vec<Card> {
    let mut order: Vec<(u64, &Card)> = (0..52)
        .map(|_| -> u64{ rand::random::<u64>() })
        .zip(deck.iter())
        .collect();
    order.sort_by_key(|(order, _)| -> u64{ *order });
    order.iter()
        .map(|(_, &card)| -> Card { card.clone() })
        .collect()
}

trait Game {
    fn new() -> Self;
}

impl Game for GameState {
    fn new() -> Self {
        let deck = shuffle_deck(&create_standard_deck());
        GameState {
            tableau: [
                TableauPile {
                    face_down: Vec::new(),
                    face_up: deck[0..1].to_vec(),
                },
                TableauPile {
                    face_down: deck[1..2].to_vec(),
                    face_up: deck[2..3].to_vec(),
                },
                TableauPile {
                    face_down: deck[3..5].to_vec(),
                    face_up: deck[5..6].to_vec(),
                },
                TableauPile {
                    face_down: deck[6..9].to_vec(),
                    face_up: deck[9..10].to_vec(),
                },
                TableauPile {
                    face_down: deck[10..14].to_vec(),
                    face_up: deck[14..15].to_vec(),
                },
                TableauPile {
                    face_down: deck[15..20].to_vec(),
                    face_up: deck[20..21].to_vec(),
                },
                TableauPile {
                    face_down: deck[21..28].to_vec(),
                    face_up: deck[28..29].to_vec(),
                },
            ],
            foundation: [
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
            ],
            hand: deck[29..].to_vec(),
            waste: Vec::new(),
        }
    }
}