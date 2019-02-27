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
