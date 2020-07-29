use crate::state::card::Card;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    pub fn new_shuffled() -> Deck {
        let mut deck = Deck::default();
        deck.shuffle();
        deck
    }

    pub fn cards_remaining(&self) -> u32 {
        self.cards.len() as u32
    }

    pub fn deal_one(&mut self) -> Option<Card> {
        self.cards.pop()
    }

    pub fn shuffle(&mut self) {
        self.cards.shuffle(&mut thread_rng());
    }
}

impl std::default::Default for Deck {
    fn default() -> Self {
        Deck {
            cards: vec![
                Card::Guard,
                Card::Guard,
                Card::Guard,
                Card::Guard,
                Card::Guard,
                Card::Priest,
                Card::Priest,
                Card::Baron,
                Card::Baron,
                Card::Handmaid,
                Card::Handmaid,
                Card::Prince,
                Card::Prince,
                Card::King,
                Card::Countess,
                Card::Princess,
            ],
        }
    }
}

impl Display for Deck {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.cards.first() {
            None => write!(f, "Empty deck"),
            Some(card) => write!(f, "{} left. Top: {}", self.cards.len(), card),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_basic_functions() {
        let mut deck = Deck::default();
        assert_eq!(16, deck.cards_remaining());

        assert!(deck.deal_one().is_some());
        assert!(deck.deal_one().is_some());
        assert!(deck.deal_one().is_some());
        assert!(deck.deal_one().is_some());
        assert!(deck.deal_one().is_some());
        assert!(deck.deal_one().is_some());
        assert!(deck.deal_one().is_some());
        assert!(deck.deal_one().is_some());
        assert!(deck.deal_one().is_some());
        assert!(deck.deal_one().is_some());
        assert!(deck.deal_one().is_some());
        assert!(deck.deal_one().is_some());
        assert!(deck.deal_one().is_some());
        assert!(deck.deal_one().is_some());
        assert!(deck.deal_one().is_some());
        assert_eq!(1, deck.cards_remaining());
        assert!(deck.deal_one().is_some());
        assert_eq!(0, deck.cards_remaining());
        assert!(deck.deal_one().is_none());
        assert_eq!(0, deck.cards_remaining());
    }

    #[test]
    fn test_shuffle() {
        // Just making sure that the default deck and the shuffled deck have the same cards.
        let mut deck = Deck::default();
        let mut cards = Vec::default();
        while let Some(card) = deck.deal_one() {
            cards.push(card);
        }

        let mut shuffled = Deck::default();
        let mut shuffled_cards = Vec::default();
        while let Some(card) = shuffled.deal_one() {
            shuffled_cards.push(card);
        }

        cards.sort();
        shuffled_cards.sort();
        assert_eq!(cards, shuffled_cards)
    }
}
