//! # Builder
//!
//! this module exposes a helper struct to build `Game` struct

use super::{Board, Clock, Duration, Game, GameMove, Metadata, Options};

use alloc::vec::Vec;

/// ## GameBuilder
///
/// Helper struct to build `Game`
pub struct GameBuilder {
    game: Option<Game>,
}

impl Default for GameBuilder {
    fn default() -> Self {
        Self {
            game: Some(Game::default()),
        }
    }
}

impl GameBuilder {
    /// ### board
    ///
    /// Set board for game
    pub fn board(mut self, board: Board) -> Self {
        self.game.as_mut().unwrap().board = board;
        self
    }

    /// ### timeout
    ///
    /// Set remaining time for players
    pub fn timeout(mut self, for_white: Duration, for_black: Duration) -> Self {
        self.game.as_mut().unwrap().clock = Clock::new(for_white, for_black);
        self
    }

    /// ### metadata
    ///
    /// Set game metadata
    pub fn metadata(mut self, metadata: Metadata) -> Self {
        self.game.as_mut().unwrap().metadata = metadata;
        self
    }

    /// ### moves
    ///
    /// Set game moves
    pub fn moves(mut self, moves: Vec<GameMove>) -> Self {
        self.game.as_mut().unwrap().moves = moves;
        self
    }

    /// ### options
    ///
    /// Set game options
    pub fn options(mut self, options: Options) -> Self {
        self.game.as_mut().unwrap().options = options;
        self
    }

    /// ### build
    ///
    /// Take `Game` structure out from builder
    pub fn build(mut self) -> Game {
        self.game.take().unwrap()
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use crate::{Color, Move};

    use pretty_assertions::assert_eq;

    #[test]
    fn game_builder() {
        let game: Game = GameBuilder::default()
            .board(Board::dunsany())
            .metadata(Metadata::default().with_date(2021, 08, 08))
            .moves(vec![GameMove::new(
                Move::Resign,
                Color::White,
                1,
                Duration::from_secs(60),
                None,
                None,
            )])
            .timeout(Duration::from_secs(3), Duration::from_secs(5))
            .build();
        assert_eq!(game.board.get_turn(), Color::Black);
        assert_eq!(game.metadata.date().unwrap().year(), 2021);
        assert_eq!(game.moves.len(), 1);
        assert_eq!(
            game.clock.remaining_time(),
            (Duration::from_secs(3), Duration::from_secs(5))
        );
        // TODO: add option test
    }

    #[test]
    #[should_panic]
    fn game_already_built() {
        let mut builder: GameBuilder = GameBuilder::default();
        builder.game = None;
        builder.build();
    }
}
