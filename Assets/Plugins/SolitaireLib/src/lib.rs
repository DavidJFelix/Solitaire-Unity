use std;
use rand;
use core::iter::Zip;

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

#[derive(Debug, Clone)]
struct GameState {
    tableau: [Vec<Card>; 7],
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

fn shuffle_deck(deck: Vec<Card>) -> Vec<Card> {
    let mut order: Vec<(u64, &Card)> = vec!(0..52)
        .iter()
        .map(|_| -> u64{ rand::random::<u64>() })
        .zip(deck.iter())
        .collect();
    order.sort_by_key(|(order, _)| -> u64{ *order });
    order.iter().map(|(_, card)| -> Card { *card.clone() }).collect()
}

trait Game {
    fn new() -> Self;
}