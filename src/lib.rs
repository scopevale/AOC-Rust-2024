// Use this file to add helper functions and additional modules.
pub mod template;

// use std::cmp::Ordering;

// AOC 2024 - Day 02
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
