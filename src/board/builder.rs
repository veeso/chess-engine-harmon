//! # Builder
//!
//! The builder exposes the BoardBuilder, which can be used to create custom games

use super::{Board, Color, Piece, Square, BLACK, WHITE};

/// ## BoardBuilder
///
/// The board builder is the struct which provides a helper to build custom chess games
pub struct BoardBuilder {
    board: Board,
}

impl From<Board> for BoardBuilder {
    fn from(board: Board) -> Self {
        Self { board }
    }
}

impl Default for BoardBuilder {
    fn default() -> Self {
        let mut board = Board::empty();
        board.white_castling_rights.disable_all();
        board.black_castling_rights.disable_all();
        Self { board }
    }
}

impl BoardBuilder {
    pub fn row(mut self, piece: Piece) -> Self {
        let mut pos = piece.get_pos();
        while pos.get_col() > 0 {
            pos = pos.next_left()
        }

        for _ in 0..8 {
            *self.board.get_square(pos) = Square::from(piece.move_to(pos));
            pos = pos.next_right();
        }

        self
    }

    pub fn column(mut self, piece: Piece) -> Self {
        let mut pos = piece.get_pos();
        while pos.get_row() > 0 {
            pos = pos.next_below()
        }

        for _ in 0..8 {
            *self.board.get_square(pos) = Square::from(piece.move_to(pos));
            pos = pos.next_above();
        }

        self
    }

    pub fn piece(mut self, piece: Piece) -> Self {
        let pos = piece.get_pos();
        *self.board.get_square(pos) = Square::from(piece);
        self
    }

    pub fn enable_castling(mut self) -> Self {
        self.board.black_castling_rights.enable_all();
        self.board.white_castling_rights.enable_all();
        self
    }

    pub fn disable_castling(mut self) -> Self {
        self.board.black_castling_rights.disable_all();
        self.board.white_castling_rights.disable_all();
        self
    }

    pub fn enable_queenside_castle(mut self, color: Color) -> Self {
        match color {
            WHITE => self.board.white_castling_rights.enable_queenside(),
            BLACK => self.board.black_castling_rights.enable_queenside(),
        }
        self
    }

    pub fn disable_queenside_castle(mut self, color: Color) -> Self {
        match color {
            WHITE => self.board.white_castling_rights.disable_queenside(),
            BLACK => self.board.black_castling_rights.disable_queenside(),
        }
        self
    }

    pub fn enable_kingside_castle(mut self, color: Color) -> Self {
        match color {
            WHITE => self.board.white_castling_rights.enable_kingside(),
            BLACK => self.board.black_castling_rights.enable_kingside(),
        }
        self
    }

    pub fn disable_kingside_castle(mut self, color: Color) -> Self {
        match color {
            WHITE => self.board.white_castling_rights.disable_kingside(),
            BLACK => self.board.black_castling_rights.disable_kingside(),
        }
        self
    }

    pub fn build(self) -> Board {
        self.board
    }
}
