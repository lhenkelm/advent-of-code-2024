use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use regex::Regex; // next_tuple
#[aoc_generator(day19)]
fn parse(input: &str) -> (Vec<String>, Vec<String>) {
    let input = input.trim().replace("\r\n", "\n");
    let (towel_patterns, towel_designs): (&str, &str) = input.split("\n\n").next_tuple().unwrap();
    let towel_patterns: Vec<String> = towel_patterns.split(", ").map(|s| s.to_string()).collect();
    let towel_designs: Vec<String> = towel_designs.lines().map(|s| s.to_string()).collect();
    (towel_patterns, towel_designs)
}

#[aoc(day19, part1)]
fn part1((towel_patterns, towel_designs): &(Vec<String>, Vec<String>)) -> u64 {
    let patterns = towel_patterns.join("|");
    let patterns = format!("^({})+$", patterns);
    let re = Regex::new(&patterns).unwrap();
    towel_designs
        .iter()
        .filter(|design| re.is_match(design))
        .count() as u64
}

#[aoc(day19, part2)]
fn part2((towel_patterns, towel_designs): &(Vec<String>, Vec<String>)) -> String {
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

    #[ignore]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE)), "<RESULT>");
    }
}
