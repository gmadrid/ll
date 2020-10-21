use super::card::Card;

#[derive(Clone, Debug)]
pub struct TestCard(&'static str, u8);

impl TestCard {
    pub fn boxed(name: &'static str, value: u8) -> Box<TestCard> {
        Box::new(TestCard::new(name, value))
    }

    pub fn new(name: &'static str, value: u8) -> TestCard {
        TestCard(name, value)
    }
}

impl Card for TestCard {
    fn name(&self) -> &str {
        self.0
    }

    fn value(&self) -> u8 {
        self.1
    }
}

impl std::fmt::Display for TestCard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TestCard: {}:{}", self.value(), self.name())
    }
}
