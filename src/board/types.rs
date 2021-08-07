//! # Types
//!
//! This module exposes different kind of types for `Board`

use super::{Board, Color, Move, Position};

/// ## RatedMove
///
/// A tuple made up of the Move and its score
pub type RatedMove = (Move, f64);

/// ## Promotion
///
/// Defines the kind of promotion to perform whenever a pawn reaches the last line
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Promotion {
    Queen,
    Knight,
    Bishop,
    Rook,
}

/// ## MoveResult
///
/// Describes the result of a move being played on the board.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum MoveResult {
    /// The game is not finished, and the game is still in play.
    Continuing(Board),
    /// The game is not finished, and is still in play.
    /// In addition to this a `Promotion` must be performed via `promote()` method
    Promote(Board, Position),
    /// One player, the victor, checkmated the other.
    /// This stores the color of the winner.
    Victory(Color),
    /// The game is drawn. There are 3 conditions where this can happen:
    ///
    /// 1. The current player has no legal moves and not being in check
    /// 2. both players have insufficient material on the board.
    ///
    ///     Insufficient material consists of:
    ///
    ///     1. The player only has a king
    ///     2. The player only has a king and a knight
    ///     3. The player only has a king and two knights
    ///     4. The player only has a king and a bishop
    ///     5. The player only has a king and two bishops
    ///
    /// 3. Threefold repetition. The same moves are played for 3 turns.
    ///     NOTE: this cannot be handled by `Board`. Only `Game` handles this
    Stalemate,
    /// An illegal move was made. This can include many things,
    /// such as moving a piece through another piece, attempting
    /// to capture an allied piece, moving non-orthogonally or
    /// non-diagonally, or non-knight-like according the rules
    /// governing the movement of the piece. Additionally,
    /// moves that put the player in check, (for example, moving a pinned piece),
    /// are also illegal.
    IllegalMove(Move),
}
