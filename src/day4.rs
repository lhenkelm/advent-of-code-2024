use aoc_runner_derive::{aoc, aoc_generator};
#[aoc_generator(day4)]
fn parse(input: &str) -> String {
    input.to_string()
}

#[aoc(day4, part1)]
fn part1(input: &str) -> u64 {
    todo!()
}

#[aoc(day4, part2)]
fn part2(input: &str) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    const PART_1_EXAMPLE: &'static str = indoc! {"
        MMMSXXMASM
        MSAMXMSMSA
        AMXSXMAAMM
        MSAMASMSMX
        XMASAMXAMM
        XXAMMXXAMA
        SMSMSASXSS
        SAXAMASAAA
        MAMMMXMMMM
        MXMXAXMASX
    "};

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(PART_1_EXAMPLE)), 18);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse("<EXAMPLE>")), "<RESULT>");
    }
}
