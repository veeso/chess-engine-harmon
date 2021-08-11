//! # Game
//!
//! The game module is the highest level library api. This module exposes the `Game` type which, along to the board, contains also
//! the metadata of a match and the played moves. In addition to this, it also adds more sophisticated game logics to complete
//! the chess game (e.g. the threefold repetition stallmate).

use alloc::vec::Vec;
use core::time::Duration;

// -- modules
mod builder;
mod clock;
pub mod metadata;
mod options;
mod result;
mod types;

// -- imports
use crate::{Board, Color, Move, MoveResult, Piece, Position, Promotion};
use metadata::{Metadata, Result as MetadataResult};

// -- export
pub use builder::GameBuilder;
pub use clock::Clock;
pub use options::Options;
pub use result::{EndGame, GameError, GameEvent, GameResult, GameState, VictoryReason};
pub use types::GameMove;

/// ## Game
///
/// A wrapper around the `Board`, which also tracks time, moves and match metadata (e.g. player data, date, event, location...).
/// The game also adds some missing logics to the board, such as the "threefold repetition" stalemate.
/// Game exposes a limited api to the board, which allows only to play a "standard and clean" chess match.
#[derive(Debug, Clone)]
pub struct Game {
    /// Current board state
    board: Board,
    /// Game clocks
    clock: Clock,
    /// Game metadata
    metadata: Metadata,
    /// Game moves
    moves: Vec<GameMove>,
    /// Game options
    options: Options,
}

impl Default for Game {
    fn default() -> Self {
        Self {
            board: Board::default(),
            clock: Clock::new(Duration::MAX, Duration::MAX),
            metadata: Metadata::default(),
            moves: Vec::default(),
            options: Options::default(),
        }
    }
}

impl Game {
    // -- getters

    /// ### board
    ///
    /// Get a reference of the board, in case you want to access to more specific board methods.
    /// Mind that you won't be able to mutate the board, but only to read or copy it
    pub fn board(&self) -> &Board {
        &self.board
    }

    /// ### remaining_time
    ///
    /// Get remaining time on the clock.
    /// The first element of the tuple is remaining time for white player,
    /// while the second element is remaining time for black player
    pub fn remaining_time(&self) -> (Duration, Duration) {
        self.clock.remaining_time()
    }

    /// ### metadata
    ///
    /// Get a reference to metadata
    pub fn metadata(&self) -> &Metadata {
        &self.metadata
    }

    /// ### moves
    ///
    /// Get a reference to the moves list
    pub fn moves(&self) -> &[GameMove] {
        self.moves.as_slice()
    }

    // -- board getters

    /// ### turn
    ///
    /// Return turn color and number
    pub fn turn(&self) -> (Color, u16) {
        (self.board().get_turn(), self.get_turn())
    }

    /// ### get_legal_moves
    ///
    /// Get legal moves for current player
    pub fn get_legal_moves(&self) -> Vec<Move> {
        self.board().get_legal_moves(self.board().get_turn())
    }

    /// ### get_piece_legal_moves
    ///
    /// Get legal moves for piece at `pos` position
    pub fn get_piece_legal_moves(&self, pos: Position) -> Vec<Move> {
        self.board().get_piece_legal_moves(pos)
    }

    /// ### in_progress
    ///
    /// Returns whether current match is still in progress
    pub fn in_progress(&self) -> bool {
        matches!(self.metadata().result(), MetadataResult::InProgress)
    }

    /// ### has_terminated
    ///
    /// Returns whether current match has terminated
    pub fn has_terminated(&self) -> bool {
        !self.in_progress()
    }

    // -- game

    /// ### play_move
    ///
    /// play a move.
    /// You must also provide the time taken to move the piece.
    pub fn play_move(&mut self, m: Move, time: Duration) -> GameResult {
        let (player, turn): (Color, u16) = self.turn();
        // sub time and check timeout
        self.sub_time(player, time);
        if self.clock.timeout(player) {
            self.set_result_win(!player);
            return GameResult::Ok((
                GameState::Ended(EndGame::Victory(!player, VictoryReason::Timeout)),
                GameEvent::NONE,
            ));
        }
        // Play move
        let result: MoveResult = self.board.play_move(m);
        // Handle game result
        let result: GameResult = self.handle_move_result(result, None);
        // Push move, unless illegal
        if !result::was_illegal_move(&result) {
            self.push_move(m, player, turn, time, self.board().get_taken_piece());
        }
        // Check events
        let result: GameResult = self.check_events(result);
        // If is checkmate, set win result
        if let Ok((GameState::Ended(EndGame::Victory(player, _)), _)) = result {
            self.set_result_win(player);
        }
        // Return result
        result
    }

    /// ### resign
    ///
    /// Resign match for current player
    pub fn resign(&mut self) -> GameResult {
        self.handle_move_result(self.board.play_move(Move::Resign), None)
    }

    /// ### draw
    ///
    /// Draw game
    pub fn draw(&mut self) -> GameResult {
        self.set_result_drawn();
        Ok((GameState::Ended(EndGame::Draw), GameEvent::NONE))
    }

    /// ### promote
    ///
    /// Promote the pawn on the last line.
    /// Returns the GameState.
    /// If there's no pawn to promote, returns `Err(GameError::CantPromote)`
    pub fn promote(&mut self, promotion: Promotion) -> GameResult {
        if self.board.get_promoting_pawn().is_some() {
            // Promote piece and return
            self.handle_move_result(self.board.promote(promotion), Some(promotion))
        } else {
            Err(GameError::CantPromote)
        }
    }

    // -- clocks

    /// ### add_time
    ///
    /// Increment time for player for the amount provided
    pub fn add_time(&mut self, player: Color, time: Duration) {
        self.clock.add_time(player, time);
    }

    /// ### sub_time
    ///
    /// Subtract time for player for the amount provided.
    /// Use then `timeout()` to check whether the time's up
    pub fn sub_time(&mut self, player: Color, time: Duration) {
        self.clock.sub_time(player, time);
    }

    /// ### timeout
    ///
    /// Returns whether time's out for provided player
    pub fn timeout(&self, player: Color) -> bool {
        self.clock.timeout(player)
    }

    // -- validation

    /// ### is_threefold_repetition
    ///
    /// checks whether in threefold repetition condition.
    /// Basically checks whether last 3 turns are the same.
    /// This function is public, in case you want to allow player to claim for draw on threefold repetition
    pub fn is_threefold_repetition(&self) -> bool {
        self.is_n_repetition(3)
    }

    // -- private

    // -- result

    /// ### check_events
    ///
    /// Check events and put them in game results
    fn check_events(&mut self, mut result: GameResult) -> GameResult {
        // Get check event
        if self.board().is_check() {
            result = result::set_result_event(result, GameEvent::CHECK);
        }
        // Get checkmate event
        if self.board().is_checkmate() {
            result = result::set_result_event(result, GameEvent::CHECKMATE);
        }
        // get promotion event
        if self.board().get_promoting_pawn().is_some() {
            result = result::set_result_event(result, GameEvent::PROMOTION_AVAILABLE);
        }
        // Check threefold repetition
        if self.is_threefold_repetition() {
            result = result::set_result_event(result, GameEvent::THREEFOLD_REPETITION);
            // If option is enabled, draw game
            if self.options.threefold_repetition {
                // Draw game
                self.set_result_drawn();
            }
        }
        // Check fivefold repetition
        if self.is_fivefold_repetition() && self.options.fivefold_repetition {
            // Draw game
            self.set_result_drawn();
            result = result::set_result_event(result, GameEvent::FIVEFOLD_REPETITION);
        }
        result
    }

    /// ### handle_move_result
    ///
    /// Given a move result, returns a `GameResult` after updating the board
    /// If the last move has promoted a pawn, the history is patched with the promotion.
    fn handle_move_result(
        &mut self,
        res: MoveResult,
        has_promoted: Option<Promotion>,
    ) -> GameResult {
        // Patch last move
        if let Some(promotion) = has_promoted {
            self.patch_last_move_promotion(promotion);
        }
        match res {
            MoveResult::Continuing(board) | MoveResult::Promote(board, _) => {
                // Update board
                self.board = board;
                // Return continuing
                Ok((GameState::Continuing, GameEvent::NONE))
            }
            MoveResult::Victory(color) => {
                // Set result and return game ended
                self.set_result_win(color);
                Ok((
                    GameState::Ended(EndGame::Victory(color, VictoryReason::Checkmate)),
                    GameEvent::NONE,
                ))
            }
            MoveResult::Stalemate => {
                // Set result and return game ended
                self.set_result_drawn();
                Ok((GameState::Ended(EndGame::Draw), GameEvent::NONE))
            }
            MoveResult::IllegalMove(m) => Err(GameError::IllegalMove(m)),
        }
    }

    // -- repetitions

    /// ### is_threefold_repetition
    ///
    /// checks whether in fivefold repetition condition.
    /// Basically checks whether last 5 turns are the same
    fn is_fivefold_repetition(&self) -> bool {
        self.is_n_repetition(5)
    }

    /// ### is_n_repetition
    ///
    /// checks whether the last `repetitions` turns are the equal to each other
    fn is_n_repetition(&self, repetitions: usize) -> bool {
        let repetitions: usize = repetitions * 2; // Multiply by 2, since we calculate turns, not moves
        if self.moves().len() < repetitions || self.moves().is_empty() {
            false
        } else {
            // Get last `repetitions` moves
            let moves: Vec<&Move> = self
                .moves()
                .iter()
                .rev()
                .take(repetitions)
                .map(|x| &x.itself)
                .collect();
            let first: &Move = moves[0];
            moves.iter().all(|x| **x == *first)
        }
    }

    // -- moves

    /// ### push_move
    ///
    /// Push move to history
    fn push_move(
        &mut self,
        m: Move,
        player: Color,
        turn: u16,
        time: Duration,
        piece_taken: Option<Piece>,
    ) {
        self.moves
            .push(GameMove::new(m, player, turn, time, piece_taken, None));
    }

    /// ### get_turn
    ///
    /// Get turn number.
    /// Turn number is `moves.len() / 2) + 1`
    fn get_turn(&self) -> u16 {
        ((self.moves.len() / 2) + 1) as u16
    }

    /// ### patch_last_move_promotion
    ///
    /// Patch last move, setting the promotion to `p` value
    fn patch_last_move_promotion(&mut self, p: Promotion) {
        if let Some(m) = self.last_move() {
            m.promotion = Some(p);
        }
    }

    /// ### last_move
    ///
    /// Get a mutable reference to the last move
    fn last_move(&mut self) -> Option<&mut GameMove> {
        self.moves.iter_mut().last()
    }

    // -- metadata result

    /// ### set_result_win
    ///
    /// Set result to win for provided player
    fn set_result_win(&mut self, color: Color) {
        self.metadata.set_result(match color {
            Color::Black => MetadataResult::BlackWins,
            Color::White => MetadataResult::WhiteWins,
        });
    }

    /// ### set_result_drawn
    ///
    /// Set result to drawn
    fn set_result_drawn(&mut self) {
        self.metadata.set_result(MetadataResult::DrawnGame);
    }
}

#[cfg(test)]
mod test {

    use super::*;

    use pretty_assertions::assert_eq;
}
