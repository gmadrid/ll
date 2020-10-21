//! Module representing the physical state of a LoveLetter game.
//! 
//! State has no concept of the game rules, and nothing prevents creating
//! illegal game states. 

mod card;
mod deck;
mod player;
mod table;

#[cfg(test)]
mod testcard;

pub use card::Card;
pub use deck::Deck;
pub use player::Player;
pub use table::Table;
