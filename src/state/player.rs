use crate::state::card::Card;
use std::fmt::{Display, Formatter};

#[derive(Debug, Default)]
pub struct Player {
    cards: Vec<Card>,
    discards: Vec<Card>,
}

impl Player {
    pub fn add_card_to_hand(&mut self, card: Card) {
        self.cards.push(card);
    }
}

impl Display for Player {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let card_desc = self
            .cards
            .iter()
            .map(|c| c.to_string())
            .collect::<Vec<_>>()
            .join(", ");
        let discard_desc = self
            .discards
            .iter()
            .map(|c| c.to_string())
            .collect::<Vec<_>>()
            .join(", ");

        write!(f, "{}\nDiscards: {}", card_desc, discard_desc)
    }
}
