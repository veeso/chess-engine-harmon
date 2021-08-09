//! # Types
//!
//! This module exposes different kind of types for `Game`

use crate::{Color, Move, MoveResult, Piece, Position, Promotion};

use core::time::Duration;

// -- results

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
        turn: u16,
        time: Duration,
        piece_taken: Option<Piece>,
        promotion: Option<Promotion>,
    ) -> Self {
        Self {
            itself: m,
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

    #[test]
    fn game_move() {
        let m: GameMove = GameMove {
            itself: Move::Resign,
            turn: 2,
            time: Duration::from_secs(5),
            piece_taken: None,
            promotion: None,
        };
        assert_eq!(m.itself, Move::Resign);
        assert_eq!(m.turn, 2);
        assert_eq!(m.time, Duration::from_secs(5));
        assert_eq!(m.piece_taken, None);
        assert_eq!(m.promotion, None);
        let m: GameMove = GameMove::new(Move::Resign, 2, Duration::from_secs(5), None, None);
        assert_eq!(m.itself, Move::Resign);
        assert_eq!(m.turn, 2);
        assert_eq!(m.time, Duration::from_secs(5));
        assert_eq!(m.piece_taken, None);
        assert_eq!(m.promotion, None);
    }
}
