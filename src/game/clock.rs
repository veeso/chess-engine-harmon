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
    pub fn add_time(&mut self, color: Color, amount: Duration) {
        match color {
            Color::Black => self.black += amount,
            Color::White => self.white += amount,
        };
    }

    /// ### sub_time
    ///
    /// Subtract time from player's clock
    pub fn sub_time(&mut self, color: Color, amount: Duration) {
        match color {
            Color::Black => self.black -= amount,
            Color::White => self.white -= amount,
        };
    }

    /// ### set_time
    ///
    /// Set new remaining time for player's clock
    pub fn set_time(&mut self, color: Color, time: Duration) {
        match color {
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
}

#[cfg(test)]
mod test {
    use super::*;

    use pretty_assertions::assert_eq;

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
    fn add_and_subtract_and_set_time() {
        let mut clock: Clock = Clock::new(Duration::from_secs(300), Duration::from_secs(300));
        // sub time
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
        // add time
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
        // set time
        clock.set_time(Color::White, Duration::from_secs(60));
        assert_eq!(
            clock.remaining_time(),
            (Duration::from_secs(60), Duration::from_secs(293))
        );
        clock.set_time(Color::Black, Duration::from_secs(90));
        assert_eq!(
            clock.remaining_time(),
            (Duration::from_secs(60), Duration::from_secs(90))
        );
    }
}
