use crate::game::card_action::CardAction;
use crate::Error;
use fehler::{throw, throws};
use std::collections::HashSet;

#[derive(Debug)]
pub struct CardRules {
    target_required: bool,
    current_allowed_as_target: bool,
    guess_required: bool,
}

impl CardRules {
    pub const fn new(
        target_required: bool,
        current_allowed_as_target: bool,
        guess_required: bool,
    ) -> CardRules {
        CardRules {
            target_required,
            current_allowed_as_target,
            guess_required,
        }
    }

    #[throws]
    pub fn action_allowed(
        &self,
        action: &CardAction,
        current_player: usize,
        active: &HashSet<usize>,
        protected: &HashSet<usize>,
    ) {
        CardRules::player_is_current(action, current_player)?;
        self.target_is_valid(action, current_player, active, protected)?;
        self.guess_is_valid(action)?;
    }

    #[throws]
    fn guess_is_valid(&self, action: &CardAction) {
        if self.guess_required {
            action.guess()?;
        }
    }

    #[throws]
    fn target_is_valid(
        &self,
        action: &CardAction,
        current_player: usize,
        active: &HashSet<usize>,
        protected: &HashSet<usize>,
    ) {
        if self.target_required {
            let target = action.target()?;
            if !active.contains(&target) {
                throw!(Error::BadActionTargetingInactive(target));
            }
            if protected.contains(&target) {
                throw!(Error::BadActionTargetingProtected(target));
            }
            if !self.current_allowed_as_target && current_player == target {
                throw!(Error::BadActionCannotTargetSelf);
            }
        }
    }

    #[throws]
    fn player_is_current(action: &CardAction, current_player: usize) {
        if current_player != action.current() {
            throw!(Error::BadActionNotCurrentPlayer(action.current()));
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::state::Card;
    use std::iter::FromIterator;

    fn basic_rules() -> CardRules {
        CardRules {
            guess_required: false,
            current_allowed_as_target: false,
            target_required: false,
        }
    }

    fn target_required() -> CardRules {
        CardRules {
            target_required: true,
            ..basic_rules()
        }
    }

    fn current_allowed() -> CardRules {
        CardRules {
            current_allowed_as_target: true,
            ..target_required()
        }
    }

    fn guess_required() -> CardRules {
        CardRules {
            guess_required: true,
            ..basic_rules()
        }
    }

    macro_rules! valid_action {
        (($card:expr, $current:expr, $target:expr, $guess:expr), $rules:expr, $actual_current:expr, $active:expr, $protected:expr) => {
            assert_eq!(
                $rules.action_allowed(
                    &CardAction::new($card, $current, $target, $guess),
                    $actual_current,
                    &HashSet::from_iter((&$active).iter().cloned()),
                    &HashSet::from_iter((&$protected).iter().cloned()),
                )?,
                ()
            );
        };
    }

    macro_rules! invalid_action {
        (($card:expr, $current:expr, $target:expr, $guess:expr), $err:expr, $rules:expr, $actual_current:expr, $active:expr, $protected:expr) => {
            assert_eq!(
                $rules
                    .action_allowed(
                        &CardAction::new($card, $current, $target, $guess),
                        $actual_current,
                        &HashSet::from_iter((&$active).iter().cloned()),
                        &HashSet::from_iter((&$protected).iter().cloned()),
                    )
                    .err(),
                Some($err)
            )
        };
    }

    #[test]
    #[throws]
    fn test_basic() {
        let rules = basic_rules();

        valid_action!((Card::Princess, 0, None, None), rules, 0, [0, 1, 2, 3], []);

        invalid_action!(
            (Card::Princess, 1, None, None),
            Error::BadActionNotCurrentPlayer(1),
            rules,
            0,
            [0, 1, 2, 3],
            []
        );
    }

    #[test]
    #[throws]
    fn test_targets() {
        let rules = target_required();

        valid_action!((Card::Baron, 0, Some(1), None), rules, 0, [0, 1, 2, 3], []);
        valid_action!(
            (Card::Baron, 0, Some(1), None),
            rules,
            0,
            [0, 1, 2, 3],
            [0, 2, 3]
        );

        invalid_action!(
            (Card::Baron, 0, Some(1), None),
            Error::BadActionNotCurrentPlayer(0),
            rules,
            3,
            [0, 1, 2, 3],
            []
        );
        invalid_action!(
            (Card::Baron, 0, Some(1), None),
            Error::BadActionTargetingInactive(1),
            rules,
            0,
            [0, 2, 3],
            []
        );
        invalid_action!(
            (Card::Baron, 0, Some(1), None),
            Error::BadActionTargetingInactive(1),
            rules,
            0,
            [0, 2, 3],
            []
        );
        invalid_action!(
            (Card::Baron, 0, Some(1), None),
            Error::BadActionTargetingProtected(1),
            rules,
            0,
            [0, 1, 2, 3],
            [1]
        );
        invalid_action!(
            (Card::Baron, 0, Some(0), None),
            Error::BadActionCannotTargetSelf,
            rules,
            0,
            [0, 1, 2, 3],
            [1]
        );
	valid_action!(
	    (Card::Baron, 0, Some(2), None),
	    rules,
	    0,
	    [0, 1, 2, 3],
	    [1]
	);
    }

    #[throws]
    #[test]
    fn target_self() {
        let rules = current_allowed();

        valid_action!((Card::Prince, 0, Some(0), None), rules, 0, [0, 1, 2, 3], []);
    }

    #[throws]
    #[test]
    fn test_guess_required() {
        let rules = guess_required();

        valid_action!(
            (Card::Guard, 0, Some(1), Some(Card::Baron)),
            rules,
            0,
            [0, 1, 2, 3],
            []
        );
        invalid_action!(
            (Card::Guard, 0, Some(1), None),
            Error::BadActionMissingGuess,
            rules,
            0,
            [0, 1, 2, 3],
            []
        );
    }
}
