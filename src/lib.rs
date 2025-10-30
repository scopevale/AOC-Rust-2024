// Use this file to add helper functions and additional modules.
pub mod template;

/// AOC 2024 - Day 02
pub fn is_safe_monotonic(nums: &[i64]) -> bool {
    if nums.len() < 2 {
        return false; // need at least one adjacent pair
    }

    // Establish initial direction from the first pair
    let d0 = nums[1] - nums[0];
    if d0 == 0 || !(1..=3).contains(&d0.abs()) {
        return false; // equal or step > 3 fails immediately
    }

    let dir = d0.signum(); // +1 for increasing, -1 for decreasing

    // Check all subsequent adjacent pairs
    for w in nums.windows(2) {
        let d = w[1] - w[0];
        if d.signum() != dir {
            return false; // direction changed → fail
        }
        let step = d.abs();
        if step == 0 || step > 3 {
            return false; // equal or gap too big → fail
        }
    }

    true
}

fn is_step_ok(d: i64, dir: i64) -> bool {
    d.signum() == dir && d != 0 && d.abs() <= 3
}

// Check if nums is monotonic in `dir` (+1 increasing, -1 decreasing) with ≤1 removal.
fn safe_with_one_removal_dir(nums: &[i64], dir: i64) -> bool {
    let n = nums.len();
    if n < 2 {
        return false;
    }

    let mut removed = false;
    // `prev` is the last element we decided to *keep*.
    let mut prev = nums[0];

    for i in 1..n {
        let d = nums[i] - prev;

        if is_step_ok(d, dir) {
            // Normal advance.
            prev = nums[i];
            continue;
        }

        if removed {
            // Already used our one removal.
            return false;
        }

        // Try removing **current** element (skip nums[i]):
        // Keep `prev` and check prev → nums[i+1]
        let can_skip_curr = if i + 1 < n {
            let d2 = nums[i + 1] - prev;
            is_step_ok(d2, dir)
        } else {
            // Dropping the last element is always fine.
            true
        };

        // Try removing **previous** kept element:
        // Link nums[i-2] → nums[i] (or drop the very first element)
        let can_skip_prev = if i >= 2 {
            let prev2 = nums[i - 2];
            let d2 = nums[i] - prev2;
            is_step_ok(d2, dir)
        } else {
            // Dropping the first element is always fine.
            true
        };

        if !can_skip_curr && !can_skip_prev {
            return false;
        }

        removed = true;

        if can_skip_prev && !can_skip_curr {
            // Remove previous: we virtually connect nums[i-2] → nums[i].
            // After removing prev, the "kept last" becomes nums[i].
            prev = nums[i];
        } else if can_skip_curr && !can_skip_prev {
            // Remove current: keep `prev` and continue; do not advance `prev`.
            // (The next loop iteration will compare prev → nums[i+1].)
            continue;
        } else {
            // Both possible: prefer removing current (often safer for future steps).
            continue;
        }
    }

    true
}

// Accepts either direction with ≤1 removal.
fn is_safe_monotonic_with_one_removal(nums: &[i64]) -> bool {
    safe_with_one_removal_dir(nums, 1) || safe_with_one_removal_dir(nums, -1)
}

// Example glue: treat a line as safe if it's already safe OR can be made safe by removing one.
pub fn is_line_safe(nums: &[i64]) -> bool {
    is_safe_monotonic(nums) || is_safe_monotonic_with_one_removal(nums)
}

/// AOC 2024 - Day 04

/// Parse a rectangular char grid from a &str (all lines must have equal length).
fn parse_grid(input: &str) -> Result<Vec<Vec<char>>, String> {
    let lines: Vec<&str> = input.lines().collect();
    if lines.is_empty() {
        return Err("empty input".into());
    }
    let cols = lines[0].chars().count();
    if cols == 0 {
        return Err("lines must be non-empty".into());
    }
    let mut grid = Vec::with_capacity(lines.len());
    for (i, line) in lines.iter().enumerate() {
        let row: Vec<char> = line.chars().collect();
        if row.len() != cols {
            return Err(format!(
                "line {i} has length {}, expected {cols}",
                row.len()
            ));
        }
        grid.push(row);
    }
    Ok(grid)
}

/// Count occurrences of `word` in `grid`, allowing overlaps and all 8 directions.
fn count_word(grid: &[Vec<char>], word: &str) -> usize {
    if word.is_empty() {
        return 0;
    }
    let rows = grid.len();
    let cols = grid[0].len();
    let w: Vec<char> = word.chars().collect();
    let l = w.len();

    // Single-letter words: every matching cell counts.
    if l == 1 {
        let target = w[0];
        return grid.iter().flatten().filter(|&&ch| ch == target).count();
    }

    // 8 compass directions (dr, dc)
    const DIRS: [(isize, isize); 8] = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    let mut matches = 0usize;

    for r in 0..rows {
        for c in 0..cols {
            if grid[r][c] != w[0] {
                continue;
            }
            for (dr, dc) in DIRS {
                // Boundary precheck for the last character of the word along this ray.
                let end_r = r as isize + (l as isize - 1) * dr;
                let end_c = c as isize + (l as isize - 1) * dc;
                if end_r < 0 || end_c < 0 || end_r >= rows as isize || end_c >= cols as isize {
                    continue;
                }

                let mut ok = true;
                // We already matched k = 0 at (r, c)
                for k in 1..l {
                    let rr = r as isize + (k as isize) * dr;
                    let cc = c as isize + (k as isize) * dc;
                    // Safe due to boundary precheck
                    if grid[rr as usize][cc as usize] != w[k] {
                        ok = false;
                        break;
                    }
                }
                if ok {
                    matches += 1;
                }
            }
        }
    }

    matches
}

/// Convenience: parse + count directly from a &str input.
pub fn count_word_in_str(input: &str, word: &str) -> Result<usize, String> {
    let grid = parse_grid(input)?;
    Ok(count_word(&grid, word))
}

/// Check if `word_chars` (odd length) matches centered at (r,c) along the diagonal
/// given by unit direction (udr, udc), accepting either orientation around the center.
fn diag_matches_centered(
    grid: &[Vec<char>],
    word_chars: &[char],
    r: usize,
    c: usize,
    udr: isize,
    udc: isize,
    k: usize,
) -> bool {
    let rows = grid.len() as isize;
    let cols = grid[0].len() as isize;
    let r = r as isize;
    let c = c as isize;

    // Endpoints for the diagonal segment of length 2k+1 centered at (r,c)
    let end1_r = r - (k as isize) * udr;
    let end1_c = c - (k as isize) * udc;
    let end2_r = r + (k as isize) * udr;
    let end2_c = c + (k as isize) * udc;

    let in_bounds = |rr: isize, cc: isize| rr >= 0 && rr < rows && cc >= 0 && cc < cols;

    if !in_bounds(end1_r, end1_c) || !in_bounds(end2_r, end2_c) {
        return false;
    }

    let min_r = end1_r.min(end2_r);
    let max_r = end1_r.max(end2_r);
    let min_c = end1_c.min(end2_c);
    let max_c = end1_c.max(end2_c);

    if min_r < 0 || max_r >= rows || min_c < 0 || max_c >= cols {
        return false;
    }

    // Orientation 1: indices grow with distance from center (… L A G …)
    let mut ok1 = true;
    for t in -(k as isize)..=(k as isize) {
        let rr = r + t * udr;
        let cc = c + t * udc;
        let w_idx = (k as isize + t) as usize;
        if rr < 0 || rr >= rows || cc < 0 || cc >= cols {
            ok1 = false;
            break;
        }
        if grid[rr as usize][cc as usize] != word_chars[w_idx] {
            ok1 = false;
            break;
        }
    }
    if ok1 {
        return true;
    }

    // Orientation 2: reversed around center (… G A L …)
    let mut ok2 = true;
    for t in -(k as isize)..=(k as isize) {
        let rr = r + t * udr;
        let cc = c + t * udc;
        let w_idx = (k as isize - t) as usize;
        if rr < 0 || rr >= rows || cc < 0 || cc >= cols {
            ok2 = false;
            break;
        }
        if grid[rr as usize][cc as usize] != word_chars[w_idx] {
            ok2 = false;
            break;
        }
    }
    ok2
}

/// Count X-shaped matches of `word` (odd length) centered on its middle letter.
/// A valid X requires the word to appear on BOTH diagonals (NW–SE and NE–SW) around the center.
/// “+” shapes are ignored because we never check horizontal/vertical.
fn count_x_word(grid: &[Vec<char>], word: &str) -> usize {
    let l = word.chars().count();
    if l < 3 || l % 2 == 0 {
        return 0; // require odd length ≥ 3
    }
    let k = l / 2;
    let w: Vec<char> = word.chars().collect();

    let rows = grid.len();
    let cols = grid[0].len();
    let mut matches = 0usize;

    for r in 0..rows {
        for c in 0..cols {
            if grid[r][c] != w[k] {
                continue; // center must match middle letter
            }
            // Diagonals: NW–SE and NE–SW
            let a = diag_matches_centered(grid, &w, r, c, -1, -1, k); // NW–SE
            let b = diag_matches_centered(grid, &w, r, c, -1, 1, k); // NE–SW
            if a && b {
                matches += 1; // one per valid center
            }
        }
    }
    matches
}

/// Convenience: parse + count from &str.
pub fn count_x_word_in_str(input: &str, word: &str) -> Result<usize, String> {
    let grid = parse_grid(input)?;
    Ok(count_x_word(&grid, word))
}
