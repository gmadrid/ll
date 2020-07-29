use thiserror::Error;

mod game;
mod state;

#[derive(Debug, Error)]
pub enum LoveLetterError {
    #[error("You can only play Love Letter with 3 or 4 players. {0} is not allowed")]
    InvalidNumberOfPlayers(usize),

    #[error("Invalid player number: {0}")]
    InvalidPlayerNumber(usize),
}

use LoveLetterError as Error;

pub use game::{Game, GameBuilder};
