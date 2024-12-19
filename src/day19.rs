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

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE)), 16);
    }

    #[test]
    fn part2_example_design_0() {
        let example = indoc! {"
            r, wr, b, g, bwu, rb, gb, br

            brwrr
        "};
        assert_eq!(part2(&parse(example)), 2);
    }

    #[test]
    fn part2_example_design_1() {
        let example = indoc! {"
            r, wr, b, g, bwu, rb, gb, br

            bggr
        "};
        assert_eq!(part2(&parse(example)), 1);
    }

    #[test]
    fn part2_example_design_2() {
        let example = indoc! {"
            r, wr, b, g, bwu, rb, gb, br

            gbbr
        "};
        assert_eq!(part2(&parse(example)), 4);
    }

    #[test]
    fn part2_example_design_3() {
        let example = indoc! {"
            r, wr, b, g, bwu, rb, gb, br

            rrbgbr
        "};
        assert_eq!(part2(&parse(example)), 6);
    }

    #[test]
    fn part2_example_design_4() {
        let example = indoc! {"
            r, wr, b, g, bwu, rb, gb, br

            ubwu
        "};
        assert_eq!(part2(&parse(example)), 0);
    }

    #[test]
    fn part2_example_design_5() {
        let example = indoc! {"
            r, wr, b, g, bwu, rb, gb, br

            bwurrg
        "};
        assert_eq!(part2(&parse(example)), 1);
    }

    #[test]
    fn part2_example_design_6() {
        let example = indoc! {"
            r, wr, b, g, bwu, rb, gb, br

            brgr
        "};
        assert_eq!(part2(&parse(example)), 2);
    }

    #[test]
    fn part2_example_design_7() {
        let example = indoc! {"
            r, wr, b, g, bwu, rb, gb, br

            bbrgwb
        "};
        assert_eq!(part2(&parse(example)), 0);
    }
}
