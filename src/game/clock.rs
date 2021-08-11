//! # Clock
//!
//! This module exposes a chess clock in order to track remaining time for each player

use core::time::Duration;

use crate::Color;

/// ## Clock
///
/// chess clock which tracks remaining time for each player
#[derive(Debug, Clone)]
pub struct Clock {
    /// Remaining time for black player, expressed as `Duration`
    black: Duration,
    /// Remaining time for white player, expressed as `Duration`
    white: Duration,
}

impl Clock {
    /// ### new
    ///
    /// Instantiates a new `Clock' providing remaining times for both players
    pub fn new(white: Duration, black: Duration) -> Self {
        Self { black, white }
    }

    /// ### add_time
    ///
    /// Add time to player's clock
    pub fn add_time(&mut self, player: Color, amount: Duration) {
        match player {
            Color::Black => self.black += amount,
            Color::White => self.white += amount,
        };
    }

    /// ### sub_time
    ///
    /// Subtract time from player's clock
    pub fn sub_time(&mut self, player: Color, amount: Duration) {
        match player {
            Color::Black => self.black = self.black.checked_sub(amount).unwrap_or(Duration::ZERO),
            Color::White => self.white = self.white.checked_sub(amount).unwrap_or(Duration::ZERO),
        };
    }

    /// ### set_time
    ///
    /// Set new remaining time for player's clock
    pub fn set_time(&mut self, player: Color, time: Duration) {
        match player {
            Color::Black => self.black = time,
            Color::White => self.white = time,
        };
    }

    /// ### remaining_time
    ///
    /// Returns remaining time on clock.
    /// The first element of the tuple is remaining time for white player,
    /// while the second element is remaining time for black player
    pub fn remaining_time(&self) -> (Duration, Duration) {
        (self.white, self.black)
    }

    /// ### player_remaining_time
    ///
    /// Returns remaining time for player
    pub fn player_remaining_time(&self, player: Color) -> Duration {
        match player {
            Color::Black => self.black,
            Color::White => self.white,
        }
    }

    /// ### timeout
    ///
    /// Returns whether player's time is zero
    pub fn timeout(&self, player: Color) -> bool {
        self.player_remaining_time(player).is_zero()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use pretty_assertions::assert_eq;

    #[test]
    fn clock_struct() {
        let clock: Clock = Clock {
            black: Duration::from_secs(3600),
            white: Duration::from_secs(1800),
        };
        assert_eq!(clock.black, Duration::from_secs(3600));
        assert_eq!(clock.white, Duration::from_secs(1800));
    }

    #[test]
    fn new_clock() {
        let clock: Clock = Clock::new(Duration::from_secs(3600), Duration::from_secs(1800));
        assert_eq!(clock.white, Duration::from_secs(3600));
        assert_eq!(clock.black, Duration::from_secs(1800));
    }

    #[test]
    fn remaining_time() {
        let clock: Clock = Clock::new(Duration::from_secs(3600), Duration::from_secs(1800));
        assert_eq!(
            clock.remaining_time(),
            (Duration::from_secs(3600), Duration::from_secs(1800))
        );
    }

    #[test]
    fn player_remaining_time() {
        let clock: Clock = Clock::new(Duration::from_secs(3600), Duration::from_secs(1800));
        assert_eq!(
            clock.player_remaining_time(Color::Black),
            (Duration::from_secs(1800))
        );
        assert_eq!(
            clock.player_remaining_time(Color::White),
            (Duration::from_secs(3600))
        );
    }

    #[test]
    fn add_time() {
        let mut clock: Clock = Clock::new(Duration::from_secs(295), Duration::from_secs(290));
        clock.add_time(Color::White, Duration::from_secs(2));
        assert_eq!(
            clock.remaining_time(),
            (Duration::from_secs(297), Duration::from_secs(290))
        );
        clock.add_time(Color::Black, Duration::from_secs(3));
        assert_eq!(
            clock.remaining_time(),
            (Duration::from_secs(297), Duration::from_secs(293))
        );
        // Checked sub (underflow)
        let mut clock: Clock = Clock::new(Duration::from_secs(10), Duration::from_secs(23));
        clock.sub_time(Color::White, Duration::from_secs(15));
        assert_eq!(
            clock.remaining_time(),
            (Duration::from_secs(0), Duration::from_secs(23))
        );
    }

    #[test]
    fn sub_time() {
        let mut clock: Clock = Clock::new(Duration::from_secs(300), Duration::from_secs(300));
        clock.sub_time(Color::White, Duration::from_secs(5));
        assert_eq!(
            clock.remaining_time(),
            (Duration::from_secs(295), Duration::from_secs(300))
        );
        clock.sub_time(Color::Black, Duration::from_secs(10));
        assert_eq!(
            clock.remaining_time(),
            (Duration::from_secs(295), Duration::from_secs(290))
        );
    }

    #[test]
    fn set_time() {
        let mut clock: Clock = Clock::new(Duration::from_secs(300), Duration::from_secs(300));
        clock.set_time(Color::White, Duration::from_secs(60));
        assert_eq!(
            clock.remaining_time(),
            (Duration::from_secs(60), Duration::from_secs(300))
        );
        clock.set_time(Color::Black, Duration::from_secs(90));
        assert_eq!(
            clock.remaining_time(),
            (Duration::from_secs(60), Duration::from_secs(90))
        );
    }

    #[test]
    fn timeout() {
        let clock: Clock = Clock::new(Duration::from_secs(0), Duration::from_secs(300));
        assert_eq!(clock.timeout(Color::White), true);
        assert_eq!(clock.timeout(Color::Black), false);
        let clock: Clock = Clock::new(Duration::from_secs(60), Duration::from_secs(0));
        assert_eq!(clock.timeout(Color::White), false);
        assert_eq!(clock.timeout(Color::Black), true);
    }
}
