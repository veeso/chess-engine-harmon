//! # Position
//!
//! This module exposes the type to define a position on the chess board.
//! A position is made up of two attributes:
//!
//! - the row
//! - the column
//!
//! This module also exposes all the alias for the positions (e.g. `D4` or `C6`)
//!

use super::{Color, BLACK, WHITE};
use alloc::{str::FromStr, string::String, vec::Vec};

// -- alias

pub const A1: Position = Position::new(0, 0);
pub const A2: Position = Position::new(1, 0);
pub const A3: Position = Position::new(2, 0);
pub const A4: Position = Position::new(3, 0);
pub const A5: Position = Position::new(4, 0);
pub const A6: Position = Position::new(5, 0);
pub const A7: Position = Position::new(6, 0);
pub const A8: Position = Position::new(7, 0);

pub const B1: Position = Position::new(0, 1);
pub const B2: Position = Position::new(1, 1);
pub const B3: Position = Position::new(2, 1);
pub const B4: Position = Position::new(3, 1);
pub const B5: Position = Position::new(4, 1);
pub const B6: Position = Position::new(5, 1);
pub const B7: Position = Position::new(6, 1);
pub const B8: Position = Position::new(7, 1);

pub const C1: Position = Position::new(0, 2);
pub const C2: Position = Position::new(1, 2);
pub const C3: Position = Position::new(2, 2);
pub const C4: Position = Position::new(3, 2);
pub const C5: Position = Position::new(4, 2);
pub const C6: Position = Position::new(5, 2);
pub const C7: Position = Position::new(6, 2);
pub const C8: Position = Position::new(7, 2);

pub const D1: Position = Position::new(0, 3);
pub const D2: Position = Position::new(1, 3);
pub const D3: Position = Position::new(2, 3);
pub const D4: Position = Position::new(3, 3);
pub const D5: Position = Position::new(4, 3);
pub const D6: Position = Position::new(5, 3);
pub const D7: Position = Position::new(6, 3);
pub const D8: Position = Position::new(7, 3);

pub const E1: Position = Position::new(0, 4);
pub const E2: Position = Position::new(1, 4);
pub const E3: Position = Position::new(2, 4);
pub const E4: Position = Position::new(3, 4);
pub const E5: Position = Position::new(4, 4);
pub const E6: Position = Position::new(5, 4);
pub const E7: Position = Position::new(6, 4);
pub const E8: Position = Position::new(7, 4);

pub const F1: Position = Position::new(0, 5);
pub const F2: Position = Position::new(1, 5);
pub const F3: Position = Position::new(2, 5);
pub const F4: Position = Position::new(3, 5);
pub const F5: Position = Position::new(4, 5);
pub const F6: Position = Position::new(5, 5);
pub const F7: Position = Position::new(6, 5);
pub const F8: Position = Position::new(7, 5);

pub const G1: Position = Position::new(0, 6);
pub const G2: Position = Position::new(1, 6);
pub const G3: Position = Position::new(2, 6);
pub const G4: Position = Position::new(3, 6);
pub const G5: Position = Position::new(4, 6);
pub const G6: Position = Position::new(5, 6);
pub const G7: Position = Position::new(6, 6);
pub const G8: Position = Position::new(7, 6);

pub const H1: Position = Position::new(0, 7);
pub const H2: Position = Position::new(1, 7);
pub const H3: Position = Position::new(2, 7);
pub const H4: Position = Position::new(3, 7);
pub const H5: Position = Position::new(4, 7);
pub const H6: Position = Position::new(5, 7);
pub const H7: Position = Position::new(6, 7);
pub const H8: Position = Position::new(7, 7);

/// ## Position
///
/// Defines a position on the chess board
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Position {
    row: i32,
    col: i32,
}

impl core::fmt::Display for Position {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> Result<(), core::fmt::Error> {
        write!(
            f,
            "{}{}",
            match self.col {
                0 => 'a',
                1 => 'b',
                2 => 'c',
                3 => 'd',
                4 => 'e',
                5 => 'f',
                6 => 'g',
                7 => 'h',
                _ => '?',
            },
            self.row + 1
        )
    }
}

impl Position {
    /// ### new
    ///
    /// Create a `Position` from its respective row or column number.
    /// The row and column numbers can be any of 0, 1, 2, 3, 4, 5, 6, or 7.
    ///
    /// Examples:
    /// - `A1 = Position::new(0, 0)`
    /// - `A8 = Position::new(7, 0)`
    /// - `H1 = Position::new(0, 7)`
    /// - `H8 = Position::new(7, 7)`
    #[inline]
    pub const fn new(row: i32, col: i32) -> Self {
        Self { row, col }
    }

    /// ### king_pos
    ///
    /// Return the starting position for a given color's king.
    #[inline]
    pub const fn king_pos(color: Color) -> Self {
        match color {
            WHITE => Self::new(0, 4),
            BLACK => Self::new(7, 4),
        }
    }

    /// ### queen_pos
    ///
    /// Return the starting position for a given color's queen.
    #[inline]
    pub const fn queen_pos(color: Color) -> Self {
        match color {
            WHITE => Self::new(0, 3),
            BLACK => Self::new(7, 3),
        }
    }

    /// is_on_board
    ///
    /// Is this position a valid spot on the board?
    #[inline]
    pub fn is_on_board(&self) -> bool {
        !self.is_off_board()
    }

    /// ### is_off_board
    ///
    /// Is this position NOT a valid spot on the board?
    #[inline]
    pub fn is_off_board(&self) -> bool {
        self.row < 0 || self.row > 7 || self.col < 0 || self.col > 7
    }

    /// ### get_row
    ///
    /// Get the row number of the position.
    /// This can be any of 0, 1, 2, 3, 4, 5, 6, or 7.
    #[inline]
    pub fn get_row(&self) -> i32 {
        self.row
    }

    /// ### get_col
    ///
    /// Get column index from 0 to 7
    #[inline]
    pub fn get_col(&self) -> i32 {
        self.col
    }

    /// ### add_row
    ///
    /// Increment row by drow
    #[inline]
    fn add_row(&self, drow: i32) -> Self {
        let mut result = *self;
        result.row += drow;
        result
    }

    /// ### add_col
    ///
    /// Incrrement column by dcol
    #[inline]
    fn add_col(&self, dcol: i32) -> Self {
        let mut result = *self;
        result.col += dcol;
        result
    }

    /// ### is_diagonal_to
    ///
    /// Is this position diagonal to another position?
    #[inline]
    pub fn is_diagonal_to(&self, other: Self) -> bool {
        // Algorithm for determining whether or not two squares are diagonal
        // <https://math.stackexchange.com/questions/1194565/how-to-know-if-two-points-are-diagonally-aligned>
        (self.col - other.col).abs() == (self.row - other.row).abs()
    }

    /// ### diagonal_distance
    ///
    /// Get the diagonal distance between two positions
    #[inline]
    fn diagonal_distance(&self, other: Self) -> i32 {
        (self.col - other.col).abs()
    }
    /// ### is_orthogonal_to
    ///
    /// Is this position orthogonal to another position?
    #[inline]
    pub fn is_orthogonal_to(&self, other: Self) -> bool {
        (self.col == other.col) || (self.row == other.row)
    }

    /// ### orthogonal_distance
    ///
    /// Get the orthogonal distance between two positions
    #[inline]
    fn orthogonal_distance(&self, other: Self) -> i32 {
        (self.col - other.col).abs() + (self.row - other.row).abs()
    }

    /// ### is_adjacent_to
    ///
    /// Is this position adjacent to another position?
    ///
    /// Adjacent positions have either:
    /// 1. A diagonal distance of one from each other
    /// 2. An orthogonal distance of one from each other
    #[inline]
    pub fn is_adjacent_to(&self, other: Self) -> bool {
        if self.is_orthogonal_to(other) {
            self.orthogonal_distance(other) == 1
        } else if self.is_diagonal_to(other) {
            self.diagonal_distance(other) == 1
        } else {
            false
        }
    }

    /// ### is_below
    ///
    /// Is this position beneath another position on the board?
    /// Pieces "beneath" other pieces on the board have lower ranks.
    ///
    /// So, for example, A7 is below A8.
    #[inline]
    pub fn is_below(&self, other: Self) -> bool {
        self.row < other.row
    }

    /// ### is_above
    ///
    /// Is this position above another position on the board?
    /// Pieces "above" other pieces on the board have higher ranks.
    ///
    /// So, for example, A8 is above A8.
    #[inline]
    pub fn is_above(&self, other: Self) -> bool {
        self.row > other.row
    }

    /// ### is_left_of
    ///
    /// Is this position left of another position on the board?
    /// Pieces "left of" other pieces on the board have a lower
    /// lexigraphical column character.
    ///
    /// So, for example, A8 is left of B8.
    #[inline]
    pub fn is_left_of(&self, other: Self) -> bool {
        self.col < other.col
    }

    /// ### is_right_of
    ///
    /// Is this position right of another position on the board?
    /// Pieces "right of" other pieces on the board have a higher
    /// lexigraphical column character.
    ///
    /// So, for example, B8 is right of A8.
    #[inline]
    pub fn is_right_of(&self, other: Self) -> bool {
        self.col > other.col
    }

    /// ### next_below
    ///
    /// Get the position directly below this position.
    ///
    /// IMPORTANT NOTE: This will NOT check for positions
    /// off of the board! You could easily get an invalid
    /// position if you do not check with the `is_on_board`
    /// method!
    #[inline]
    pub fn next_below(&self) -> Self {
        Self::new(self.row - 1, self.col)
    }

    /// ### next_above
    ///
    /// Get the position directly above this position.
    ///
    /// IMPORTANT NOTE: This will NOT check for positions
    /// off of the board! You could easily get an invalid
    /// position if you do not check with the `is_on_board`
    /// method!
    #[inline]
    pub fn next_above(&self) -> Self {
        Self::new(self.row + 1, self.col)
    }

    /// ### pawn_up
    ///
    /// Get the next square upwards from a respective player's
    /// pawn.
    ///
    /// IMPORTANT NOTE: This will NOT check for positions
    /// off of the board! You could easily get an invalid
    /// position if you do not check with the `is_on_board`
    /// method!
    #[inline]
    pub fn pawn_up(&self, ally_color: Color) -> Self {
        match ally_color {
            WHITE => self.next_above(),
            BLACK => self.next_below(),
        }
    }

    /// ### pawn_back
    ///
    /// Get the next square backwards from a respective player's
    /// pawn.
    ///
    /// IMPORTANT NOTE: This will NOT check for positions
    /// off of the board! You could easily get an invalid
    /// position if you do not check with the `is_on_board`
    /// method!
    #[inline]
    pub fn pawn_back(&self, ally_color: Color) -> Self {
        self.pawn_up(!ally_color)
    }
    /// ### next_left
    ///
    /// Get the position directly left of this position.
    ///
    /// IMPORTANT NOTE: This will NOT check for positions
    /// off of the board! You could easily get an invalid
    /// position if you do not check with the `is_on_board`
    /// method!
    #[inline]
    pub fn next_left(&self) -> Self {
        Self::new(self.row, self.col - 1)
    }

    /// ### next_right
    ///
    /// Get the position directly right of this position.
    ///
    /// IMPORTANT NOTE: This will NOT check for positions
    /// off of the board! You could easily get an invalid
    /// position if you do not check with the `is_on_board`
    /// method!
    #[inline]
    pub fn next_right(&self) -> Self {
        Self::new(self.row, self.col + 1)
    }

    /// ### is_starting_pawn
    ///
    /// Is this pawn on the starting rank for the respective player?
    #[inline]
    pub fn is_starting_pawn(&self, color: Color) -> bool {
        match color {
            WHITE => self.row == 1,
            BLACK => self.row == 6,
        }
    }

    /// ### is_kingside_rook
    ///
    /// Is this the starting position of the kingside rook?
    #[inline]
    pub fn is_kingside_rook(&self, color: Color) -> bool {
        match color {
            BLACK => self == &H8,
            WHITE => self == &H1,
        }
    }
    /// ### is_queenside_rook
    ///
    /// Is this the starting position of the queenside rook?
    #[inline]
    pub fn is_queenside_rook(&self, color: Color) -> bool {
        match color {
            BLACK => self == &A8,
            WHITE => self == &A1,
        }
    }

    /// ### diagonals_to
    ///
    /// Get the list of positions from this position to another
    /// position, moving diagonally.
    ///
    /// This does _not_ include the `from` position, and includes the `to` position.
    pub fn diagonals_to(&self, to: Self) -> Vec<Self> {
        if !self.is_diagonal_to(to) {
            return Vec::new();
        }

        let row_step;
        let col_step;
        if self.is_left_of(to) {
            col_step = 1;
        } else {
            col_step = -1;
        }

        if self.is_below(to) {
            row_step = 1;
        } else {
            row_step = -1;
        }

        let mut acc = *self;
        let mut result = Vec::new();
        for _ in 0..self.diagonal_distance(to) {
            acc = acc.add_row(row_step).add_col(col_step);
            result.push(acc);
        }

        result
    }

    /// ### orthogonals_to
    ///
    /// Get the list of positions from this position to another
    /// position, moving orthogonally.
    ///
    /// This does _not_ include the `from` position, and includes the `to` position.
    pub fn orthogonals_to(&self, to: Self) -> Vec<Self> {
        if !self.is_orthogonal_to(to) {
            return Vec::new();
        }
        let mut row_step = 0;
        let mut col_step = 0;
        if self.is_left_of(to) {
            col_step = 1;
        } else if self.is_right_of(to) {
            col_step = -1;
        } else if self.is_above(to) {
            row_step = -1;
        } else if self.is_below(to) {
            row_step = 1;
        }

        let mut acc = *self;
        let mut result = Vec::new();

        for _ in 0..self.orthogonal_distance(to) {
            acc = acc.add_row(row_step).add_col(col_step);
            result.push(acc);
        }

        result
    }

    /// ### is_knight_move
    ///
    /// Checks whether the provided position is a valid knight move
    #[inline]
    pub fn is_knight_move(&self, other: Self) -> bool {
        (self.row - other.row).abs() == 2 && (self.col - other.col).abs() == 1
            || (self.row - other.row).abs() == 1 && (self.col - other.col).abs() == 2
    }
}

impl FromStr for Position {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Check string length
        if s.len() != 2 {
            return Err("Invalid position");
        }
        // Convert string to uppercase
        let s: String = s.to_uppercase();
        // Get column
        let row: i32 = (s.chars().nth(1).unwrap().to_digit(10).unwrap_or(0) as i32) - 1;
        let col: i32 = match s.chars().next().unwrap() {
            'A' => 0,
            'B' => 1,
            'C' => 2,
            'D' => 3,
            'E' => 4,
            'F' => 5,
            'G' => 6,
            'H' => 7,
            _ => return Err("Invalid column"),
        };
        let position: Position = Position::new(row, col);
        if position.is_on_board() {
            Ok(position)
        } else {
            Err("Invalid row")
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;

    use pretty_assertions::assert_eq;

    #[test]
    fn new() {
        let position: Position = Position::new(0, 4);
        assert_eq!(position.row, 0);
        assert_eq!(position.col, 4);
        // queen pos
        let position: Position = Position::queen_pos(WHITE);
        assert_eq!(position.row, 0);
        assert_eq!(position.col, 3);
        let position: Position = Position::queen_pos(BLACK);
        assert_eq!(position.row, 7);
        assert_eq!(position.col, 3);
        // king pos
        let position: Position = Position::king_pos(WHITE);
        assert_eq!(position.row, 0);
        assert_eq!(position.col, 4);
        let position: Position = Position::king_pos(BLACK);
        assert_eq!(position.row, 7);
        assert_eq!(position.col, 4);
    }

    #[test]
    fn getters() {
        assert_eq!(D5.get_col(), 3);
        assert_eq!(D5.get_row(), 4);
    }

    #[test]
    fn add_col() {
        assert_eq!(D5.add_col(2), F5);
    }

    #[test]
    fn add_row() {
        assert_eq!(A2.add_row(3), A5);
    }

    #[test]
    fn is_diagonal_to() {
        assert_eq!(A2.is_diagonal_to(E6), true);
        assert_eq!(A2.is_diagonal_to(F7), true);
        assert_eq!(H1.is_diagonal_to(A8), true);
        assert_eq!(A2.is_diagonal_to(F8), false);
        assert_eq!(A2.is_diagonal_to(A8), false);
    }

    #[test]
    fn diagonal_distance() {
        assert_eq!(A2.diagonal_distance(E6), 4);
        assert_eq!(C3.diagonal_distance(Position::new(-1, -1)), 3); // Negative distance
        assert_eq!(
            Position::new(-1, -1).diagonal_distance(Position::new(-4, -4)),
            3
        ); // Negative distance
    }

    #[test]
    fn is_orthogonal() {
        assert_eq!(A2.is_orthogonal_to(A6), true);
        assert_eq!(A2.is_orthogonal_to(F2), true);
        assert_eq!(A2.is_orthogonal_to(C8), false);
    }

    #[test]
    fn orthogonal_distance() {
        assert_eq!(A2.orthogonal_distance(A6), 4);
        assert_eq!(A2.orthogonal_distance(F2), 5);
        assert_eq!(
            Position::new(0, -5).orthogonal_distance(Position::new(0, -10)),
            5
        ); // Negative
    }

    #[test]
    fn is_adjacent_to() {
        assert_eq!(D4.is_adjacent_to(C3), true);
        assert_eq!(D4.is_adjacent_to(C4), true);
        assert_eq!(D4.is_adjacent_to(C5), true);
        assert_eq!(D4.is_adjacent_to(D5), true);
        assert_eq!(D4.is_adjacent_to(E5), true);
        assert_eq!(D4.is_adjacent_to(E4), true);
        assert_eq!(D4.is_adjacent_to(E3), true);
        assert_eq!(D4.is_adjacent_to(D3), true);
        assert_eq!(D4.is_adjacent_to(D2), false);
    }

    #[test]
    fn is_below() {
        assert_eq!(D4.is_below(D5), true);
        assert_eq!(D4.is_below(D8), true);
        assert_eq!(D4.is_below(G8), true);
        assert_eq!(D4.is_below(D4), false);
        assert_eq!(D4.is_below(A1), false);
    }

    #[test]
    fn is_above() {
        assert_eq!(D4.is_above(D2), true);
        assert_eq!(D4.is_above(A1), true);
        assert_eq!(D4.is_above(G3), true);
        assert_eq!(D4.is_above(D4), false);
        assert_eq!(D4.is_above(A8), false);
    }

    #[test]
    fn is_left_of() {
        assert_eq!(D4.is_left_of(F4), true);
        assert_eq!(D4.is_left_of(E8), true);
        assert_eq!(D4.is_left_of(D6), false);
        assert_eq!(D4.is_left_of(D4), false);
        assert_eq!(D4.is_left_of(A1), false);
    }

    #[test]
    fn is_right_of() {
        assert_eq!(D4.is_right_of(C4), true);
        assert_eq!(D4.is_right_of(C1), true);
        assert_eq!(D4.is_right_of(D7), false);
        assert_eq!(D4.is_right_of(D4), false);
        assert_eq!(D4.is_right_of(H8), false);
    }

    #[test]
    fn next_below() {
        assert_eq!(D4.next_below(), D3);
    }

    #[test]
    fn next_above() {
        assert_eq!(D4.next_above(), D5);
    }

    #[test]
    fn pawn_up() {
        assert_eq!(D2.pawn_up(WHITE), D3);
        assert_eq!(D7.pawn_up(BLACK), D6);
    }

    #[test]
    fn pawn_back() {
        assert_eq!(D3.pawn_back(WHITE), D2);
        assert_eq!(D6.pawn_back(BLACK), D7);
    }

    #[test]
    fn next_left() {
        assert_eq!(D3.next_left(), C3);
    }

    #[test]
    fn next_right() {
        assert_eq!(D3.next_right(), E3);
    }

    #[test]
    fn is_starting_pawn() {
        assert_eq!(A2.is_starting_pawn(WHITE), true);
        assert_eq!(B2.is_starting_pawn(WHITE), true);
        assert_eq!(C2.is_starting_pawn(WHITE), true);
        assert_eq!(D2.is_starting_pawn(WHITE), true);
        assert_eq!(E2.is_starting_pawn(WHITE), true);
        assert_eq!(F2.is_starting_pawn(WHITE), true);
        assert_eq!(G2.is_starting_pawn(WHITE), true);
        assert_eq!(H2.is_starting_pawn(WHITE), true);
        assert_eq!(A7.is_starting_pawn(BLACK), true);
        assert_eq!(B7.is_starting_pawn(BLACK), true);
        assert_eq!(C7.is_starting_pawn(BLACK), true);
        assert_eq!(D7.is_starting_pawn(BLACK), true);
        assert_eq!(E7.is_starting_pawn(BLACK), true);
        assert_eq!(F7.is_starting_pawn(BLACK), true);
        assert_eq!(G7.is_starting_pawn(BLACK), true);
        assert_eq!(H7.is_starting_pawn(BLACK), true);
        // -- bad
        assert_eq!(D4.is_starting_pawn(WHITE), false);
        assert_eq!(D6.is_starting_pawn(BLACK), false);
    }

    #[test]
    fn is_kingside_rook() {
        assert_eq!(H1.is_kingside_rook(WHITE), true);
        assert_eq!(H1.is_kingside_rook(BLACK), false);
        assert_eq!(H8.is_kingside_rook(WHITE), false);
        assert_eq!(H8.is_kingside_rook(BLACK), true);
    }

    #[test]
    fn is_queenside_rook() {
        assert_eq!(A1.is_queenside_rook(WHITE), true);
        assert_eq!(A1.is_queenside_rook(BLACK), false);
        assert_eq!(A8.is_queenside_rook(WHITE), false);
        assert_eq!(A8.is_queenside_rook(BLACK), true);
    }

    #[test]
    fn diagonals_to() {
        assert_eq!(D4.diagonals_to(H8), vec![E5, F6, G7, H8]);
        assert_eq!(G8.diagonals_to(B3), vec![F7, E6, D5, C4, B3]);
        assert_eq!(A7.diagonals_to(F2), vec![B6, C5, D4, E3, F2]);
        assert_eq!(H1.diagonals_to(A8), vec![G2, F3, E4, D5, C6, B7, A8]);
        assert_eq!(H1.diagonals_to(A1), vec![]); // Not diagonal to
    }

    #[test]
    fn orthogonals_to() {
        assert_eq!(D4.orthogonals_to(H4), vec![E4, F4, G4, H4]);
        assert_eq!(A1.orthogonals_to(H1), vec![B1, C1, D1, E1, F1, G1, H1]);
        assert_eq!(B1.orthogonals_to(B8), vec![B2, B3, B4, B5, B6, B7, B8]);
        assert_eq!(D8.orthogonals_to(D1), vec![D7, D6, D5, D4, D3, D2, D1]);
        assert_eq!(A1.orthogonals_to(H8), vec![]); // Non-orthogonal
    }

    #[test]
    fn on_off_board() {
        // -- on board
        assert_eq!(A4.is_on_board(), true);
        assert_eq!(A4.is_off_board(), false);
        assert_eq!(A1.is_on_board(), true);
        assert_eq!(A1.is_off_board(), false);
        assert_eq!(H8.is_on_board(), true);
        assert_eq!(H8.is_off_board(), false);
        // -- off board
        assert_eq!(Position::new(-1, 4).is_on_board(), false);
        assert_eq!(Position::new(-1, 4).is_off_board(), true);
        assert_eq!(Position::new(0, -4).is_on_board(), false);
        assert_eq!(Position::new(0, -4).is_off_board(), true);
        assert_eq!(Position::new(8, 4).is_on_board(), false);
        assert_eq!(Position::new(8, 4).is_off_board(), true);
        assert_eq!(Position::new(2, 14).is_on_board(), false);
        assert_eq!(Position::new(2, 14).is_off_board(), true);
    }

    #[test]
    fn knight_move() {
        // Knight at d5; moves are b6, c3, e7, f6, f4, e3, c3, b4
        assert_eq!(D5.is_knight_move(B6), true);
        assert_eq!(D5.is_knight_move(C3), true);
        assert_eq!(D5.is_knight_move(E7), true);
        assert_eq!(D5.is_knight_move(F6), true);
        assert_eq!(D5.is_knight_move(F4), true);
        assert_eq!(D5.is_knight_move(E3), true);
        assert_eq!(D5.is_knight_move(C3), true);
        assert_eq!(D5.is_knight_move(B4), true);
        // bad moves
        assert_eq!(D5.is_knight_move(D6), false);
        assert_eq!(D5.is_knight_move(A1), false);
        assert_eq!(D5.is_knight_move(F3), false);
    }

    #[test]
    fn position_from_str() {
        assert_eq!(Position::from_str("A1").ok().unwrap(), A1);
        assert_eq!(Position::from_str("h8").ok().unwrap(), H8);
        assert_eq!(Position::from_str("a8").ok().unwrap(), A8);
        assert_eq!(Position::from_str("H1").ok().unwrap(), H1);
        assert_eq!(Position::from_str("C6").ok().unwrap(), C6);
        assert!(Position::from_str("a0").is_err());
        assert!(Position::from_str("A9").is_err());
        assert!(Position::from_str("J5").is_err());
        assert!(Position::from_str("a01").is_err());
    }
}
