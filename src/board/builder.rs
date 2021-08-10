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
    /// ### row
    ///
    /// Fill piece row with clones of piece
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

    /// ### column
    ///
    /// Fill piece column with clones of piece
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

    /// ### piece
    ///
    /// Put `piece` in the board
    pub fn piece(mut self, piece: Piece) -> Self {
        let pos = piece.get_pos();
        *self.board.get_square(pos) = Square::from(piece);
        self
    }

    /// ### enable_castling
    ///
    /// Enable castling rights for both king and queen for both players
    pub fn enable_castling(mut self) -> Self {
        self.board.black_castling_rights.enable_all();
        self.board.white_castling_rights.enable_all();
        self
    }

    /// ### disable_castling
    ///
    /// Disable castling rights for both king and queen for both players
    pub fn disable_castling(mut self) -> Self {
        self.board.black_castling_rights.disable_all();
        self.board.white_castling_rights.disable_all();
        self
    }

    /// ### enable_queenside_castle
    ///
    /// enable queenside castling rights for selected player
    pub fn enable_queenside_castle(mut self, color: Color) -> Self {
        match color {
            WHITE => self.board.white_castling_rights.enable_queenside(),
            BLACK => self.board.black_castling_rights.enable_queenside(),
        }
        self
    }

    /// ### disable_queenside_castle
    ///
    /// disable queenside castling rights for selected player
    pub fn disable_queenside_castle(mut self, color: Color) -> Self {
        match color {
            WHITE => self.board.white_castling_rights.disable_queenside(),
            BLACK => self.board.black_castling_rights.disable_queenside(),
        }
        self
    }

    /// ### enable_kingside_castle
    ///
    /// enable kingside castling rights for selected player
    pub fn enable_kingside_castle(mut self, color: Color) -> Self {
        match color {
            WHITE => self.board.white_castling_rights.enable_kingside(),
            BLACK => self.board.black_castling_rights.enable_kingside(),
        }
        self
    }

    /// ### disable_kingside_castle
    ///
    /// disable kingside castling rights for selected player
    pub fn disable_kingside_castle(mut self, color: Color) -> Self {
        match color {
            WHITE => self.board.white_castling_rights.disable_kingside(),
            BLACK => self.board.black_castling_rights.disable_kingside(),
        }
        self
    }

    /// ### player_moving
    ///
    /// Set first player moving
    pub fn player_moving(mut self, color: Color) -> Self {
        self.board.set_turn(color);
        self
    }

    /// ### build
    ///
    /// Get board with selected options
    pub fn build(self) -> Board {
        self.board
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use crate::position::*;
    use crate::{BLACK, WHITE};

    use pretty_assertions::assert_eq;

    #[test]
    fn default() {
        let builder: BoardBuilder = BoardBuilder::default();
        assert_eq!(builder.board.get_legal_moves(WHITE).is_empty(), true);
        assert_eq!(builder.board.get_legal_moves(BLACK).is_empty(), true);
        assert_eq!(
            builder.board.black_castling_rights.can_kingside_castle(),
            false
        );
        assert_eq!(
            builder.board.black_castling_rights.can_queenside_castle(),
            false
        );
        assert_eq!(
            builder.board.white_castling_rights.can_kingside_castle(),
            false
        );
        assert_eq!(
            builder.board.white_castling_rights.can_queenside_castle(),
            false
        );
    }

    #[test]
    fn from() {
        let builder: BoardBuilder = BoardBuilder::from(Board::default());
        assert_eq!(builder.board.get_legal_moves(WHITE).len(), 20); // You have 20 available moves at the beginning
        assert_eq!(builder.board.get_legal_moves(BLACK).len(), 20); // You have 20 available moves at the beginning
        assert_eq!(
            builder.board.black_castling_rights.can_kingside_castle(),
            true
        );
        assert_eq!(
            builder.board.black_castling_rights.can_queenside_castle(),
            true
        );
        assert_eq!(
            builder.board.white_castling_rights.can_kingside_castle(),
            true
        );
        assert_eq!(
            builder.board.white_castling_rights.can_queenside_castle(),
            true
        );
    }

    #[test]
    fn row() {
        let board: Board = BoardBuilder::default().row(Piece::Queen(WHITE, A1)).build();
        assert_eq!(board.get_piece(A1).unwrap(), Piece::Queen(WHITE, A1));
        assert_eq!(board.get_piece(B1).unwrap(), Piece::Queen(WHITE, B1));
        assert_eq!(board.get_piece(C1).unwrap(), Piece::Queen(WHITE, C1));
        assert_eq!(board.get_piece(D1).unwrap(), Piece::Queen(WHITE, D1));
        assert_eq!(board.get_piece(E1).unwrap(), Piece::Queen(WHITE, E1));
        assert_eq!(board.get_piece(F1).unwrap(), Piece::Queen(WHITE, F1));
        assert_eq!(board.get_piece(G1).unwrap(), Piece::Queen(WHITE, G1));
        assert_eq!(board.get_piece(H1).unwrap(), Piece::Queen(WHITE, H1));
    }

    #[test]
    fn col() {
        let board: Board = BoardBuilder::default()
            .column(Piece::Queen(WHITE, A1))
            .build();
        assert_eq!(board.get_piece(A1).unwrap(), Piece::Queen(WHITE, A1));
        assert_eq!(board.get_piece(A2).unwrap(), Piece::Queen(WHITE, A2));
        assert_eq!(board.get_piece(A3).unwrap(), Piece::Queen(WHITE, A3));
        assert_eq!(board.get_piece(A4).unwrap(), Piece::Queen(WHITE, A4));
        assert_eq!(board.get_piece(A5).unwrap(), Piece::Queen(WHITE, A5));
        assert_eq!(board.get_piece(A6).unwrap(), Piece::Queen(WHITE, A6));
        assert_eq!(board.get_piece(A7).unwrap(), Piece::Queen(WHITE, A7));
        assert_eq!(board.get_piece(A8).unwrap(), Piece::Queen(WHITE, A8));
    }

    #[test]
    fn piece() {
        let board: Board = BoardBuilder::default()
            .piece(Piece::Rook(WHITE, A1))
            .piece(Piece::Rook(BLACK, H8))
            .build();
        assert_eq!(board.get_piece(A1).unwrap(), Piece::Rook(WHITE, A1));
        assert_eq!(board.get_piece(H8).unwrap(), Piece::Rook(BLACK, H8));
    }

    #[test]
    fn player_moving() {
        let board: Board = BoardBuilder::default().player_moving(BLACK).build();
        assert_eq!(board.get_turn(), BLACK);
    }

    #[test]
    fn castling_rights() {
        // all
        let board: Board = BoardBuilder::default().enable_castling().build();
        assert_eq!(board.black_castling_rights.can_kingside_castle(), true);
        assert_eq!(board.black_castling_rights.can_queenside_castle(), true);
        assert_eq!(board.white_castling_rights.can_kingside_castle(), true);
        assert_eq!(board.white_castling_rights.can_queenside_castle(), true);
        // white king only
        let board: Board = BoardBuilder::default()
            .enable_kingside_castle(WHITE)
            .build();
        assert_eq!(board.black_castling_rights.can_kingside_castle(), false);
        assert_eq!(board.black_castling_rights.can_queenside_castle(), false);
        assert_eq!(board.white_castling_rights.can_kingside_castle(), true);
        assert_eq!(board.white_castling_rights.can_queenside_castle(), false);
        // black king only
        let board: Board = BoardBuilder::default()
            .enable_kingside_castle(BLACK)
            .build();
        assert_eq!(board.black_castling_rights.can_kingside_castle(), true);
        assert_eq!(board.black_castling_rights.can_queenside_castle(), false);
        assert_eq!(board.white_castling_rights.can_kingside_castle(), false);
        assert_eq!(board.white_castling_rights.can_queenside_castle(), false);
        // white queen only
        let board: Board = BoardBuilder::default()
            .enable_queenside_castle(WHITE)
            .build();
        assert_eq!(board.black_castling_rights.can_kingside_castle(), false);
        assert_eq!(board.black_castling_rights.can_queenside_castle(), false);
        assert_eq!(board.white_castling_rights.can_kingside_castle(), false);
        assert_eq!(board.white_castling_rights.can_queenside_castle(), true);
        // black queen only
        let board: Board = BoardBuilder::default()
            .enable_queenside_castle(BLACK)
            .build();
        assert_eq!(board.black_castling_rights.can_kingside_castle(), false);
        assert_eq!(board.black_castling_rights.can_queenside_castle(), true);
        assert_eq!(board.white_castling_rights.can_kingside_castle(), false);
        assert_eq!(board.white_castling_rights.can_queenside_castle(), false);
        // Disable all
        let board: Board = BoardBuilder::default().disable_castling().build();
        assert_eq!(board.black_castling_rights.can_kingside_castle(), false);
        assert_eq!(board.black_castling_rights.can_queenside_castle(), false);
        assert_eq!(board.white_castling_rights.can_kingside_castle(), false);
        assert_eq!(board.white_castling_rights.can_queenside_castle(), false);
        // Disable queen
        let board: Board = BoardBuilder::default()
            .enable_castling()
            .disable_queenside_castle(BLACK)
            .build();
        assert_eq!(board.black_castling_rights.can_kingside_castle(), true);
        assert_eq!(board.black_castling_rights.can_queenside_castle(), false);
        assert_eq!(board.white_castling_rights.can_kingside_castle(), true);
        assert_eq!(board.white_castling_rights.can_queenside_castle(), true);
        let board: Board = BoardBuilder::default()
            .enable_castling()
            .disable_kingside_castle(BLACK)
            .build();
        assert_eq!(board.black_castling_rights.can_kingside_castle(), false);
        assert_eq!(board.black_castling_rights.can_queenside_castle(), true);
        assert_eq!(board.white_castling_rights.can_kingside_castle(), true);
        assert_eq!(board.white_castling_rights.can_queenside_castle(), true);
    }
}
