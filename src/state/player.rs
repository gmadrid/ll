use crate::state::card::*;
use crate::Error;
use fehler::{throw, throws};
use std::fmt::{Display, Formatter};

/// A player in a Love Letter game. This struct describes only the
/// physical aspects of a player - those things you can *see* at the
/// table. It has no knowledge of the rules of the game, and creation
/// of a Player with illegal state is possible.
///
/// A player has a name, a hand, and a discard pile.
#[derive(Debug)]
pub struct Player {
    /// The player's name.
    name: String,
    /// The player's hand of cards. Usually one or two cards.
    hand: Vec<Box<dyn Card>>,
    /// The player's discard pile.
    discards: Vec<Box<dyn Card>>,
}

impl Player {
    /// Creates a new Player with the given name.
    /// The player starts with an empty hand and an empty discard
    /// pile.
    pub fn with_name(name: &str) -> Player {
        Player {
            name: name.to_string(),
            hand: Default::default(),
            discards: Default::default(),
        }
    }

    /// Returns the Player's name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Adds a card to the Player's hand.
    pub fn add_card_to_hand(&mut self, card: Box<dyn Card>) {
        self.hand.push(card);
    }

    /// Returns the Card in the Player's hand, iff the Player has a
    /// single card. Otherwise, throws an Err.
    #[throws]
    pub fn card_in_hand(&self) -> &dyn Card {
        if self.hand.len() != 1 {
            throw!(Error::InvalidNumberOfCards(self.hand.len()))
        }
        self.hand[0].as_ref()
    }

    /// Returns the Card at the specified index in the Player's hand.
    /// Throws an Err on a bad index.
    #[throws]
    pub fn discard(&mut self, index: usize) {
        if index < self.hand.len() {
            let card = self.hand.remove(index);
            self.discards.push(card);
        } else {
            throw!(Error::InternalErrorBadCardIndex(index));
        }
    }
}

impl Display for Player {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let card_desc = self
            .hand
            .iter()
            .map(|c| c.as_ref().to_string())
            .collect::<Vec<_>>()
            .join(", ");
        let discard_desc = self
            .discards
            .iter()
            .map(|c| c.as_ref().to_string())
            .collect::<Vec<_>>()
            .join(", ");

        write!(f, "{}\nDiscards: {}", card_desc, discard_desc)
    }
}

#[cfg(test)]
mod test {
    use super::super::testcard::TestCard;
    use super::*;

    fn baron_for_testing() -> Box<TestCard> {
        TestCard::boxed("Baron", 3)
    }
    fn princess_for_testing() -> Box<TestCard> {
        TestCard::boxed("Princess", 8)
    }

    #[test]
    fn test_name() {
        let name = "Dwigt";
        let player = Player::with_name(name);
        assert_eq!(name, player.name());
    }

    #[throws]
    #[test]
    fn test_add_card() {
        let b = baron_for_testing();
        let mut player = Player::with_name("Guilty Joe");
        player.add_card_to_hand(baron_for_testing());
        assert!(player.card_in_hand()?.is_same_card(b.as_ref()));
    }

    #[test]
    fn test_card_in_hand() {
        let mut player = Player::with_name("Merrywinkle");
        let baron = baron_for_testing();
        let princess = princess_for_testing();

        assert!(player.card_in_hand().is_err());

        player.add_card_to_hand(princess_for_testing());
        assert!(player.card_in_hand().is_ok());
        assert!(princess.is_same_card(player.card_in_hand().unwrap()));
        assert!(!baron.is_same_card(player.card_in_hand().unwrap()));

        player.add_card_to_hand(baron);
        assert!(player.card_in_hand().is_err());
    }

    #[throws]
    #[test]
    fn test_discard() {
        let mut player = Player::with_name("Bryce Wayne");

        player.add_card_to_hand(princess_for_testing());
        assert!(player
            .card_in_hand()?
            .is_same_card(princess_for_testing().as_ref()));
        assert!(player.discard(1).is_err());
        assert_eq!(1, player.hand.len());
        assert_eq!(0, player.discards.len());
        player.discard(0)?;

        assert_eq!(0, player.hand.len());
        assert_eq!(1, player.discards.len());

        player.add_card_to_hand(baron_for_testing());
        assert!(player
            .card_in_hand()?
            .is_same_card(baron_for_testing().as_ref()));
        player.discard(0);

        assert_eq!(0, player.hand.len());
        assert_eq!(2, player.discards.len());

        assert!(player
            .discards
            .iter()
            .find(|c| c.is_same_card(princess_for_testing().as_ref()))
            .is_some());
        assert!(player
            .discards
            .iter()
            .find(|c| c.is_same_card(baron_for_testing().as_ref()))
            .is_some());
    }
}
