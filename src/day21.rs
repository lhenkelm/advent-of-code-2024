use aoc_runner_derive::{aoc, aoc_generator};
#[aoc_generator(day21)]
fn parse(input: &str) -> String {
    todo!()
}

#[aoc(day21, part1)]
fn part1(input: &str) -> u64 {
    todo!()
}

#[aoc(day21, part2)]
fn part2(input: &str) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const EXAMPLE: &str = indoc! {"
        029A
        980A
        179A
        456A
        379A
    "};

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), 126384);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse("<EXAMPLE>")), "<RESULT>");
    }
}
