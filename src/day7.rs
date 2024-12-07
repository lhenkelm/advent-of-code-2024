use aoc_runner_derive::{aoc, aoc_generator};
#[aoc_generator(day7)]
fn parse(input: &str) -> String {
    todo!()
}

#[aoc(day7, part1)]
fn part1(input: &str) -> u64 {
    todo!()
}

#[aoc(day7, part2)]
fn part2(input: &str) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    const PART_1_EXAMPLE_INPUT: &str = indoc! {"
        190: 10 19
        3267: 81 40 27
        83: 17 5
        156: 15 6
        7290: 6 8 6 15
        161011: 16 10 13
        192: 17 8 14
        21037: 9 7 18 13
        292: 11 6 16 20
    "};

    #[ignore]
    fn part1_example() {
        assert_eq!(part1(&parse(PART_1_EXAMPLE_INPUT)), 3749u64);
    }

    #[ignore]
    fn part2_example() {
        assert_eq!(part2(&parse("<EXAMPLE>")), "<RESULT>");
    }
}
