use advent_of_code::{is_line_safe, is_safe_monotonic};

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u64> {
    let mut success_count = 0u64;
    let mut _fail_count = 0u64;

    for line in input.lines().filter(|l| !l.trim().is_empty()) {
        // Parse the line into numbers (u32 → cast to i64 for signed diffs)
        let nums: Vec<i64> = match line
            .split_whitespace()
            .map(|tok| tok.parse::<u32>().map(|x| x as i64))
            .collect::<Result<Vec<_>, _>>()
        {
            Ok(v) => v,
            Err(_) => {
                // Bad line → count as fail and skip
                _fail_count += 1;
                continue;
            }
        };

        if is_safe_monotonic(&nums) {
            success_count += 1;
        } else {
            _fail_count += 1;
        }
    }

    Some(success_count)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut success_count = 0u64;
    let mut _fail_count = 0u64;

    for line in input.lines().filter(|l| !l.trim().is_empty()) {
        // Parse the line into numbers (u32 → cast to i64 for signed diffs)
        let nums: Vec<i64> = match line
            .split_whitespace()
            .map(|tok| tok.parse::<u32>().map(|x| x as i64))
            .collect::<Result<Vec<_>, _>>()
        {
            Ok(v) => v,
            Err(_) => {
                // Bad line → count as fail and skip
                _fail_count += 1;
                continue;
            }
        };

        if is_line_safe(&nums) {
            success_count += 1;
        } else {
            _fail_count += 1;
        }
    }

    Some(success_count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }
}
