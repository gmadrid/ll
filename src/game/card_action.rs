use crate::state::Card;
use crate::Error;
use fehler::{throw, throws};

pub struct CardAction {
    card: Card,
    current: usize,
    target: Option<usize>,
    guess: Option<Card>,
}

pub struct CardActionBuilder {
    card: Card,
    current: usize,
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
