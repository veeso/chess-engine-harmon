//! # Game
//!
//! The game module is the highest level library api. This module exposes the `Game` type which, along to the board, contains also
//! the metadata of a match and the played moves. In addition to this, it also adds more sophisticated game logics to complete
//! the chess game (e.g. the threefold repetition stallmate).
//!

use alloc::vec::Vec;
use core::time::Duration;

// -- modules
mod builder;
mod clock;
pub mod metadata;
mod options;
mod types;

// -- imports
use crate::{Board, Color, Move, MoveResult, Piece, Position, Promotion};
use metadata::{Metadata, Result as MetadataResult};

// -- export
pub use builder::GameBuilder;
pub use clock::Clock;
pub use options::Options;
pub use types::{EndGame, GameEvent, GameMove, GameResult, VictoryReason};

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

    /// ### can_promote
    ///
    /// Returns whether current player can promote pawn
    pub fn can_promote(&self) -> bool {
        self.board().get_promoting_pawn().is_some()
    }

    /// ### is_in_check
    ///
    /// Returns whether current player is in check
    pub fn is_in_check(&self) -> bool {
        self.board().is_in_check(self.board().get_turn())
    }

    // -- game

    /// ### play_move
    ///
    /// play a move.
    /// You must also provide the time taken to move the piece.
    pub fn play_move(&mut self, m: Move, time: Duration) -> (GameResult, GameEvent) {
        let (player, turn): (Color, u16) = self.turn();
        // Play move
        let result: MoveResult = self.board.play_move(m);
        // Handle game result
        let mut result: GameResult = self.handle_move_result(result, None);
        // Push move, unless illegal
        if !result.was_illegal_move() {
            self.push_move(m, player, turn, time, self.board().get_taken_piece());
        }
        let mut event: GameEvent = GameEvent::None;
        // get promotion event
        if let Some(promotion) = self.board().get_promoting_pawn() {
            event = GameEvent::Promotion(promotion);
        }
        // Check threefold repetition
        if self.is_threefold_repetition() {
            // If option is enabled, draw game
            if self.options.threefold_repetition {
                // Draw game
                self.set_result_drawn();
                result = GameResult::Ended(EndGame::Draw);
            }
            event = GameEvent::ThreefoldRepetition;
        }
        // Check fivefold repetition
        if self.is_fivefold_repetition() && self.options.fivefold_repetition {
            // Draw game
            self.set_result_drawn();
            result = GameResult::Ended(EndGame::Draw);
            event = GameEvent::FivefoldRepetition;
        }
        // If is checkmate, set win result
        if let GameResult::Ended(EndGame::Victory(player, _)) = result {
            self.set_result_win(player);
        }
        // Return result
        (result, event)
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
        GameResult::Ended(EndGame::Draw)
    }

    /// ### promote
    ///
    /// Promote the pawn on the last line.
    /// Returns the GameResult.
    /// If there's no pawn to promote, returns `Err(())`
    pub fn promote(&mut self, promotion: Promotion) -> Result<GameResult, ()> {
        // TODO: replace this empty error
        if self.board.get_promoting_pawn().is_some() {
            // Promote piece and return
            Ok(self.handle_move_result(self.board.promote(promotion), Some(promotion)))
        } else {
            Err(())
        }
    }

    // -- clocks

    // TODO: handle clocks

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
                GameResult::Continuing
            }
            MoveResult::Victory(color) => {
                // Set result and return game ended
                self.set_result_win(color);
                GameResult::Ended(EndGame::Victory(color, VictoryReason::Checkmate))
            }
            MoveResult::Stalemate => {
                // Set result and return game ended
                self.set_result_drawn();
                GameResult::Ended(EndGame::Draw)
            }
            MoveResult::IllegalMove(m) => GameResult::IllegalMove(m),
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
