use crate::state::card::Card;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::fmt::{Display, Formatter};

/// A deck of LoveLetter cards.
///
/// Enforces no rules, but allows only operations that would occur in a
/// game of LoveLetter.
#[derive(Debug)]
pub struct Deck {
    // Cards are stored in reverse order so that we can deal off the
    // efficient end.
    cards: Vec<Box<dyn Card>>,
}

impl Deck {
    /// Create a new Deck containing the specified cards.
    ///
    /// The cards will be dealt in the order that they are in the input vector.
    pub fn new(mut cards: Vec<Box<dyn Card>>) -> Deck {
        cards.reverse();
        Deck { cards }
    }

    /// Returns the number of cards remaining in the deck.
    pub fn cards_remaining(&self) -> usize {
        self.cards.len()
    }

    /// Deals one card off the "top" of the deck.
    ///
    /// If the deck is empty(), returns None.
    pub fn deal_one(&mut self) -> Option<Box<dyn Card>> {
        self.cards.pop()
    }

    /// Shuffles the cards in the deck.
    ///
    /// After shuffling, the cards will be dealt in a random order.
    pub fn shuffle(&mut self) {
        self.cards.shuffle(&mut thread_rng());
    }
}

impl Display for Deck {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.cards.len() == 0 {
            write!(f, "Empty deck")
        } else {
            let s = self
                .cards
                .iter()
                .map(|c| c.name())
                .collect::<Vec<_>>()
                .join(", ");

            write!(f, "{} left. {}", self.cards.len(), s)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::state::testcard::TestCard;

    #[test]
    fn test_basic() {
        let mut deck = Deck::new(vec![
	    TestCard::boxed("One", 1),
	    TestCard::boxed("Two", 2),
	    TestCard::boxed("Three", 3),
        ]);

        assert_eq!(3, deck.cards_remaining());
        assert_eq!("One", deck.deal_one().unwrap().name());
        assert_eq!(2, deck.cards_remaining());
        assert_eq!("Two", deck.deal_one().unwrap().name());
        assert_eq!(1, deck.cards_remaining());
        assert_eq!("Three", deck.deal_one().unwrap().name());
        assert_eq!(0, deck.cards_remaining());
    }

    #[test]
    fn test_shuffle() {
        // We aren't testing the randomness or anything.
        // We just want to make sure that all of the cards are in the shuffled
        // deck exactly once.
        let mut deck = Deck::new(vec![
	    TestCard::boxed("One", 1),
	    TestCard::boxed("Two", 2),
	    TestCard::boxed("Three", 3),
	    TestCard::boxed("Four", 4),
	    TestCard::boxed("Five", 5),
	    TestCard::boxed("Six", 6),
	    TestCard::boxed("Seven", 7),
        ]);

        deck.shuffle();

        let mut card_names: Vec<&str> = deck.cards.iter().map(|c| c.name()).collect();
        card_names.sort();
        assert_eq!(
            &vec!["Five", "Four", "One", "Seven", "Six", "Three", "Two"],
            &card_names
        );
    }
}
