use rustc_hash::{FxHashMap, FxHashSet};

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, PartialEq, Eq, Hash)]
struct Point<Num> {
    x: Num,
    y: Num,
}

#[derive(PartialEq, Eq)]
enum Part {
    One,
    Two,
}

#[aoc_generator(day8)]
fn parse(input: &str) -> (FxHashMap<char, Vec<Point<usize>>>, Point<usize>) {
    let input = input.trim();
    let mut points = FxHashMap::default();
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

fn find_antinodes(
    antennas: &FxHashMap<char, Vec<Point<usize>>>,
    max_point: &Point<usize>,
    max_harmonics: usize,
    part: Part,
) -> FxHashSet<Point<isize>> {
    assert!(max_harmonics > 0);
    let max_point = Point {
        x: max_point.x as isize,
        y: max_point.y as isize,
    };
    let mut points_with_antinodes = FxHashSet::default();
    for points in antennas.values() {
        for picked in points.iter() {
            for other in points.iter() {
                if (part == Part::One) && picked == other {
                    continue;
                }
                let dx = picked.x as isize - other.x as isize;
                let dy = picked.y as isize - other.y as isize;
                for i in 0..max_harmonics {
                    let antinode = Point::<isize> {
                        x: (picked.x as isize + dx * (i + 1) as isize),
                        y: (picked.y as isize + dy * (i + 1) as isize),
                    };
                    if antinode.x > max_point.x
                        || antinode.y > max_point.y
                        || antinode.x < 0
                        || antinode.y < 0
                    {
                        break;
                    }
                    points_with_antinodes.insert(antinode);
                }
            }
        }
    }
    points_with_antinodes
}

#[aoc(day8, part1)]
fn part1((antennas, max_point): &(FxHashMap<char, Vec<Point<usize>>>, Point<usize>)) -> u64 {
    find_antinodes(antennas, max_point, 1, Part::One).len() as u64
}

#[aoc(day8, part2)]
fn part2((antennas, max_point): &(FxHashMap<char, Vec<Point<usize>>>, Point<usize>)) -> u64 {
    find_antinodes(antennas, max_point, max_point.x.max(max_point.y), Part::Two).len() as u64
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

    #[test]
    fn part1_simpler_example() {
        let input = indoc! {"
            ..........
            ...#......
            ..........
            ....a.....
            ..........
            .....a....
            ..........
            ......#...
            ..........
            ..........
        "};
        assert_eq!(part1(&parse(input)), 2);
    }

    #[test]
    fn part1_simpler_example2() {
        let input = indoc! {"
            ..........
            ...#......
            #.........
            ....a.....
            ........a.
            .....a....
            ..#.......
            ......#...
            ..........
            ..........
        "};
        assert_eq!(part1(&parse(input)), 4);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(PART_1_EXAMPLE)), 34);
    }
}
