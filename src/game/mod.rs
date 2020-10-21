mod original;
mod card_action;
mod card_rules;

pub use crate::game::card_action::CardAction;
use crate::messenger::Messenger;
use crate::state::{Player, Table};
use crate::Error;
use fehler::{throw, throws};
use std::collections::HashSet;
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

impl Default for GameBuilder {
    fn default() -> Self {
        GameBuilder::new()
    }
}

#[derive(Debug)]
pub struct Game {
    table: Table,

    current_player: usize,
    active: HashSet<usize>,
    protected: HashSet<usize>,
}

impl Game {
    #[throws]
    fn new(num_players: usize) -> Game {
        let mut game = Game {
            table: Table::new(num_players)?,
            current_player: 0,
            active: HashSet::default(),
            protected: HashSet::default(),
        };

        for player_num in 0..num_players {
            game.active.insert(player_num);
            game.deal_one_to_player(player_num)?;
        }

	if let Some(out_card) = game.table.deck_mut().deal_one() {
	    game.table.set_out_card(Some(out_card));
	} else {
	    throw!(Error::InternalErrorUnexpectedEmptyDeck);
	}

        game
    }

    #[throws]
    pub fn perform_action(&mut self, action: CardAction, messenger: &mut impl Messenger) {
        self.is_valid_action(&action)?;

        messenger.message(
            None,
            &format!("Player {} discards a {}", action.current(), action.card()),
        );
        let current = self.player_mut(self.current_player)?;
        current.discard(action.card())?;

        original::perform_card_action(&action, self, messenger)?;

        self.make_next_player_current();
    }

    #[throws]
    fn make_inactive(&mut self, player_index: usize) {
        todo!("");
    }

    #[throws]
    fn make_protected(&mut self, player_index: usize) {
        todo!("");
    }

    #[throws]
    fn make_unprotected(&mut self, player_index: usize) {
	todo!("");
    }

    fn make_next_player_current(&mut self) {
        todo!("");
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

    #[throws]
    fn player_mut(&mut self, player_num: usize) -> &mut Player {
        self.table.player_mut(player_num)?
    }

    #[throws]
    fn is_valid_action(&self, action: &CardAction) {
        let player = self.player(action.current())?;
        let card = action.card();
        if player.card_index(card).is_none() {
            throw!(Error::BadActionPlayerDoesntHaveCard(action.current(), card));
        }

        let rules = original::rules_for_card(card);
        rules.action_allowed(&action, self.current_player, &self.active, &self.protected)?;
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.table)
    }
}
