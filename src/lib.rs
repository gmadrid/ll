use crate::state::Card;
use thiserror::Error;

mod game;
mod messenger;
mod state;

#[derive(Debug, Error, Eq, PartialEq)]
pub enum LoveLetterError {
    #[error("Discarding a card, {0}, that is not in the player's hand")]
    DiscardingCardNotInHand(Card),

    #[error("Internal error: player should only have one card, but they have {0}")]
    InvalidNumberOfCards(usize),

    #[error("You can only play Love Letter with 3 or 4 players. {0} is not allowed")]
    InvalidNumberOfPlayers(usize),

    #[error("Invalid player number: {0}")]
    InvalidPlayerNumber(usize),

    #[error("Only the current player can take an action. {0} provided.")]
    BadActionNotCurrentPlayer(usize),
    #[error("Inactive player targeted: {0}")]
    BadActionTargetingInactive(usize),
    #[error("Protected player targeted: {0}")]
    BadActionTargetingProtected(usize),
    #[error("Cannot target self")]
    BadActionCannotTargetSelf,
    #[error("Missing target")]
    BadActionMissingTarget,
    #[error("Missing guess")]
    BadActionMissingGuess,
    #[error("Player {0} does not have card, {1}")]
    BadActionPlayerDoesntHaveCard(usize, Card),
}

use LoveLetterError as Error;

pub use game::{Game, GameBuilder};
