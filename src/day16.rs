use aoc_runner_derive::{aoc, aoc_generator};
#[aoc_generator(day16)]
fn parse(input: &str) -> String {
    todo!()
}

#[aoc(day16, part1)]
fn part1(input: &str) -> u64 {
    todo!()
}

#[aoc(day16, part2)]
fn part2(input: &str) -> String {
    todo!()
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
