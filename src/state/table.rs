use crate::state::deck::Deck;
use crate::state::player::Player;
use crate::Error;
use fehler::{throw, throws};
use std::fmt::{Display, Formatter};

#[derive(Debug, Default)]
pub struct Table {
    players: Vec<Player>,
    deck: Deck,
}

impl Table {
    #[throws]
    pub fn new(num_players: usize) -> Table {
        if num_players != 3 && num_players != 4 {
            throw!(Error::InvalidNumberOfPlayers(num_players));
        }
        let mut players = Vec::new();
        players.resize_with(num_players, Default::default);
        Table {
            players,
            deck: Deck::new_shuffled(),
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
