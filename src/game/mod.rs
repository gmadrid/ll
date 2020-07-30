use crate::state::{Player, Table};
use crate::Error;
use fehler::throws;
use std::fmt::{Display, Formatter};

pub struct GameBuilder {
    num_players: usize,
}

impl GameBuilder {
    pub fn new() -> GameBuilder {
        GameBuilder { num_players: 4 }
    }

    #[throws]
    pub fn build(self) -> Game {
        Game::new(self.num_players)?
    }

    pub fn num_players(mut self, num_players: usize) -> GameBuilder {
        self.num_players = num_players;
        self
    }
}

#[derive(Debug)]
pub struct Game {
    table: Table,
}

impl Game {
    #[throws]
    fn new(num_players: usize) -> Game {
        let mut game = Game {
            table: Table::new(num_players)?,
        };

        for player_num in 0..num_players {
            game.deal_one_to_player(player_num)?;
        }

        game
    }

    fn is_deck_empty(&self) -> bool {
        self.table.deck().cards_remaining() == 0
    }

    #[throws]
    fn deal_one_to_player(&mut self, player_num: usize) {
        if let Some(card) = self.table.deck_mut().deal_one() {
            let player = self.table.player_mut(player_num)?;
            player.add_card_to_hand(card);
        }
    }

    #[throws]
    fn player(&self, player_num: usize) -> &Player {
        self.table.player(player_num)?
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.table)
    }
}
