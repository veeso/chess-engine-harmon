//! # Square
//!
//! This module exposes the type to define an individual square of the chess board

use super::Piece;

/// ## Square
///
/// Essentially a container for a single piece on a board.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Square {
    piece: Option<Piece>,
}

impl From<Piece> for Square {
    fn from(piece: Piece) -> Self {
        Self { piece: Some(piece) }
    }
}

impl Square {
    /// ### empty
    ///
    /// A constructor for an empty Square
    pub fn empty() -> Self {
        Square { piece: None }
    }

    /// ### is_empty
    ///
    /// Does this square contain a piece?
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.piece == None
    }

    /// ### get_piece
    ///
    /// Get the piece contained in this square.
    #[inline]
    pub fn get_piece(&self) -> Option<Piece> {
        self.piece
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use crate::position::C8;
    use crate::WHITE;

    use pretty_assertions::assert_eq;

    #[test]
    fn square() {
        // empty
        let square: Square = Square::empty();
        assert_eq!(square.is_empty(), true);
        assert_eq!(square.get_piece(), None);
        // from
        let square: Square = Square::from(Piece::Queen(WHITE, C8));
        assert_eq!(square.is_empty(), false);
        assert_eq!(square.get_piece().unwrap().is_queen(), true);
    }
}
