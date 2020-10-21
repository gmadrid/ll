use crate::game::card_action::CardAction;
use crate::game::card_rules::CardRules;
use crate::messenger::Messenger;
use crate::state::Card;
use crate::{Error, Game};
use fehler::throws;

const NO_TARGET: CardRules = CardRules::new(false, false, false);
const TARGET_REQUIRED: CardRules = CardRules::new(true, false, false);
const GUESS_REQUIRED: CardRules = CardRules::new(true, false, true);
const TARGET_SELF_ALLOWED: CardRules = CardRules::new(true, true, false);

pub fn rules_for_card(card: Card) -> &'static CardRules {
    match card {
        Card::Guard => &GUESS_REQUIRED,
        Card::Priest => &TARGET_REQUIRED,
        Card::Baron => &TARGET_REQUIRED,
        Card::Handmaid => &NO_TARGET,
        Card::Prince => &TARGET_SELF_ALLOWED,
        Card::King => &TARGET_REQUIRED,
        Card::Countess => &NO_TARGET,
        Card::Princess => &NO_TARGET,
    }
}

#[throws]
pub fn perform_card_action(action: &CardAction, game: &mut Game, messenger: &mut impl Messenger) {
    match action.card() {
        Card::Guard => guard_action(action, game, messenger)?,
        Card::Priest => priest_action(action, game, messenger)?,
        Card::Baron => baron_action(action, game, messenger)?,
        Card::Handmaid => handmaid_action(action, game, messenger)?,
        Card::Prince => prince_action(action, game, messenger)?,
        Card::King => king_action(action, game, messenger)?,
        Card::Countess => countess_action(action, game, messenger)?,
        Card::Princess => princess_action(action, game, messenger)?,
    }
}

#[throws]
fn guard_action(action: &CardAction, game: &mut Game, messenger: &mut impl Messenger) {
    let target_index = action.target()?;
    let guess = action.guess()?;
    let target = game.player(target_index)?;

    messenger.to_all(&format!(
        "Player {} guesses that Player {} has a {}",
        action.current(),
        target_index,
        guess
    ));

    let target_card = target.card_in_hand()?;
    if guess == target_card {
        messenger.to_all(&format!(
            "Player {} has a {} and is out!",
            target_index, guess
        ));
        game.make_inactive(target_index)?;
    } else {
        messenger.to_all(&format!(
            "Player {} does not have a {}",
            target_index, guess
        ));
    }
}

#[throws]
fn priest_action(action: &CardAction, game: &mut Game, messenger: &mut impl Messenger) {
    let target_index = action.target()?;
    let target = game.player(target_index)?;

    send_card_played_with_target_message(action, messenger)?;

    messenger.to_all(&format!(
        "Player {} shows his card to Player {}",
        target_index,
        action.current()
    ));

    let target_card = target.card_in_hand()?;
    messenger.to_player(
        game.player(action.current())?,
        &format!("Player {} shows you a {}", target_index, target_card),
    );
}

#[throws]
fn send_card_played_with_target_message(action: &CardAction, messenger: &mut impl Messenger) {
    messenger.to_all(&format!(
        "Player {} plays a {} on Player {}",
        action.current(),
        action.card(),
        action.target()?
    ));
}

#[throws]
fn send_card_played_message(action: &CardAction, messenger: &mut impl Messenger) {
    messenger.to_all(&format!(
        "Player {} plays a {}",
        action.current(),
        action.card()
    ));
}

#[throws]
fn baron_action(action: &CardAction, game: &mut Game, messenger: &mut impl Messenger) {
    send_card_played_with_target_message(action, messenger)?;

    let current_index = action.current();
    let target_index = action.target()?;
    let current_player = game.player(current_index)?;
    let target_player = game.player(target_index)?;

    let player_card = current_player.card_in_hand()?;
    let target_card = target_player.card_in_hand()?;

    if player_card == target_card {
        messenger.to_all(&format!("Boingy, boingy, boingy. (The cards are equal.)"));
    } else {
        let (out_index, out_card) = if player_card > target_card {
            (target_index, target_card)
        } else {
            (current_index, player_card)
        };
        messenger.to_all(&format!(
            "Player {} showed a {} and is out.",
            out_index, out_card
        ));
        game.make_inactive(out_index)?;
    }
}

#[throws]
fn handmaid_action(action: &CardAction, game: &mut Game, messenger: &mut impl Messenger) {
    send_card_played_message(action, messenger)?;
    messenger.to_all(&format!("Player {} is safe.", action.current()));
    game.make_protected(action.current())?;
}

#[throws]
fn prince_action(action: &CardAction, game: &mut Game, messenger: &mut impl Messenger) {
    //    check_valid_target(action)?;
    send_card_played_with_target_message(action, messenger)?;

    let current_index = action.current();
    let target_index = action.target()?;
    let current_player = game.player(current_index)?;
    let target_player = game.player(target_index)?;

    let target_card = target_player.card_in_hand()?;
    messenger.to_all(&format!(
        "Player {} discards a {}.",
        target_index, target_card
    ));
    if let Card::Princess = target_card {
        messenger.to_all(&format!("Player {} is out!", target_index));
    }

    todo!()
}

#[throws]
fn king_action(action: &CardAction, game: &mut Game, messenger: &mut impl Messenger) {
    todo!()
}

#[throws]
fn countess_action(action: &CardAction, game: &mut Game, messenger: &mut impl Messenger) {
    todo!()
}

#[throws]
fn princess_action(action: &CardAction, game: &mut Game, messenger: &mut impl Messenger) {
    todo!()
}
