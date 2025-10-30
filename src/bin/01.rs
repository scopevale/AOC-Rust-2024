use std::collections::HashMap;
advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u64> {
    let mut left: Vec<u32> = Vec::new();
    let mut right: Vec<u32> = Vec::new();

    for line in input.lines() {
        let mut items = line.split_whitespace();
        left.push(items.next().unwrap().parse::<u32>().unwrap());
        right.push(items.next().unwrap().parse::<u32>().unwrap());
    }
    // sort the 2 lists
    left.sort();
    right.sort();

    let answer: u32 = std::iter::zip(left, right)
        .map(|(a, b)| a.abs_diff(b))
        .sum();

    Some(answer.into())
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut left: Vec<&str> = Vec::new();
    let mut right: Vec<&str> = Vec::new();

    for line in input.lines() {
        let mut items = line.split_whitespace();
        left.push(items.next().unwrap());
        right.push(items.next().unwrap());
    }
    // sort the 2 lists
    left.sort();
    right.sort();

    let left_in_right = counts_for_a_in_b(&left, &right);

    let total_nonzero: usize = left_in_right
        .into_iter()
        .filter(|&v| v.1 > 0)
        .map(|v| v.0.parse::<usize>().unwrap() * v.1)
        .sum();

    Some(total_nonzero.try_into().unwrap())
}

fn counts_for_a_in_b<'a>(a: &'a [&'a str], b: &'a [&'a str]) -> HashMap<&'a str, usize> {
    let mut b_counts: HashMap<&'a str, usize> = HashMap::new();
    for &s in b {
        *b_counts.entry(s).or_insert(0) += 1;
    }

    let mut out: HashMap<&'a str, usize> = HashMap::with_capacity(a.len());
    for &s in a {
        // entry() ensures duplicates in `a` don’t redo work
        out.entry(s).or_insert(*b_counts.get(s).unwrap_or(&0));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(223801));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288694));
    }
}
