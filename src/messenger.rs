use crate::state::Player;

pub trait Messenger {
    fn message(&mut self, player: Option<&Player>, msg: &str);

    fn to_all(&mut self, msg: &str) {
        self.message(None, msg);
    }

    fn to_player(&mut self, player: &Player, msg: &str) {
        self.message(Some(player), msg);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::Error;
    use fehler::throws;

    struct TestMessenger {
        pub messages: Vec<String>,
    }

    impl TestMessenger {
        fn new() -> TestMessenger {
            TestMessenger { messages: vec![] }
        }

        fn player_name<'a>(&self, player: Option<&'a Player>) -> &'a str {
            match player {
                None => "All",
                Some(player) => player.name(),
            }
        }
    }

    impl Messenger for TestMessenger {
        fn message(&mut self, player: Option<&Player>, msg: &str) {
            let saved = format!("{}: {}", self.player_name(player), msg);
            self.messages.push(saved);
        }
    }

    #[test]
    #[throws]
    fn base_test() {
        let mut messenger = TestMessenger::new();

        messenger.to_all("foobar");
        messenger.to_player(&Player::with_name("Henry"), "quux");

        assert_eq!(
            messenger.messages,
            vec!["All: foobar", "Henry: quux"]
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
        );
    }
}
