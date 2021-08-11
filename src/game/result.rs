//! # Result
//!
//! Result types for Game.

use crate::{Color, Move};

use core::fmt;

/// ## GameResult
///
/// Result of a game action
pub type GameResult = Result<(GameState, GameEvent), GameError>;

/// ## GameError
///
/// Describes the error of a game
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GameError {
    /// An illegal move was made. This can include many things,
    /// such as moving a piece through another piece, attempting
    /// to capture an allied piece, moving non-orthogonally or
    /// non-diagonally, or non-knight-like according the rules
    /// governing the movement of the piece. Additionally,
    /// moves that put the player in check, (for example, moving a pinned piece),
    /// are also illegal.
    IllegalMove(Move),
    /// Promotion is not allowed
    CantPromote,
}

/// ## GameState
///
/// Describes the state of a game
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GameState {
    /// The game continues without any problem
    Continuing,
    /// The game has ended; match the `EndGame` variant, to get the end game result
    Ended(EndGame),
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

// -- event

bitflags! {
    /// ## GameEvent
    ///
    /// Describes an event "raised" after a move is played.
    /// An event is an intersection of different values
    pub struct GameEvent: u8 {
        /// No event reported
        const NONE                  = 0b00000000;
        /// Opponent king is now in check
        const CHECK                 = 0b00000010;
        /// Opponent king is in checkmate
        const CHECKMATE             = 0b00000100;
        /// A promotion is available for one of current player pawn
        const PROMOTION_AVAILABLE   = 0b00001000;
        /// Threefold repetition detected; it is reported even if not enabled in options
        const THREEFOLD_REPETITION  = 0b00010000;
        /// Fivefold repetition detected; it is reported only if enabled in options
        const FIVEFOLD_REPETITION   = 0b00100000;
    }
}

impl GameEvent {
    /// ### is_check
    ///
    /// Returns whether event is check
    pub fn is_check(&self) -> bool {
        self.intersects(GameEvent::CHECK)
    }

    /// ### is_checkmate
    ///
    /// Returns whether event is checkmate
    pub fn is_checkmate(&self) -> bool {
        self.intersects(GameEvent::CHECKMATE)
    }

    /// ### is_promotion_available
    ///
    /// Returns whether a pawn promotion is available
    pub fn is_promotion_available(&self) -> bool {
        self.intersects(GameEvent::PROMOTION_AVAILABLE)
    }

    /// ### is_checkmate
    ///
    /// Returns whether event is threefold repetition
    pub fn is_threefold_repetition(&self) -> bool {
        self.intersects(GameEvent::THREEFOLD_REPETITION)
    }

    /// ### is_checkmate
    ///
    /// Returns whether event is fivefold repetition
    pub fn is_fivefold_repetition(&self) -> bool {
        self.intersects(GameEvent::FIVEFOLD_REPETITION)
    }
}

// -- fmt

impl fmt::Display for GameError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GameError::CantPromote => {
                write!(f, "Can't promote pawn, since there's no pawn to promote")
            }
            GameError::IllegalMove(m) => write!(f, "Illegal move: {}", m),
        }
    }
}

// -- functions

/// ### was_illegal_move
///
/// Returns whether game result was an illegal move
pub fn was_illegal_move(res: &GameResult) -> bool {
    matches!(res, Err(GameError::IllegalMove(_)))
}

/// ### set_result_event
///
/// Set `GameEvent` to `GameResult`
pub fn set_result_event(res: GameResult, ev: GameEvent) -> GameResult {
    match res {
        Ok((state, mut event)) => {
            event.insert(ev);
            Ok((state, event))
        }
        Err(err) => Err(err),
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use alloc::string::ToString;

    #[test]
    fn fmt_game_error() {
        assert_eq!(
            GameError::CantPromote.to_string().as_str(),
            "Can't promote pawn, since there's no pawn to promote"
        );
        assert_eq!(
            GameError::IllegalMove(Move::KingSideCastle)
                .to_string()
                .as_str(),
            "Illegal move: O-O"
        );
    }

    #[test]
    fn was_illegal_move() {
        assert_eq!(
            super::was_illegal_move(&Err(GameError::IllegalMove(Move::Resign))),
            true
        );
        assert_eq!(
            super::was_illegal_move(&Ok((GameState::Continuing, GameEvent::NONE))),
            false
        );
    }

    #[test]
    fn set_result_event() {
        assert_eq!(
            super::set_result_event(
                GameResult::Ok((GameState::Continuing, GameEvent::NONE)),
                GameEvent::THREEFOLD_REPETITION
            ),
            GameResult::Ok((GameState::Continuing, GameEvent::THREEFOLD_REPETITION))
        );
        // Intersect
        assert_eq!(
            super::set_result_event(
                GameResult::Ok((GameState::Continuing, GameEvent::THREEFOLD_REPETITION)),
                GameEvent::CHECK
            ),
            GameResult::Ok((
                GameState::Continuing,
                GameEvent::THREEFOLD_REPETITION | GameEvent::CHECK
            ))
        );
        // Error
        assert_eq!(
            super::set_result_event(GameResult::Err(GameError::CantPromote), GameEvent::NONE,),
            GameResult::Err(GameError::CantPromote)
        );
    }

    #[test]
    fn game_event_flags() {
        assert_eq!(GameEvent::CHECK.is_check(), true);
        assert_eq!(GameEvent::NONE.is_check(), false);
        assert_eq!(GameEvent::CHECKMATE.is_checkmate(), true);
        assert_eq!(GameEvent::NONE.is_checkmate(), false);
        assert_eq!(
            GameEvent::PROMOTION_AVAILABLE.is_promotion_available(),
            true
        );
        assert_eq!(GameEvent::NONE.is_promotion_available(), false);
        assert_eq!(
            GameEvent::THREEFOLD_REPETITION.is_threefold_repetition(),
            true
        );
        assert_eq!(GameEvent::NONE.is_check(), false);
        assert_eq!(
            GameEvent::FIVEFOLD_REPETITION.is_fivefold_repetition(),
            true
        );
        assert_eq!(GameEvent::NONE.is_check(), false);
    }
}
