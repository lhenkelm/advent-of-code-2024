use aoc_runner_derive::{aoc, aoc_generator};
#[aoc_generator(day12)]
fn parse(input: &str) -> String {
    todo!()
}

#[aoc(day12, part1)]
fn part1(input: &str) -> u64 {
    todo!()
}

#[aoc(day12, part2)]
fn part2(input: &str) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const EXAMPLE_INPUT_SMALL: &str = indoc! {"
        AAAA
        BBCD
        BBCC
        EEEC
    "};

    const EXAMPLE_INPUT_ISLANDS: &str = indoc! {"
        OOOOO
        OXOXO
        OOOOO
        OXOXO
        OOOOO
    "};

    const EXAMPLE_INPUT_LARGE: &str = indoc! {"
        RRRRIICCFF
        RRRRIICCCF
        VVRRRCCFFF
        VVRCCCJFFF
        VVVVCJJCFE
        VVIVCCJJEE
        VVIIICJJEE
        MIIIIIJJEE
        MIIISIJEEE
        MMMISSJEEE
    "};

    #[ignore]
    fn part1_example_small() {
        assert_eq!(part1(&parse(EXAMPLE_INPUT_SMALL)), 140);
    }

    #[ignore]
    fn part1_example_islands() {
        assert_eq!(part1(&parse(EXAMPLE_INPUT_ISLANDS)), 772);
    }

    #[ignore]
    fn part1_example_large() {
        assert_eq!(part1(&parse(EXAMPLE_INPUT_LARGE)), 1930);
    }

    #[ignore]
    fn part2_example() {
        assert_eq!(part2(&parse("<EXAMPLE>")), "<RESULT>");
    }
}
