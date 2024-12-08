use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, PartialEq, Eq)]
struct Point {
    x: usize,
    y: usize,
}

#[aoc_generator(day8)]
fn parse(input: &str) -> (HashMap<char, Vec<Point>>, Point) {
    let input = input.trim();
    let mut points = HashMap::new();
    let mut max_point = Point { x: 0, y: 0 };
    for (y, line) in input.lines().enumerate() {
        max_point.y = y;
        for (x, character) in line.chars().enumerate() {
            max_point.x = x;
            if character.is_alphanumeric() {
                let entry = points.entry(character).or_insert_with(Vec::new);
                entry.push(Point { x, y });
            }
        }
    }
    (points, max_point)
}

#[aoc(day8, part1)]
fn part1(input: &(HashMap<char, Vec<Point>>, Point)) -> u64 {
    todo!()
}

#[aoc(day8, part2)]
fn part2(input: &(HashMap<char, Vec<Point>>, Point)) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    const PART_1_EXAMPLE: &str = indoc! {"
        ............
        ........0...
        .....0......
        .......0....
        ....0.......
        ......A.....
        ............
        ............
        ........A...
        .........A..
        ............
        ............
    "};

    #[ignore]
    fn part1_example() {
        assert_eq!(part1(&parse(PART_1_EXAMPLE)), 14);
    }

    #[test]
    fn part1_parse_example() {
        let (parsed, max_point) = parse(PART_1_EXAMPLE);
        assert_eq!(parsed.len(), 2);
        assert_eq!(
            parsed[&'0'],
            vec![
                Point { x: 8, y: 1 },
                Point { x: 5, y: 2 },
                Point { x: 7, y: 3 },
                Point { x: 4, y: 4 }
            ]
        );
        assert_eq!(
            parsed[&'A'],
            vec![
                Point { x: 6, y: 5 },
                Point { x: 8, y: 8 },
                Point { x: 9, y: 9 }
            ]
        );
        assert_eq!(max_point, Point { x: 11, y: 11 });
    }

    #[ignore]
    fn part2_example() {
        assert_eq!(part2(&parse(PART_1_EXAMPLE)), "<RESULT>");
    }
}
