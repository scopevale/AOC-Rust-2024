advent_of_code::solution!(6);

use std::collections::HashSet;

type Grid = Vec<Vec<u8>>; // b'.' empty, b'#' obstacle
type Pos = (i32, i32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Dir {
    N = 0,
    E = 1,
    S = 2,
    W = 3,
}

impl Dir {
    #[inline]
    fn right(self) -> Self {
        match self {
            Dir::N => Dir::E,
            Dir::E => Dir::S,
            Dir::S => Dir::W,
            Dir::W => Dir::N,
        }
    }
    #[inline]
    fn delta(self) -> (i32, i32) {
        match self {
            Dir::N => (-1, 0),
            Dir::E => (0, 1),
            Dir::S => (1, 0),
            Dir::W => (0, -1),
        }
    }
}

#[derive(Debug)]
enum SimOutcome {
    Exited { visited: HashSet<Pos> }, // left the map
    Looped,                           // repeated a (pos,dir) state
}

#[derive(Debug)]
struct Problem {
    grid: Grid,
    rows: i32,
    cols: i32,
    start: Pos,
    dir0: Dir,
}

fn parse(input: &str) -> Problem {
    let mut grid = Vec::new();
    let mut start: Option<(Pos, Dir)> = None;

    for (r, line) in input.lines().enumerate() {
        let mut row = Vec::with_capacity(line.len());
        for (c, ch) in line.bytes().enumerate() {
            let (rr, cc) = (r as i32, c as i32);
            match ch {
                b'^' => {
                    start = Some(((rr, cc), Dir::N));
                    row.push(b'.');
                }
                b'>' => {
                    start = Some(((rr, cc), Dir::E));
                    row.push(b'.');
                }
                b'v' => {
                    start = Some(((rr, cc), Dir::S));
                    row.push(b'.');
                }
                b'<' => {
                    start = Some(((rr, cc), Dir::W));
                    row.push(b'.');
                }
                b'#' => row.push(b'#'),
                b'.' => row.push(b'.'),
                other => panic!("Unexpected char {other:?} at ({r},{c})"),
            }
        }
        grid.push(row);
    }

    let rows = grid.len() as i32;
    let cols = grid.first().map(|r| r.len()).unwrap_or(0) as i32;
    let (start_pos, dir0) = start.expect("No start (^>v<) found");
    Problem {
        grid,
        rows,
        cols,
        start: start_pos,
        dir0,
    }
}

#[inline]
fn in_bounds(p: Pos, rows: i32, cols: i32) -> bool {
    p.0 >= 0 && p.0 < rows && p.1 >= 0 && p.1 < cols
}

#[inline]
fn is_blocked(p: Pos, prob: &Problem, extra_block: Option<Pos>) -> bool {
    if let Some(b) = extra_block {
        if p == b {
            return true;
        }
    }
    let (r, c) = (p.0 as usize, p.1 as usize);
    prob.grid[r][c] == b'#'
}

/// Simulate one run. Returns Exited (with all visited cells) or Looped.
/// - Turns right on obstacle.
/// - Moves one cell at a time.
/// - Counts a cell as visited **after moving into it**, but also includes the start.
fn simulate(prob: &Problem, extra_block: Option<Pos>) -> SimOutcome {
    let (rows, cols) = (prob.rows, prob.cols);
    let mut pos = prob.start;
    let mut dir = prob.dir0;

    let mut visited: HashSet<Pos> = HashSet::with_capacity(1 << 16);
    visited.insert(pos);

    // For loop detection we must include the facing
    let mut seen: HashSet<(Pos, Dir)> = HashSet::with_capacity(1 << 16);

    loop {
        // If we have seen the exact (pos,dir), it’s a loop
        if !seen.insert((pos, dir)) {
            return SimOutcome::Looped;
        }

        let (dr, dc) = dir.delta();
        let next = (pos.0 + dr, pos.1 + dc);

        // Exiting the map?
        if !in_bounds(next, rows, cols) {
            return SimOutcome::Exited { visited };
        }

        // Obstacle (either original # or temporary extra_block)?
        if is_blocked(next, prob, extra_block) {
            dir = dir.right();
            continue; // turn but do not move
        }

        // Move ahead
        pos = next;
        visited.insert(pos);
    }
}

/// Part 1: number of distinct cells visited before exiting.
fn part_one(input: &str) -> Option<u64> {
    let prob = parse(input);
    match simulate(&prob, None) {
        SimOutcome::Exited { visited } => Some(visited.len() as u64),
        SimOutcome::Looped => panic!("Unexpected loop in part 1"),
    }
}

/// Part 2: number of **single extra obstacle placements** that cause a loop.
/// Pruned: only consider placing on cells from the original path (excluding start).
fn part_two(input: &str) -> Option<u64> {
    let prob = parse(input);
    let path_cells: HashSet<Pos> = match simulate(&prob, None) {
        SimOutcome::Exited { visited } => visited,
        SimOutcome::Looped => panic!("Unexpected loop in baseline run"),
    };

    let mut count = 0usize;

    for &cand in &path_cells {
        if cand == prob.start {
            continue;
        } // cannot place on start
          // skip if original map already has a wall (shouldn’t happen since path excludes walls)
        let (r, c) = (cand.0 as usize, cand.1 as usize);
        if prob.grid[r][c] == b'#' {
            continue;
        }

        match simulate(&prob, Some(cand)) {
            SimOutcome::Looped => count += 1,
            SimOutcome::Exited { .. } => {} // not a looper
        }
    }

    Some(count as u64)
}

/* ----------------------- tiny smoke tests ----------------------- */
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn tiny_exit() {
        // Starts facing up at (1,1); will turn right on the top edge and eventually exit.
        let input = "\
.....
.^#..
.....";
        let n = part_one(input);
        assert!(n > Some(0));
    }

    #[test]
    fn loop_detection_with_block() {
        // A tiny loop is possible by placing an extra obstacle on the path.
        let input = "\
.......
.^...#.
.......
";
        // Ensure part2 runs and returns a finite number
        let _ = part_two(input);
    }
}
