use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools; // for next_tuple

#[aoc_generator(day15)]
fn parse(input: &str) -> (Grid, Vec<Direction>) {
    let (map_str, instruct_str) = input.trim().split("\n\n").next_tuple().unwrap();

    let mut map = Vec::with_capacity(input.len());
    let mut width = None;
    let mut height = 0;
    for line in map_str.lines() {
        height += 1;
        if width.is_none() {
            width = Some(line.len());
        }
        for c in line.chars() {
            match c {
                '#' => map.push(Occupant::Wall),
                '.' => map.push(Occupant::Empty),
                'O' => map.push(Occupant::Box),
                '@' => map.push(Occupant::Robot),
                _ => panic!("Unexpected warehouse occupant: '{}'", c),
            }
        }
    }
    let width = width.unwrap();
    debug_assert_eq!(width * height, map.len());
    let grid = Grid {
        data: map,
        height,
        width,
    };

    let instructions = instruct_str
        .chars()
        .filter(|c| !c.is_whitespace())
        .map(|c| match c {
            '^' => Direction::North,
            '>' => Direction::East,
            'v' => Direction::South,
            '<' => Direction::West,
            _ => panic!("Unexpected robot instruction: {c}"),
        })
        .collect();
    (grid, instructions)
}

#[aoc(day15, part1)]
fn part1((initial_warehouse, instructions): &(Grid, Vec<Direction>)) -> u64 {
    todo!()
}

#[aoc(day15, part2)]
fn part2((initial_warehouse, instructions): &(Grid, Vec<Direction>)) -> String {
    todo!()
}

struct Grid {
    data: Vec<Occupant>,
    height: usize,
    width: usize,
}

enum Occupant {
    Wall,
    Box,
    Empty,
    Robot,
}

enum Direction {
    North,
    East,
    South,
    West,
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const BIG_EXAMPLE: &str = indoc! {"
            ##########
            #..O..O.O#
            #......O.#
            #.OO..O.O#
            #..O@..O.#
            #O#..O...#
            #O..O..O.#
            #.OO.O.OO#
            #....O...#
            ##########

            <vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
            vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
            ><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
            <<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
            ^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
            ^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
            >^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
            <><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
            ^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
            v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
    "};

    const SMALL_EXAMPLE: &str = indoc! {"
            ########
            #..O.O.#
            ##@.O..#
            #...O..#
            #.#.O..#
            #...O..#
            #......#
            ########

            <^^>>>vv<v>>v<<
    "};

    #[ignore]
    fn part1_big_example() {
        assert_eq!(part1(&parse(BIG_EXAMPLE)), 10092);
    }

    #[ignore]
    fn part1_small_example() {
        assert_eq!(part1(&parse(SMALL_EXAMPLE)), 2028);
    }

    #[ignore]
    fn part2_example() {
        assert_eq!(part2(&parse("<EXAMPLE>")), "<RESULT>");
    }
}
