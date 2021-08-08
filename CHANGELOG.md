# Changelog

- [Changelog](#changelog)
  - [0.2.0](#020)

## 0.2.0

Released on ??

- **New features**:
  - Added `FromStr` for `Position` (e.g. `assert_eq!(Position::from_str("A1").ok().unwrap() == A1)`)
  - Added `get_taken_piece` to `Board` to get the last taken piece after a turn
  - Added `Promotion` to engine. (Before it just promoted pawn on last rank to queen, but that's incorrect, since sometimes can prevent checkmates and causing stalemates)
  - blah blah
- **API changes**:
  - Renamed `print_rating_bar` to `get_rating`. Now it returns the scores as percentage `(white, black)` and not a string
  - Renamed `get_turn_color` to `get_turn`
- **Bugfix 🐛**:
  - Fixed a bug where you couldn't take both up left and up right pawns (e.g. white `e4`, black `d5` and `f5`; you could take only `d5`)
  - Fixed a bug where you couldn't castle both on kingside and queenside at the same time
  - Fixed typo in move parser for `kingside castle` (See PR <https://github.com/adam-mcdaniel/chess-engine/pull/11>)
- **Linted** rust code with `clippy`
- Added test units and CI steps
- Better function **documentation**
