use std::collections::{HashMap, HashSet};

advent_of_code::solution!(8);

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

pub fn part_one(input: &str) -> Option<u64> {
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let height = grid.len() as i32;
    let width = grid[0].len() as i32;

    // Collect antenna locations by frequency
    let mut antennas: HashMap<char, Vec<Point>> = HashMap::new();
    for (y, row) in grid.iter().enumerate() {
        for (x, &ch) in row.iter().enumerate() {
            if ch != '.' {
                antennas.entry(ch).or_default().push(Point {
                    x: x as i32,
                    y: y as i32,
                });
            }
        }
    }

    let mut antinodes = HashSet::new();
    for (_freq, points) in &antennas {
        for (i, &a) in points.iter().enumerate() {
            for &b in &points[i + 1..] {
                let dx = a.x - b.x;
                let dy = a.y - b.y;
                let p1 = Point {
                    x: a.x + dx,
                    y: a.y + dy,
                };
                let p2 = Point {
                    x: b.x - dx,
                    y: b.y - dy,
                };
                for p in [p1, p2] {
                    if (0..width).contains(&p.x) && (0..height).contains(&p.y) {
                        antinodes.insert(p);
                    }
                }
            }
        }
    }

    Some(antinodes.len().try_into().unwrap())
}

pub fn part_two(input: &str) -> Option<u64> {
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let height = grid.len() as i32;
    let width = grid[0].len() as i32;

    // Collect antenna locations by frequency
    let mut antennas: HashMap<char, Vec<Point>> = HashMap::new();
    for (y, row) in grid.iter().enumerate() {
        for (x, &ch) in row.iter().enumerate() {
            if ch != '.' {
                antennas.entry(ch).or_default().push(Point {
                    x: x as i32,
                    y: y as i32,
                });
            }
        }
    }

    let mut all_antinodes = HashSet::new();
    for (_freq, points) in &antennas {
        for (i, &a) in points.iter().enumerate() {
            for &b in &points[i + 1..] {
                let dx = a.x - b.x;
                let dy = a.y - b.y;

                // Step in both directions until outside the grid
                for n in -1000..=1000 {
                    let x = a.x + n * dx;
                    let y = a.y + n * dy;
                    if (0..width).contains(&x) && (0..height).contains(&y) {
                        all_antinodes.insert(Point { x, y });
                    }
                }
            }
        }
    }

    Some(all_antinodes.len().try_into().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
