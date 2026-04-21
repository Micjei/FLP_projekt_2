// ============================================================
// dungeon-scribe — reachability analysis
//
// You must implement the function body below.
// You may add private helper functions.
// ============================================================

use crate::model::{DungeonMap, Position};

/// Returns the number of passable tiles reachable from `start` via
/// 4-directional movement (up, down, left, right), without crossing walls.
///
/// A tile is passable if `Tile::is_passable()` returns `true` for it.
/// The starting tile itself is counted if it is passable.
///
/// # Returns
///
/// - `0` if `start` is out of bounds.
/// - `0` if the tile at `start` is not passable (i.e. is a `Wall`).
/// - Otherwise, the total number of passable tiles in the connected region
///   that contains `start`, including `start` itself.
///
/// # Implementation notes
///
/// A standard BFS or DFS is the expected approach:
/// - Use a `VecDeque<Position>` (BFS) or `Vec<Position>` (DFS) as the frontier.
/// - Use a `HashSet<Position>` to track visited positions.
/// - `Position` is `(usize, usize)` which is `Copy`, so you can push positions
///   into the queue without lifetime issues.
///
/// # Warning — usize subtraction
///
/// `Position` uses `usize`. Subtracting from `0_usize` causes a panic.
/// When generating the four neighbours of a position, guard against
/// `row == 0` and `col == 0` **before** subtracting.
pub fn reachable_floor_size(map: &DungeonMap, start: Position) -> usize {
    let mut visited = std::collections::HashSet::new();
    let mut queue = std::collections::VecDeque::new();

    // start out of bounds
    if start.0 >= map.height() || start.1 >= map.width() {
        return 0;
    }

    // start is not passable
    let start_tile = match map.get(start) {
        Some(tile) => tile,
        None => return 0,
    };

    if !start_tile.is_passable() {
        return 0;
    }

    visited.insert(start);
    queue.push_back(start);

    // start BFS
    while let Some(pos) = queue.pop_front() {
        let (row, col) = pos;
        let neighbors = [
            (row.wrapping_sub(1), col), // up
            (row + 1, col),             // down
            (row, col.wrapping_sub(1)), // left
            (row, col + 1),             // right
        ];

        for neighbor in neighbors {
            if neighbor.0 < map.height() && neighbor.1 < map.width() {
                let neighbor_tile = match map.get(neighbor) {
                    Some(tile) => tile,
                    None => continue,
                };
                if neighbor_tile.is_passable() && !visited.contains(&neighbor) {
                    visited.insert(neighbor);
                    queue.push_back(neighbor);
                }
            }
        }
    }

    visited.len()
}

// ── Unit tests ────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::DungeonMap;

    // Add your own tests here if needed.

    #[test]
    fn wall_start_returns_zero() {
        let map = DungeonMap::parse("###\n#@#\n###").unwrap();
        // (0, 0) is a Wall
        assert_eq!(reachable_floor_size(&map, (0, 0)), 0);
    }

    #[test]
    fn out_of_bounds_start_returns_zero() {
        let map = DungeonMap::parse("###\n#@#\n###").unwrap();
        assert_eq!(reachable_floor_size(&map, (99, 99)), 0);
    }
}
