use crate::state::Card;
use crate::Error;
use fehler::{throw, throws};

pub struct CardAction {
    card: Card,
    current: usize,
    target: Option<usize>,
    guess: Option<Card>,
}

impl CardAction {
    pub fn new(
        card: Card,
        current: usize,
        target: Option<usize>,
        guess: Option<Card>,
    ) -> CardAction {
        CardAction {
            card,
            current,
            target,
            guess,
        }
    }

    pub fn card(&self) -> Card {
        self.card
    }

    pub fn current(&self) -> usize {
        self.current
    }

    #[throws]
    pub fn target(&self) -> usize {
        if let Some(target) = self.target {
            target
        } else {
            throw!(Error::BadActionMissingTarget);
        }
    }

    #[throws]
    pub fn guess(&self) -> Card {
        if let Some(guess) = self.guess {
            guess
        } else {
            throw!(Error::BadActionMissingGuess);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[throws]
    fn test_basic() {
	let action = CardAction::new(Card::Baron, 1, None, None);
	assert_eq!(Card::Baron, action.card());
	assert_eq!(1, action.current());
	assert!(action.target().is_err());
	assert!(action.guess().is_err());

	let full_action = CardAction::new(Card::Prince, 3, Some(2), Some(Card::Princess));
	assert_eq!(Card::Prince, full_action.card());
	assert_eq!(3, full_action.current());
	assert_eq!(2, full_action.target()?);
	assert_eq!(Card::Princess, full_action.guess()?);
    }
}
