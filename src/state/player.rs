use crate::state::card::Card;
use crate::Error;
use fehler::{throw, throws};
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct Player {
    name: String,
    cards: Vec<Card>,
    discards: Vec<Card>,
}

impl Player {
    pub fn with_name(name: &str) -> Player {
        Player {
            name: name.to_string(),
            cards: Default::default(),
            discards: Default::default(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn add_card_to_hand(&mut self, card: Card) {
        self.cards.push(card);
    }

    #[throws]
    pub fn card_in_hand(&self) -> Card {
        if self.cards.len() != 1 {
            throw!(Error::InvalidNumberOfCards(self.cards.len()))
        }
        self.cards[0]
    }

    pub fn card_index(&self, card: Card) -> Option<usize> {
        self.cards
            .iter()
            .enumerate()
            .find(|(idx, val)| **val == card)
            .map(|(idx, _)| idx)
    }

    #[throws]
    pub fn discard(&mut self, card: Card) {
        if let Some(index) = self.card_index(card) {
            self.cards.remove(index);
            self.discards.push(card);
        } else {
            throw!(Error::DiscardingCardNotInHand(card));
        }
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_name() {
        let name = "Dwigt";
        let player = Player::with_name(name);
        assert_eq!(name, player.name());
    }

    #[test]
    fn test_add_card() {
        let mut player = Player::with_name("Guilty Joe");
        player.add_card_to_hand(Card::Baron);
        assert_eq!(Card::Baron, player.card_in_hand().unwrap());
    }

    #[test]
    fn test_card_in_hand() {
        let mut player = Player::with_name("Merrywinkle");

        assert!(player.card_in_hand().is_err());

        player.add_card_to_hand(Card::Princess);
        assert!(player.card_in_hand().is_ok());
        assert_eq!(Card::Princess, player.card_in_hand().unwrap());

        player.add_card_to_hand(Card::Prince);
        assert!(player.card_in_hand().is_err());
    }

    #[test]
    fn test_card_index() {
        let mut player = Player::with_name("Greta Bones");

        assert_eq!(None, player.card_index(Card::Princess));

        player.add_card_to_hand(Card::Princess);
        assert_eq!(Some(0), player.card_index(Card::Princess));
        assert_eq!(None, player.card_index(Card::Prince));

        player.add_card_to_hand(Card::Baron);
        assert_eq!(Some(0), player.card_index(Card::Princess));
        assert_eq!(Some(1), player.card_index(Card::Baron));
        assert_eq!(None, player.card_index(Card::Prince));

        let mut player = Player::with_name("Baron lover");
        player.add_card_to_hand(Card::Baron);
        player.add_card_to_hand(Card::Baron);

        assert_eq!(Some(0), player.card_index(Card::Baron));
    }

    #[test]
    fn test_discard() {
        let mut player = Player::with_name("Bryce Wayne");

        player.add_card_to_hand(Card::Princess);
        assert_eq!(Some(0), player.card_index(Card::Princess));
        player.discard(Card::Princess).unwrap();

        player.add_card_to_hand(Card::Princess);
        assert_eq!(Some(0), player.card_index(Card::Princess));
        assert!(player.discard(Card::Prince).is_err());

        player.add_card_to_hand(Card::Prince);
        assert!(player.discard(Card::Baron).is_err());

        assert!(player.discard(Card::Princess).is_ok());
        assert!(player.card_index(Card::Princess).is_none());
        assert!(player.card_index(Card::Prince).is_some());

        assert!(player.discard(Card::Princess).is_err());
        assert!(player.discard(Card::Prince).is_ok());
        assert!(player.card_index(Card::Princess).is_none());
        assert!(player.card_index(Card::Prince).is_none());
    }
}
