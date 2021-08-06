//! # Game
//!
//! The game module is the highest level library api. This module exposes the `Game` type which, along to the board, contains also
//! the metadata of a match and the played moves. In addition to this, it also adds more sophisticated game logics to complete
//! the chess game (e.g. the threefold repetition stallmate).
//!

use super::{Board, Color, Move};

/// ## GameResult
///
/// Describes the result of a move being played on the board.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum GameResult {
    // FIXME: resolve difference between GameResult and MoveResult
    /// The game is not finished, and the game is still in play.
    Continuing(Board),
    /// One player, the victor, checkmated the other.
    /// This stores the color of the winner.
    Victory(Color),
    /// The game is drawn. This can be a result of the current player
    /// having no legal moves and not being in check, or because
    /// both players have insufficient material on the board.
    ///
    /// Insufficient material consists of:
    /// 1. The player only has a king
    /// 2. The player only has a king and a knight
    /// 3. The player only has a king and two knights
    /// 4. The player only has a king and a bishop
    /// 5. The player only has a king and two bishops
    ///
    /// In a regular game of chess, threefold repetition also triggers
    /// a stalemate, but this engine does not have builtin support for
    /// threefold repetition detection yet. TODO: add threefold repetition
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

// TODO: game
// TODO: player
