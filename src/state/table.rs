use crate::state::card::Card;
use crate::state::deck::Deck;
use crate::state::player::Player;
use crate::Error;
use fehler::{throw, throws};
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct Table {
    players: Vec<Player>,
    deck: Deck,
    out_card: Option<Box<dyn Card>>,
}

impl Table {
    #[throws]
    pub fn new(num_players: usize) -> Table {
        if num_players != 3 && num_players != 4 {
            throw!(Error::InvalidNumberOfPlayers(num_players));
        }
        let mut players = Vec::new();
        players.push(Player::with_name("1"));
        players.push(Player::with_name("2"));
        players.push(Player::with_name("3"));
        if num_players == 4 {
            players.push(Player::with_name("4"));
        }
        Table {
            players,
            deck: Deck::new(Default::default()),
	    out_card: None,
        }
    }

    pub fn deck(&self) -> &Deck {
        &self.deck
    }

    pub fn deck_mut(&mut self) -> &mut Deck {
        &mut self.deck
    }

    pub fn num_players(&self) -> usize {
        self.players.len()
    }

    #[throws]
    pub fn player(&self, player_num: usize) -> &Player {
        if player_num >= self.players.len() {
            throw!(Error::InvalidPlayerNumber(player_num));
        }
        &self.players[player_num]
    }

    #[throws]
    pub fn player_mut(&mut self, player_num: usize) -> &mut Player {
        if player_num >= self.players.len() {
            throw!(Error::InvalidPlayerNumber(player_num));
        }
        &mut self.players[player_num]
    }

    pub fn out_card(&self) -> Option<&dyn Card> {
	self.out_card.as_ref().map(|bc| bc.as_ref())
    }

    pub fn set_out_card(&mut self, card: Option<Box<dyn Card>>) {
	self.out_card = card;
    }
}

impl Display for Table {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (num, player) in self.players.iter().enumerate() {
            writeln!(f, "Player {}", num)?;
            writeln!(f, "{}", player)?;
        }
        write!(f, "Deck: {}", self.deck)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use fehler::throws;

    #[test]
    #[throws]
    fn test_three_players() {
        let mut table = Table::new(3)?;
        assert_eq!("1", table.player(0)?.name());
        assert_eq!("2", table.player(1)?.name());
        assert_eq!("3", table.player(2)?.name());
        assert!(table.player(3).is_err());
    }

    #[test]
    #[throws]
    fn test_four_players() {
        let mut table = Table::new(4)?;
        assert_eq!("1", table.player(0)?.name());
        assert_eq!("2", table.player(1)?.name());
        assert_eq!("3", table.player(2)?.name());
        assert_eq!("4", table.player(3)?.name());
        assert!(table.player(4).is_err());
    }
}
