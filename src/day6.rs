use aoc_runner_derive::{aoc, aoc_generator};

enum Location {
    Clear,
    Obstacle,
}

struct MapLab {
    height: usize,
    width: usize,
    buffer: Vec<Location>,
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}
struct GuardState {
    x: usize,
    y: usize,
    facing: Direction,
}

#[aoc_generator(day6)]
fn parse(input: &str) -> (MapLab, GuardState) {
    todo!()
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
    fn part2_example() {
        assert_eq!(part2(&parse(PART_1_EXAMPLE)), "<RESULT>");
    }
}
