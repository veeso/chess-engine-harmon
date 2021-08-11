//! # Types
//!
//! This module exposes different kind of types for `Game`

use crate::{Color, Move, Piece, Promotion};

use core::time::Duration;

// -- moves

/// ## GameMove
///
/// A game move.
/// In addition to the simple `Move` type, this struct also tracks the time taken to perform the move
/// and eventually the piece taken from the opponent and the eventual pawn promotion.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct GameMove {
    /// The move itself
    pub itself: Move,
    /// the turn number (1..65535)
    pub turn: u16,
    /// Player color
    pub player: Color,
    /// the time taken to think the move
    pub time: Duration,
    /// the piece taken from the opponent
    pub piece_taken: Option<Piece>,
    /// the eventual pawn promotion performed on that turn
    pub promotion: Option<Promotion>,
}

impl GameMove {
    /// ### new
    ///
    /// Instantiates a new GameMove
    pub fn new(
        m: Move,
        player: Color,
        turn: u16,
        time: Duration,
        piece_taken: Option<Piece>,
        promotion: Option<Promotion>,
    ) -> Self {
        Self {
            itself: m,
            player,
            turn,
            time,
            piece_taken,
            promotion,
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn game_move() {
        let m: GameMove = GameMove {
            itself: Move::Resign,
            turn: 2,
            player: Color::White,
            time: Duration::from_secs(5),
            piece_taken: None,
            promotion: None,
        };
        assert_eq!(m.itself, Move::Resign);
        assert_eq!(m.turn, 2);
        assert_eq!(m.player, Color::White);
        assert_eq!(m.time, Duration::from_secs(5));
        assert_eq!(m.piece_taken, None);
        assert_eq!(m.promotion, None);
        let m: GameMove = GameMove::new(
            Move::Resign,
            Color::Black,
            2,
            Duration::from_secs(5),
            None,
            None,
        );
        assert_eq!(m.itself, Move::Resign);
        assert_eq!(m.turn, 2);
        assert_eq!(m.player, Color::Black);
        assert_eq!(m.time, Duration::from_secs(5));
        assert_eq!(m.piece_taken, None);
        assert_eq!(m.promotion, None);
    }
}
