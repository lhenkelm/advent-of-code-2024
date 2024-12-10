use aoc_runner_derive::{aoc, aoc_generator};
#[aoc_generator(day10)]
fn parse(input: &str) -> String {
    todo!()
}

#[aoc(day10, part1)]
fn part1(input: &str) -> u64 {
    todo!()
}

#[aoc(day10, part2)]
fn part2(input: &str) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    const EXAMPLE_PART_1: &str = indoc! {"
        89010123
        78121874
        87430965
        96549874
        45678903
        32019012
        01329801
        10456732
    "};

    #[ignore]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE_PART_1)), 36);
    }

    #[ignore]
    fn part2_example() {
        assert_eq!(part2(&parse("<EXAMPLE>")), "<RESULT>");
    }
}
