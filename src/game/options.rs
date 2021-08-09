//! # Options
//!
//! Game options

#[derive(Debug, Clone)]
pub struct Options {
    /// If enabled, the game is automatically terminated when the same three turns are played in a row
    /// If you want to follow the FIDE rules, where the user can claim for it (so it's not automatic), then
    /// you must disable this option and call `Game.is_threefold_repetition()` to check it manually.
    /// Default: true
    pub threefold_repetition: bool,
    /// If enabled, the game is automatically terminated when the same five turns are played in a row
    /// Default: true
    pub fivefold_repetition: bool,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            threefold_repetition: true,
            fivefold_repetition: true,
        }
    }
}

impl Options {
    /// ### threefold_repetition
    ///
    /// Set threefold repetition option
    pub fn threefold_repetition(mut self, enabled: bool) -> Self {
        self.threefold_repetition = enabled;
        self
    }

    /// ### fivefold_repetition
    ///
    /// Set fivefold repetition option
    pub fn fivefold_repetition(mut self, enabled: bool) -> Self {
        self.fivefold_repetition = enabled;
        self
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn options_default() {
        let options: Options = Options::default();
        assert_eq!(options.fivefold_repetition, true);
        assert_eq!(options.threefold_repetition, true);
    }

    #[test]
    fn options_builder() {
        let options: Options = Options::default()
            .fivefold_repetition(false)
            .threefold_repetition(false);
        assert_eq!(options.fivefold_repetition, false);
        assert_eq!(options.threefold_repetition, false);
    }
}
