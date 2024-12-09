use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day9)]
fn parse(input: &str) -> String {
    todo!()
}

#[aoc(day9, part1)]
fn part1(input: &str) -> u64 {
    todo!()
}

#[aoc(day9, part2)]
fn part2(input: &str) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    const PART_1_EXAMPLE: &str = indoc! {"
        2333133121414131402
    "};

    #[ignore]
    fn part1_example() {
        assert_eq!(part1(&parse(PART_1_EXAMPLE)), 1928u64);
    }

    #[ignore]
    fn part2_example() {
        assert_eq!(part2(&parse(PART_1_EXAMPLE)), "<RESULT>");
    }
}
