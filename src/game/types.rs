//! # Types
//!
//! This module exposes different kind of types for `Game`

use crate::{Color, Move, Piece, Position, Promotion};

use core::time::Duration;

// -- results

/// ## GameResult
///
/// Describes the result of a game
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GameResult {
    /// The game continues without any problem
    Continuing,
    /// The game has ended; match the `EndGame` variant, to get the end game result
    Ended(EndGame),
    /// An illegal move was made. This can include many things,
    /// such as moving a piece through another piece, attempting
    /// to capture an allied piece, moving non-orthogonally or
    /// non-diagonally, or non-knight-like according the rules
    /// governing the movement of the piece. Additionally,
    /// moves that put the player in check, (for example, moving a pinned piece),
    /// are also illegal.
    IllegalMove(Move),
}

/// ## EndGame
///
/// Describes the kind of end game
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EndGame {
    /// One player, the victor, checkmated the other.
    /// This stores the color of the winner and the reason
    Victory(Color, VictoryReason),
    /// The game is draw. There are 3 conditions where this can happen:
    ///
    /// 1. The current player has no legal moves and not being in check
    /// 2. both players have insufficient material on the board.
    ///     Insufficient material consists of:
    ///
    ///     1. The player only has a king
    ///     2. The player only has a king and a knight
    ///     3. The player only has a king and two knights
    ///     4. The player only has a king and a bishop
    ///     5. The player only has a king and two bishops
    ///
    /// 3. Threefold repetition. The same moves are played for 3 turns
    Draw,
}

/// ## VictoryReason
///
/// Describes the reason that brought the player to victory
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VictoryReason {
    Checkmate,
    Resign,
    Timeout,
}

/// ## GameEvent
///
/// Describes an event "raised" after a move is played
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GameEvent {
    /// A pawn promotion is available
    /// a `Promotion` must be performed via `promote()` method
    Promotion(Position),
    /// A threefold repetition has been detected.
    ThreefoldRepetition,
    /// A fivefold repetition has been detected.
    FivefoldRepetition,
}

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
