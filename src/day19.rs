use aoc_runner_derive::{aoc, aoc_generator};
#[aoc_generator(day19)]
fn parse(input: &str) -> String {
    todo!()
}

#[aoc(day19, part1)]
fn part1(input: &str) -> u64 {
    todo!()
}

#[aoc(day19, part2)]
fn part2(input: &str) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    const EXAMPLE: &str = indoc! {"
        r, wr, b, g, bwu, rb, gb, br

        brwrr
        bggr
        gbbr
        rrbgbr
        ubwu
        bwurrg
        brgr
        bbrgwb
    "};

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), 6);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE)), "<RESULT>");
    }
}
