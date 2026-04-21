// ============================================================
// dungeon-scribe — DungeonMap and Position
//
// The struct definition and all method signatures are provided.
// You must implement every method body that contains `todo!()`.
//
// You may add private helper methods freely.
// Do not change any public signature.
// ============================================================

use std::collections::HashMap;

use crate::error::{ParseError, ValidationError};
use crate::tile::Tile;

/// A position in the dungeon map, represented as `(row, col)`.
///
/// Both `row` and `col` are 0-based.
pub type Position = (usize, usize);

/// A successfully parsed dungeon map.
///
/// The grid is stored as a `Vec` of rows, each row being a `Vec<Tile>`.
/// `width` is the number of columns; `height` is the number of rows.
#[derive(Debug, PartialEq)]
pub struct DungeonMap {
    tiles: Vec<Vec<Tile>>,
    width: usize,
    height: usize,
}

impl DungeonMap {
    // ── Parsing ──────────────────────────────────────────────────────────────

    /// Parse a multi-line `&str` into a `DungeonMap`.
    ///
    /// Each line of the input corresponds to one row of the map.
    /// Lines are split on `'\n'`; any trailing `'\r'` is stripped first so
    /// that both Unix (`\n`) and Windows (`\r\n`) files are accepted.
    ///
    /// # Errors
    ///
    /// - `ParseError::EmptyInput` — input has no non-empty lines.
    ///   Lines that contain only whitespace also count as empty.
    /// - `ParseError::JaggedMap` — a row has a different width from row 0.
    ///   The error carries the 0-based index of the offending row, the
    ///   expected width (from row 0), and the actual width found.
    /// - `ParseError::UnknownTile` — a character is not recognised.
    ///   The error carries the character and its `(row, col)` position.
    ///
    /// # Hint
    ///
    /// The idiomatic Rust approach is to use `lines().enumerate()` and then
    /// `chars().enumerate()` with `map` and `collect::<Result<Vec<_>, _>>()`.
    /// This pattern propagates errors naturally while building the
    /// `Vec<Vec<Tile>>`.
    pub fn parse(input: &str) -> Result<DungeonMap, ParseError> {
        if input.trim().is_empty() {
            return Err(ParseError::EmptyInput);
        }

        let grid: Vec<Vec<Tile>> = input
            .lines()
            .enumerate()
            .map(|(row_idx, line)| {
                let row: Vec<Tile> = line
                    .chars()
                    .enumerate()
                    .map(|(col_idx, c)| {
                        Tile::from_char(c).map_err(|e| {
                            if let ParseError::UnknownTile { c, .. } = e {
                                ParseError::UnknownTile {
                                    c,
                                    row: row_idx,
                                    col: col_idx,
                                }
                            } else {
                                e
                            }
                        })
                    })
                    .collect::<Result<Vec<_>, _>>()?;

                Ok((row_idx, row))
            })
            .collect::<Result<Vec<_>, ParseError>>()?
            .into_iter()
            .map(|(_, row)| row)
            .collect();

        let height = grid.len();
        let width = grid.first().map_or(0, |r| r.len());

        for (i, row) in grid.iter().enumerate() {
            if row.len() != width {
                return Err(ParseError::JaggedMap {
                    row: i,
                    expected: width,
                    found: row.len(),
                });
            }
        }

        Ok(DungeonMap {
            tiles: grid,
            width,
            height,
        })
    }

    // ── Accessors ─────────────────────────────────────────────────────────────

    /// Returns a reference to the tile at `pos = (row, col)`.
    ///
    /// Returns `None` if the position is out of bounds.
    /// The returned reference has the same lifetime as `&self`.
    pub fn get(&self, pos: Position) -> Option<&Tile> {
        //todo!()
        let (row, col) = pos;
        if row < self.height && col < self.width {
            Some(&self.tiles[row][col])
        } else {
            None
        }
    }

    /// Returns the width of the map (number of columns).
    pub fn width(&self) -> usize {
        self.width
    }

    /// Returns the height of the map (number of rows).
    pub fn height(&self) -> usize {
        self.height
    }

    /// Returns an iterator over the rows of the map.
    ///
    /// Each item is a reference to one row (`&Vec<Tile>`).
    ///
    /// Note: returning `impl Iterator` from a method requires understanding
    /// that the iterator borrows from `&self`.
    pub fn rows(&self) -> impl Iterator<Item = &Vec<Tile>> {
        // Replace this body with your implementation.
        // std::iter::empty() is here only so the skeleton compiles.
        self.tiles.iter()
    }

    // ── Analysis ──────────────────────────────────────────────────────────────

    /// Returns a `HashMap` mapping each `Tile` variant to its count.
    ///
    /// Only tile types that appear at least once are included as keys.
    /// Absent tile types must **not** appear with a count of `0`.
    ///
    /// # Hint
    ///
    /// `self.rows().flat_map(|row| row.iter())` gives you a flat iterator
    /// over every tile in the map.
    pub fn count_tiles(&self) -> HashMap<Tile, usize> {
        //todo!()
        let mut counts = HashMap::new();
        for row in self.rows().flat_map(|row| row.iter()) {
            match counts.get(row) {
                Some(count) => counts.insert(*row, count + 1),
                None => counts.insert(*row, 1),
            };
        }
        counts
    }

    /// Returns all positions where `tile` appears, sorted row-first then
    /// column (i.e. in reading order).
    ///
    /// Returns an empty `Vec` if the tile does not appear in the map.
    pub fn find_all(&self, tile: Tile) -> Vec<Position> {
        //todo!()
        let mut positions = Vec::new();
        for (row_idx, row) in self.rows().enumerate() {
            for (col_idx, t) in row.iter().enumerate() {
                if *t == tile {
                    positions.push((row_idx, col_idx));
                }
            }
        }
        positions
    }

    // ── Validation ────────────────────────────────────────────────────────────

    /// Validates the structural integrity of the map.
    ///
    /// Returns `Ok(())` if and only if all of the following hold:
    /// - Exactly one `PlayerStart` tile exists.
    /// - At least one `Exit` tile exists.
    /// - At least one `Floor` tile exists.
    ///
    /// Otherwise returns `Err(errors)` where `errors` is a `Vec` containing
    /// **every** applicable `ValidationError`.
    ///
    /// # Important
    ///
    /// Unlike most `Result`-returning functions, this one must **accumulate
    /// all errors** before returning. Do not use `?` for early return here.
    /// Build a `Vec<ValidationError>`, push to it for each failing condition,
    /// and return `Err(errors)` at the end if it is non-empty.
    pub fn validate(&self) -> Result<(), Vec<ValidationError>> {
        //todo!()
        let player_starts = self.find_all(Tile::PlayerStart);
        let exits = self.find_all(Tile::Exit);
        let floors = self.find_all(Tile::Floor);
        let mut errors = Vec::new();

        if player_starts.is_empty() {
            errors.push(ValidationError::MissingPlayerStart);
        }

        if player_starts.len() > 1 {
            errors.push(ValidationError::MultiplePlayerStarts(player_starts));
        }

        if exits.is_empty() {
            errors.push(ValidationError::NoExit);
        }

        if floors.is_empty() {
            errors.push(ValidationError::NoFloor);
        }

        if !errors.is_empty() {
            return Err(errors);
        }

        Ok(())
    }
}

// ── Unit tests ────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // Add your own tests here.

    #[test]
    fn parse_empty_input_returns_error() {
        assert!(DungeonMap::parse("").is_err());
    }

    #[test]
    fn parse_single_row_map() {
        let map = DungeonMap::parse("###").unwrap();
        assert_eq!(map.width(), 3);
        assert_eq!(map.height(), 1);
    }
}
