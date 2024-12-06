use aoc_runner_derive::{aoc, aoc_generator};

#[derive(PartialEq, Debug)]
enum Location {
    Clear,
    Obstacle,
}

struct MapLab {
    height: usize,
    width: usize,
    buffer: Vec<Location>,
}

#[derive(PartialEq, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(PartialEq, Debug)]
struct GuardState {
    x: usize,
    y: usize,
    facing: Direction,
}

#[aoc_generator(day6)]
fn parse(input: &str) -> (MapLab, GuardState) {
    let mut buffer = Vec::new();
    let mut height = 0;
    let mut width = 0;
    let mut guard = None;
    for (y, line) in input.trim().lines().enumerate() {
        height = y + 1;
        if y > 0 {
            debug_assert_eq!(line.len(), width, "inconsistent line length");
        }
        width = line.len();
        for (x, c) in line.chars().enumerate() {
            let location = match c {
                '.' => Location::Clear,
                '#' => Location::Obstacle,
                '^' => {
                    debug_assert!(guard.is_none(), "multiple guards found");
                    guard = Some(GuardState {
                        x,
                        y,
                        facing: Direction::Up,
                    });
                    Location::Clear
                }
                _ => panic!("unexpected character: {}", c),
            };
            buffer.push(location);
        }
    }
    let guard = guard.expect("guard not found");
    (
        MapLab {
            height,
            width,
            buffer,
        },
        guard,
    )
}

#[aoc(day6, part1)]
fn part1((map_lab, initial_state): &(MapLab, GuardState)) -> u64 {
    todo!()
}

#[aoc(day6, part2)]
fn part2((map_lab, initial_state): &(MapLab, GuardState)) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const PART_1_EXAMPLE: &str = indoc! {"
        ....#.....
        .........#
        ..........
        ..#.......
        .......#..
        ..........
        .#..^.....
        ........#.
        #.........
        ......#...
    "};

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(PART_1_EXAMPLE)), 41u64);
    }

    #[test]
    fn part1_parse_example() {
        let (map_lab, guard) = parse(PART_1_EXAMPLE);
        assert_eq!(map_lab.height, 10);
        assert_eq!(map_lab.width, 10);
        assert_eq!(map_lab.buffer.len(), 100);
        assert_eq!(
            map_lab
                .buffer
                .iter()
                .filter(|&l| *l == Location::Obstacle)
                .count(),
            8
        );
        assert_eq!(guard.x, 4);
        assert_eq!(guard.y, 6);
        assert_eq!(guard.facing, Direction::Up);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(PART_1_EXAMPLE)), "<RESULT>");
    }
}
