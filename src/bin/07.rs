use std::collections::HashSet;

advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u64> {
    let puzzles = parse_input(input).expect("unable to parse input file");

    let mut result = 0u128;

    for (_, puzzle) in puzzles.iter().enumerate() {
        if solvable(&puzzle.1, puzzle.0) {
            result += puzzle.0;
        } else {
            continue;
        }
    }

    Some(result.try_into().unwrap())
}

pub fn part_two(input: &str) -> Option<u64> {
    let puzzles = parse_input(input).expect("unable to parse input file");

    let mut result = 0u128;

    for (_, puzzle) in puzzles.iter().enumerate() {
        if solvable_dfs(&puzzle.1, puzzle.0) {
            result += puzzle.0;
        } else {
            continue;
        }
    }

    Some(result.try_into().unwrap())
}

/// Parse into target values & factors
pub fn parse_input(input: &str) -> Result<Vec<(u128, Vec<u128>)>, String> {
    let lines: Vec<&str> = input.lines().collect();
    if lines.is_empty() {
        return Err("empty input".into());
    }

    let mut result: Vec<(u128, Vec<u128>)> = vec![];
    for (_, line) in lines.iter().enumerate() {
        result.push(parse_line(line).expect("Unable to parse line"));
    }
    Ok(result)
}

fn parse_line(line: &str) -> Option<(u128, Vec<u128>)> {
    // Split at the colon
    let (left, right) = line.split_once(':')?; // returns Option<(str,str)>

    // Parse the left side as the target number
    let target = left.trim().parse::<u128>().ok()?;

    // Split the right side by whitespace and parse each number
    let nums: Vec<u128> = right
        .split_whitespace()
        .filter_map(|s| s.parse::<u128>().ok())
        .collect();

    Some((target, nums))
}

/// Evaluate left-to-right using a bit-mask where:
///   bit i (0-based) selects the operator between nums[i] and nums[i+1]
///   0 => '+', 1 => '*'
fn eval_mask(nums: &[u128], mask: u64) -> u128 {
    let mut acc = nums[0];
    for i in 0..(nums.len() - 1) {
        let next = nums[i + 1];
        if ((mask >> i) & 1) == 1 {
            acc = acc * next; // '*'
        } else {
            acc = acc + next; // '+'
        }
    }
    acc
}

/// Render the expression string corresponding to a mask, e.g. "81 * 40 + 27".
fn render_expr(nums: &[u128], mask: u64) -> String {
    let mut s = nums[0].to_string();
    for i in 0..(nums.len() - 1) {
        let op = if ((mask >> i) & 1) == 1 { '*' } else { '+' };
        s.push(' ');
        s.push(op);
        s.push(' ');
        s.push_str(&nums[i + 1].to_string());
    }
    s
}

/// Return the first solution as a formatted string, if any.
fn solve_first(nums: &[u128], target: u128) -> Option<String> {
    if nums.is_empty() {
        return None;
    }
    if nums.len() == 1 {
        return (nums[0] == target).then(|| nums[0].to_string());
    }

    let gaps = nums.len() - 1;
    let total = 1u64 << gaps;

    for mask in 0..total {
        if eval_mask(nums, mask) == target {
            return Some(render_expr(nums, mask));
        }
    }
    None
}

/// Return all solutions as formatted strings (possibly empty).
fn _solve_all(nums: &[u128], target: u128) -> Vec<String> {
    if nums.is_empty() {
        return vec![];
    }
    if nums.len() == 1 {
        return if nums[0] == target {
            vec![nums[0].to_string()]
        } else {
            vec![]
        };
    }

    let gaps = nums.len() - 1;
    let total = 1u64 << gaps;
    let mut out = Vec::new();

    for mask in 0..total {
        if eval_mask(nums, mask) == target {
            out.push(render_expr(nums, mask));
        }
    }
    out
}

/// Fast boolean: is there at least one mask that hits the target?
fn solvable(nums: &[u128], target: u128) -> bool {
    solve_first(nums, target).is_some()
}

#[derive(Copy, Clone, Debug)]
enum Op {
    Add,
    Mul,
    Concat,
}

#[inline]
fn digits10(mut x: u128) -> u32 {
    if x == 0 {
        return 1;
    }
    let mut d = 0;
    while x > 0 {
        d += 1;
        x /= 10;
    }
    d
}

#[inline]
fn pow10_u128(d: u32) -> u128 {
    let mut p = 1u128;
    for _ in 0..d {
        p = p.saturating_mul(10);
    }
    p
}

#[inline]
fn concat_u128(a: u128, b: u128) -> Option<u128> {
    let p = pow10_u128(digits10(b));
    a.checked_mul(p)?.checked_add(b)
}

#[inline]
fn apply_op(acc: u128, op: Op, next: u128) -> Option<u128> {
    match op {
        Op::Add => acc.checked_add(next),
        Op::Mul => acc.checked_mul(next),
        Op::Concat => concat_u128(acc, next),
    }
}

fn render(nums: &[u128], ops: &[Op]) -> String {
    let mut s = nums[0].to_string();
    for (i, op) in ops.iter().enumerate() {
        let sym = match op {
            Op::Add => " + ",
            Op::Mul => " * ",
            Op::Concat => " || ",
        };
        s.push_str(sym);
        s.push_str(&nums[i + 1].to_string());
    }
    s
}

/// First solution via DFS with pruning & memo (AoC-style: all nums > 0).
pub fn solve_first_dfs(nums: &[u128], target: u128) -> Option<String> {
    if nums.is_empty() {
        return None;
    }
    if nums.len() == 1 {
        return (nums[0] == target).then(|| nums[0].to_string());
    }

    // Memo of dead states: (index, acc) → no solution from here
    let mut dead: HashSet<(usize, u128)> = HashSet::new();
    let mut ops: Vec<Op> = Vec::with_capacity(nums.len().saturating_sub(1));

    fn dfs(
        i: usize,
        acc: u128,
        nums: &[u128],
        target: u128,
        dead: &mut HashSet<(usize, u128)>,
        ops: &mut Vec<Op>,
    ) -> bool {
        if i == nums.len() {
            return acc == target;
        }

        // Prune: all nums > 0 → any op strictly increases acc
        if acc > target {
            return false;
        }

        let key = (i, acc);
        if dead.contains(&key) {
            return false;
        }

        // Try ops in an arbitrary order; you can reorder if you like.
        for &op in [Op::Add, Op::Mul, Op::Concat].iter() {
            if let Some(next_acc) = apply_op(acc, op, nums[i]) {
                ops.push(op);
                if dfs(i + 1, next_acc, nums, target, dead, ops) {
                    return true;
                }
                ops.pop();
            }
        }

        dead.insert(key);
        false
    }

    let ok = dfs(1, nums[0], nums, target, &mut dead, &mut ops);
    ok.then(|| render(nums, &ops))
}

/// All solutions via DFS (same pruning, memo limited to “first failure” cache).
pub fn solve_all_dfs(nums: &[u128], target: u128) -> Vec<String> {
    if nums.is_empty() {
        return vec![];
    }
    if nums.len() == 1 {
        return if nums[0] == target {
            vec![nums[0].to_string()]
        } else {
            vec![]
        };
    }

    // For collecting all, we can still use a “dead” memo: if a (i,acc) has no solution paths,
    // we can prune whenever we hit it again.
    let mut dead: HashSet<(usize, u128)> = HashSet::new();
    let mut ops: Vec<Op> = Vec::with_capacity(nums.len().saturating_sub(1));
    let mut all: Vec<String> = Vec::new();

    fn dfs_all(
        i: usize,
        acc: u128,
        nums: &[u128],
        target: u128,
        dead: &mut HashSet<(usize, u128)>,
        ops: &mut Vec<Op>,
        out: &mut Vec<String>,
    ) -> bool {
        if i == nums.len() {
            if acc == target {
                out.push(render(nums, ops));
                return true;
            }
            return false;
        }

        if acc > target {
            return false;
        }

        let key = (i, acc);
        if dead.contains(&key) {
            return false;
        }

        let mut found_any = false;
        for &op in [Op::Add, Op::Mul, Op::Concat].iter() {
            if let Some(next_acc) = apply_op(acc, op, nums[i]) {
                ops.push(op);
                let had = dfs_all(i + 1, next_acc, nums, target, dead, ops, out);
                found_any |= had;
                ops.pop();
            }
        }

        if !found_any {
            dead.insert(key);
        }
        found_any
    }

    dfs_all(1, nums[0], nums, target, &mut dead, &mut ops, &mut all);
    all
}

pub fn solvable_dfs(nums: &[u128], target: u128) -> bool {
    solve_first_dfs(nums, target).is_some()
}

/* ------------------- quick checks ------------------- */
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base_two_ops_example() {
        // From earlier example: 3267: 81 40 27
        // With + and * only, this DFS also finds valid expressions (Concat is allowed but not needed).
        let nums = [81u128, 40, 27];
        let target = 3267u128;
        let s = solve_first_dfs(&nums, target).unwrap();
        assert!(s == "81 + 40 * 27" || s == "81 * 40 + 27");
        let mut all = solve_all_dfs(&nums, target);
        all.sort();
        assert_eq!(all, vec!["81 * 40 + 27", "81 + 40 * 27"]);
        assert!(solvable_dfs(&nums, target));
    }

    #[test]
    fn with_concat() {
        // Example where concat is required:
        // 12 || 3 = 123; 123 + 4 = 127
        let nums = [12u128, 3, 4];
        let target = 127u128;
        let all = solve_all_dfs(&nums, target);
        assert!(all.iter().any(|s| s == "12 || 3 + 4"));
    }

    #[test]
    fn no_solution() {
        let nums = [2u128, 2, 2];
        assert!(solve_all_dfs(&nums, 7).is_empty());
        assert!(!solvable_dfs(&nums, 7));
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
