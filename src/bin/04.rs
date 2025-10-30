use advent_of_code::{count_word_in_str, count_x_word_in_str};

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u64> {
    let result = count_word_in_str(&input, "XMAS").ok();
    result.map::<u64, _>(|x| x as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let result = count_x_word_in_str(&input, "MAS").ok();
    result.map::<u64, _>(|x| x as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn overlapping_example() {
        // Single row; "FLAG" appears twice overlapping:
        // XBFLAGALFCV
        //   ^^^^
        //     ^^^^ (starts at the 'F' of FLAG)
        let input = "XBFLAGALFCV";
        assert_eq!(count_word_in_str(input, "FLAG").unwrap(), 2);
    }

    #[test]
    fn multi_line_all_directions() {
        // Grid:
        // S D L A B G
        // G H D F L B
        // C V X D Y T
        let input = "\
SDLABG
GHDFLB
CVXDYT";

        // Some sanity checks in various directions
        assert_eq!(count_word_in_str(input, "SDL").unwrap(), 1); // rightwards on row 0
        assert_eq!(count_word_in_str(input, "SHX").unwrap(), 1); // diagonal down-right
        assert_eq!(count_word_in_str(input, "ADV").unwrap(), 1); // diagonal down-left
        assert_eq!(count_word_in_str(input, "VHD").unwrap(), 1); // upwards
        assert_eq!(count_word_in_str(input, "BG").unwrap(), 2); // 'B'→'G' appears twice in different places/directions
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn lag_x_match() {
        // ALDGF
        // CVADR
        // WLRGH
        // Center 'A' at (1,2) forms an X with L-A-G on one diagonal and G-A-L on the other.
        let input = "\
ALDGF
CVADR
WLRGH";
        assert_eq!(count_x_word_in_str(input, "LAG").unwrap(), 1);
    }

    #[test]
    fn plus_shape_is_ignored() {
        // This is a '+' style cross around 'A'—should NOT count.
        // A L F
        // G A L
        // S G D
        let input = "\
ALF
GAL
SGD";
        assert_eq!(count_x_word_in_str(input, "LAG").unwrap(), 0);
    }

    #[test]
    fn odd_length_requirement() {
        let input = "\
AAAAA
AAAAA
AAAAA
AAAAA
AAAAA";
        // Even length or too short ⇒ 0
        assert_eq!(count_x_word_in_str(input, "AA").unwrap(), 0);
        assert_eq!(count_x_word_in_str(input, "A").unwrap(), 0);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
