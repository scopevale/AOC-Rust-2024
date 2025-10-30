use regex::Regex;
use std::option::Option;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u64> {
    let re = Regex::new(r"mul\((?P<num_a>\d+),(?P<num_b>\d+)\)").unwrap();
    let mut total: u64 = 0u64;

    for caps in re.captures_iter(input) {
        let num_a: u64 = caps["num_a"].parse().unwrap();
        let num_b: u64 = caps["num_b"].parse().unwrap();
        total += num_a * num_b;
    }
    Some(total)
}

pub fn part_two(input: &str) -> Option<u64> {
    let re = Regex::new(
        r"((?P<enable>do\(\))|(?P<disable>don\'t\(\))|mul\((?P<num_a>\d+),(?P<num_b>\d+)\))",
    )
    .unwrap();
    let mut total: u64 = 0u64;
    let mut calc_enabled: bool = true;

    for caps in re.captures_iter(input) {
        match caps.get(1).unwrap().as_str() {
            "do()" => calc_enabled = true,
            "don't()" => calc_enabled = false,
            _ => {
                if calc_enabled {
                    let num_a: u64 = caps["num_a"].parse().unwrap();
                    let num_b: u64 = caps["num_b"].parse().unwrap();
                    total += num_a * num_b;
                }
            }
        }
    }
    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(27623467));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18177601));
    }
}
