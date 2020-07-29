use std::fmt::{Display, Formatter};

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
pub enum Card {
    Guard,
    Priest,
    Baron,
    Handmaid,
    Prince,
    King,
    Countess,
    Princess,
}

impl Display for Card {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use Card::*;
        let output = match self {
            Guard => "Guard",
            Priest => "Priest",
            Baron => "Baron",
            Handmaid => "Handmaid",
            Prince => "Prince",
            King => "King",
            Countess => "Countess",
            Princess => "Princess",
        };
        write!(f, "{}", output)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn test_order() {
        // The cards have a specific order defined by the game rules.
        use Card::*;
        assert!(Guard < Priest);
        assert!(Priest < Baron);
        assert!(Baron < Handmaid);
        assert!(Handmaid < Prince);
        assert!(Prince < King);
        assert!(King < Countess);
        assert!(Countess < Princess);
    }
}
