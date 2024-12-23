use aoc_runner_derive::{aoc, aoc_generator};
// next_tuple
use itertools::Itertools;
use regex::{Regex, RegexSet};
use rustc_hash::FxHashMap;

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
fn part2((towel_patterns, towel_designs): &(Vec<String>, Vec<String>)) -> u64 {
    let towel_patterns_for_re: Vec<String> =
        towel_patterns.iter().map(|s| format!("^{}", s)).collect();
    let re_partial = RegexSet::new(towel_patterns_for_re).unwrap();
    let patterns = towel_patterns.join("|");
    let re_full = Regex::new(&format!("^({})+$", patterns)).unwrap();
    let mut cache = FxHashMap::default();
    towel_designs
        .iter()
        .filter(|design| re_full.is_match(design))
        .map(|design| count_ways_to_match(&re_partial, towel_patterns, design, &mut cache))
        .sum()
}

fn count_ways_to_match<'a>(
    regexes: &RegexSet,
    towel_patterns: &[String],
    design: &'a str,
    cache: &mut FxHashMap<&'a str, u64>,
) -> u64 {
    if design.is_empty() {
        return 1;
    }
    if let Some(&count) = cache.get(design) {
        return count;
    }
    let result = regexes
        .matches(design)
        .into_iter()
        .map(|idx| {
            count_ways_to_match(
                regexes,
                towel_patterns,
                &design[towel_patterns[idx].len()..],
                cache,
            )
        })
        .sum();
    cache.insert(design, result);
    result
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
