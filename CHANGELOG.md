# Changelog

- [Changelog](#changelog)
  - [0.2.0](#020)

## 0.2.0

Released on ??

- Linted rust code with `clippy`
- Added test units and CI steps
- Better function documentation
- Bugfix: 🐛
  - Fixed a bug where you couldn't take both up left and up right pawns (e.g. white `e4`, black `d5` and `f5`; you could take only `d5`)
  - Fixed a bug where you couldn't castle both on kingside and queenside at the same time
  - Fixed typo in move parser for `kingside castle` (See PR <https://github.com/adam-mcdaniel/chess-engine/pull/11>)
- Renamed `print_rating_bar` to `get_rating`. Now it returns the scores as percentage `(white, black)` and not a string
