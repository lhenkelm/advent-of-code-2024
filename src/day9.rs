use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DenseDiskValue {
    Empty(u8),
    Full(u8),
}

#[aoc_generator(day9)]
fn parse(input: &str) -> Vec<DenseDiskValue> {
    todo!()
}

#[aoc(day9, part1)]
fn part1(input: &Vec<DenseDiskValue>) -> u64 {
    todo!()
}

#[aoc(day9, part2)]
fn part2(input: &Vec<DenseDiskValue>) -> String {
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
    fn part_1_example_parse() {
        use DenseDiskValue::*;
        assert_eq!(
            parse(PART_1_EXAMPLE),
            vec![
                Full(2),
                Empty(3),
                Full(3),
                Empty(3),
                Full(1),
                Empty(3),
                Full(3),
                Empty(1),
                Full(2),
                Empty(1),
                Full(4),
                Empty(1),
                Full(4),
                Empty(1),
                Full(3),
                Empty(1),
                Full(4),
                // Empty(0) or Full(0) can be skipped in this representation
                Full(2),
            ],
        );
    }

    #[ignore]
    fn part2_example() {
        assert_eq!(part2(&parse(PART_1_EXAMPLE)), "<RESULT>");
    }
}
