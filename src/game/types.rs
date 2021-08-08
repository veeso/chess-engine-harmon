//! # Types
//!
//! This module exposes different kind of types for `Game`

use crate::{Color, Move, MoveResult, Position};

/// ## GameResult
///
/// Describes the result of a game
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum GameResult {
    /// The game continues without any problem
    Continuing,
    /// The game is not finished, and is still in play.
    /// In addition to this a `Promotion` must be performed via `promote()` method
    Promote(Position),
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
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum EndGame {
    /// One player, the victor, checkmated the other.
    /// This stores the color of the winner.
    Victory(Color),
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

impl From<MoveResult> for GameResult {
    fn from(res: MoveResult) -> Self {
        match res {
            MoveResult::Continuing(_) => GameResult::Continuing,
            MoveResult::Promote(_, pos) => GameResult::Promote(pos),
            MoveResult::IllegalMove(m) => GameResult::IllegalMove(m),
            MoveResult::Stalemate => GameResult::Ended(EndGame::Draw),
            MoveResult::Victory(color) => GameResult::Ended(EndGame::Victory(color)),
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use crate::position::*;
    use crate::{Board, WHITE};

    #[test]
    fn game_result_from_move_result() {
        assert_eq!(
            GameResult::from(MoveResult::Continuing(Board::default())),
            GameResult::Continuing
        );
        assert_eq!(
            GameResult::from(MoveResult::Promote(Board::default(), H2)),
            GameResult::Promote(H2)
        );
        assert_eq!(
            GameResult::from(MoveResult::IllegalMove(Move::KingSideCastle)),
            GameResult::IllegalMove(Move::KingSideCastle),
        );
        assert_eq!(
            GameResult::from(MoveResult::Stalemate),
            GameResult::Ended(EndGame::Draw)
        );
        assert_eq!(
            GameResult::from(MoveResult::Victory(WHITE)),
            GameResult::Ended(EndGame::Victory(WHITE)),
        );
    }
}
