use objekt_clonable::clonable;

#[clonable]
/// Describes the physical card - basically what would be printed on the card.
pub trait Card: std::fmt::Debug + std::fmt::Display + std::clone::Clone {
    /// The name printed on the card.
    fn name(&self) -> &str;

    /// Every Love Letter card has a numeric value.
    fn value(&self) -> u8;

    /// Compares two Cards to see if they are *exactly* the same.
    ///
    /// This means that the name and values are equal.
    /// (In some games, there may be different cards with the same value.
    /// To check for that condition, use `has_same_value()`.
    fn is_same_card(&self, other: &dyn Card) -> bool {
	self.name() == other.name() &&
	    self.value() == other.value()
    }

    /// Compares the `value` of two Cards.
    fn has_same_value(&self, other: &dyn Card) -> bool {
	self.value() == other.value()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::state::testcard::TestCard;

    #[test]
    fn test_name() {
        let frank = TestCard::new("Frank", 1);
        let sadie = TestCard::new("Sadie", 2);

        assert_eq!("Frank", frank.name());
        assert_eq!("Sadie", sadie.name());
    }

    #[test]
    fn test_clone() {
	let nigel = TestCard::boxed("Nigel", 3);
	let junior = nigel.clone();

	assert!(nigel.is_same_card(junior.as_ref()));
    }
}
