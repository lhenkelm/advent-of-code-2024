use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day20)]
fn parse(input: &str) -> RaceTrack {
    let input = input.trim().replace("\r\n", "\n");

    let mut data = Vec::new();
    let mut width = 0;
    let mut height = 0;
    let mut start = None;
    let mut end = None;

    for (y, line) in input.lines().enumerate() {
        height = y + 1;
        width = line.len();
        for (x, c) in line.chars().enumerate() {
            let location = match c {
                '#' => Location::Wall,
                '.' => Location::Empty,
                'S' => {
                    start = Some(Point { x, y });
                    Location::Empty
                }
                'E' => {
                    end = Some(Point { x, y });
                    Location::Empty
                }
                _ => panic!("invalid character: {}", c),
            };
            data.push(location);
        }
    }
    debug_assert_eq!(data.len(), width * height);
    let start = start.expect("start not found");
    let end = end.expect("end not found");

    RaceTrack {
        data,
        width,
        height,
        start,
        end,
    }
}

#[aoc(day20, part1)]
fn part1(race_track: &RaceTrack) -> u64 {
    todo!()
}

#[aoc(day20, part2)]
fn part2(race_track: &RaceTrack) -> String {
    todo!()
}

struct RaceTrack {
    data: Vec<Location>,
    width: usize,
    height: usize,
    start: Point,
    end: Point,
}

enum Location {
    Wall,
    Empty,
}

struct Point {
    x: usize,
    y: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    use indoc::indoc;

    const EXAMPLE: &str = indoc! {"
        ###############
        #...#...#.....#
        #.#.#.#.#.###.#
        #S#...#.#.#...#
        #######.#.#.###
        #######.#.#...#
        #######.#.###.#
        ###..E#...#...#
        ###.#######.###
        #...###...#...#
        #.#####.#.###.#
        #.#...#.#.#...#
        #.#.#.#.#.#.###
        #...#...#...###
        ###############
    "};

    #[ignore]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), 44);
    }

    #[ignore]
    fn part2_example() {
        assert_eq!(part2(&parse("<EXAMPLE>")), "<RESULT>");
    }
}
