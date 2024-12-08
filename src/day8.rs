use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, PartialEq, Eq)]
struct Point<Num> {
    x: Num,
    y: Num,
}

#[aoc_generator(day8)]
fn parse(input: &str) -> (HashMap<char, Vec<Point<usize>>>, Point<usize>) {
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
fn part1((antennas, max_point): &(HashMap<char, Vec<Point<usize>>>, Point<usize>)) -> u64 {
    let height = (max_point.y + 1) as isize;
    let width = (max_point.x + 1) as isize;
    let mut n_antinodes = 0;
    for points in antennas.values() {
        for picked in points.iter() {
            for other in points.iter() {
                if picked == other {
                    continue;
                }
                let dx = picked.x as isize - other.x as isize;
                let dy = picked.y as isize - other.y as isize;
                let antinode = Point::<isize> {
                    x: (picked.x as isize + dx),
                    y: (picked.y as isize + dy),
                };
                if (antinode.x < width && antinode.y < height) && (antinode.x > 0 && antinode.y > 0)
                {
                    n_antinodes += 1;
                }
            }
        }
    }
    n_antinodes
}

#[aoc(day8, part2)]
fn part2(input: &(HashMap<char, Vec<Point<usize>>>, Point<usize>)) -> String {
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

    #[test]
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
