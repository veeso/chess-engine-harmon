//! # CastlingRights
//!
//! This module exposes the castling rights type

/// ### CastlingRights
///
/// Defines the castling rights for the game
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct CastlingRights {
    kingside: bool,
    queenside: bool,
}

impl Default for CastlingRights {
    fn default() -> Self {
        Self {
            kingside: true,
            queenside: true,
        }
    }
}

impl CastlingRights {
    /// ### can_kingside_castle
    ///
    /// returns whether kingside castle is enabled
    pub fn can_kingside_castle(&self) -> bool {
        self.kingside
    }

    /// ### can_queenside_castle
    ///
    /// returns whether queenside castle is enabled
    pub fn can_queenside_castle(&self) -> bool {
        self.queenside
    }

    /// ### disable_kingside
    ///
    /// disable kingside castling rights
    pub fn disable_kingside(&mut self) {
        self.kingside = false
    }

    /// ### disable_queenside
    ///
    /// disable queenside castling rights
    pub fn disable_queenside(&mut self) {
        self.queenside = false
    }

    /// ### disable_all
    ///
    /// disable both kingside and queenside castling rights
    pub fn disable_all(&mut self) {
        self.disable_kingside();
        self.disable_queenside()
    }

    /// ### enable_kingside
    ///
    /// enable kingside castling rights
    pub fn enable_kingside(&mut self) {
        self.kingside = true
    }

    /// ### enable_queenside
    ///
    /// enable queenside castling rights
    pub fn enable_queenside(&mut self) {
        self.queenside = true
    }

    /// ### enable_all
    ///
    /// enable both kingside and queenside castling rights
    pub fn enable_all(&mut self) {
        self.enable_kingside();
        self.enable_queenside()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use pretty_assertions::assert_eq;

    #[test]
    fn castling_rights() {
        let mut rights: CastlingRights = CastlingRights::default();
        assert_eq!(rights.can_kingside_castle(), true);
        assert_eq!(rights.can_queenside_castle(), true);
        rights.disable_kingside();
        assert_eq!(rights.can_kingside_castle(), false);
        assert_eq!(rights.can_queenside_castle(), true);
        rights.enable_kingside();
        assert_eq!(rights.can_kingside_castle(), true);
        assert_eq!(rights.can_queenside_castle(), true);
        rights.disable_queenside();
        assert_eq!(rights.can_kingside_castle(), true);
        assert_eq!(rights.can_queenside_castle(), false);
        rights.enable_queenside();
        assert_eq!(rights.can_kingside_castle(), true);
        assert_eq!(rights.can_queenside_castle(), true);
        rights.disable_all();
        assert_eq!(rights.can_kingside_castle(), false);
        assert_eq!(rights.can_queenside_castle(), false);
        rights.enable_all();
        assert_eq!(rights.can_kingside_castle(), true);
        assert_eq!(rights.can_queenside_castle(), true);
    }
}
