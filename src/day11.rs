use aoc_runner_derive::{aoc, aoc_generator};
#[aoc_generator(day11)]
fn parse(input: &str) -> Vec<u64> {
    input
        .trim()
        .split_whitespace()
        .map(|tok| tok.parse().unwrap())
        .collect()
}

#[aoc(day11, part1)]
fn part1(input: &[u64]) -> u128 {
    todo!()
}

#[aoc(day11, part2)]
fn part2(input: &[u64]) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_PART_1: &str = "125 17";

    #[ignore]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE_PART_1)), 55312);
    }

    #[ignore]
    fn part2_example() {
        assert_eq!(part2(&parse("<EXAMPLE>")), "<RESULT>");
    }
}
