//! # Player
//!
//! Exposes the player data for a chess game

use alloc::string::{String, ToString};

use super::Country;

/// ## Player
///
/// Player's data
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Player {
    name: String,
    surname: String,
    nationality: Country,
    elo: u16,
}

impl Player {
    /// ### new
    ///
    /// Create a new `Player`
    pub fn new<S: AsRef<str>>(name: S, surname: S, nationality: Country, elo: u16) -> Self {
        Self {
            name: name.as_ref().to_string(),
            surname: surname.as_ref().to_string(),
            nationality,
            elo,
        }
    }

    /// ### name
    ///
    /// Get reference to name
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    /// ### surname
    ///
    /// Get reference to surname
    pub fn surname(&self) -> &str {
        self.surname.as_str()
    }

    /// ### nationality
    ///
    /// Get player's nationality
    pub fn nationality(&self) -> Country {
        self.nationality
    }

    /// ### elo
    ///
    /// Get player's elo
    pub fn elo(&self) -> u16 {
        self.elo
    }
}

#[cfg(test)]
mod test {

    use super::*;

    use pretty_assertions::assert_eq;

    #[test]
    fn player() {
        let player: Player = Player::new("magnus", "carlsen", Country::Norway, 2882);
        assert_eq!(player.elo(), 2882);
        assert_eq!(player.name(), "magnus");
        assert_eq!(player.nationality(), Country::Norway);
        assert_eq!(player.surname(), "carlsen");
    }
}
