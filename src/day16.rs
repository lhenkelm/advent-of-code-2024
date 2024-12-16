use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day16)]
fn parse(input: &str) -> Maze {
    let mut width = 0;
    let mut height = 0;
    let mut data = Vec::with_capacity(input.len());
    for (y, line) in input.trim().lines().enumerate() {
        height = y;
        for (x, c) in line.chars().enumerate() {
            width = x;
            data.push(match c {
                '#' => Location::Wall,
                '.' => Location::Empty,
                'S' => Location::Start,
                'E' => Location::End,
                _ => panic!("Invalid character at ({}, {}): {}", x, y, c),
            });
        }
    }
    width += 1;
    height += 1;
    debug_assert_eq!(data.len(), width * height);
    Maze {
        data,
        width,
        height,
    }
}

#[aoc(day16, part1)]
fn part1(maze: &Maze) -> u64 {
    todo!()
}

#[aoc(day16, part2)]
fn part2(maze: &Maze) -> String {
    todo!()
}

struct Maze {
    data: Vec<Location>,
    width: usize,
    height: usize,
}

enum Location {
    Empty,
    Wall,
    Start,
    End,
}

struct Point {
    x: usize,
    y: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const EXAMPLE_1: &str = indoc! {"
            ###############
            #.......#....E#
            #.#.###.#.###.#
            #.....#.#...#.#
            #.###.#####.#.#
            #.#.#.......#.#
            #.#.#####.###.#
            #...........#.#
            ###.#.#####.#.#
            #...#.....#.#.#
            #.#.#.###.#.#.#
            #.....#...#.#.#
            #.###.#.#.#.#.#
            #S..#.....#...#
            ###############
    "};

    const EXAMPLE_2: &str = indoc! {"
            #################
            #...#...#...#..E#
            #.#.#.#.#.#.#.#.#
            #.#.#.#...#...#.#
            #.#.#.#.###.#.#.#
            #...#.#.#.....#.#
            #.#.#.#.#.#####.#
            #.#...#.#.#.....#
            #.#.#####.#.###.#
            #.#.#.......#...#
            #.#.###.#####.###
            #.#.#...#.....#.#
            #.#.#.#####.###.#
            #.#.#.........#.#
            #.#.#.#########.#
            #S#.............#
            #################
    "};

    #[ignore]
    fn part1_example_1() {
        assert_eq!(part1(&parse(EXAMPLE_1)), 7036);
    }

    #[ignore]
    fn part1_example_2() {
        assert_eq!(part1(&parse(EXAMPLE_2)), 11048);
    }

    #[ignore]
    fn part2_example() {
        assert_eq!(part2(&parse("<EXAMPLE>")), "<RESULT>");
    }
}
