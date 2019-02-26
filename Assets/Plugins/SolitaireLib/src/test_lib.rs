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
}