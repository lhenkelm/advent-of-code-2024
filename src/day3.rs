use aoc_runner_derive::{aoc, aoc_generator};
use regex::{Regex, RegexSet};

const MUL_PATTERN: &'static str = r"mul\((?<l>\d+),(?<r>\d+)\)";
const DO_PATTERN: &'static str = r"do\(\)";
const DONT_PATTERN: &'static str = r"don't\(\)";

#[aoc_generator(day3, part1)]
fn parse(input: &str) -> Vec<(u64, u64)> {
    let re = Regex::new(MUL_PATTERN).expect("failed to compile regex");
    re.captures_iter(input)
        .map(|caps| {
            caps.extract::<2>()
                .1
                .map(|substr| substr.parse::<u64>().expect("expected numbers"))
        })
        .map(|arr| (arr[0], arr[1]))
        .collect()
}

#[aoc(day3, part1)]
fn part1(input: &[(u64, u64)]) -> u64 {
    input.iter().map(|(a, b)| a * b).sum()
}

#[derive(Debug, PartialEq)]
enum Instruction {
    Mul(u64, u64),
    Do,
    Dont,
}

#[aoc_generator(day3, part2)]
fn parse_pt2(input: &str) -> Vec<Instruction> {
    let regexes = [
        Regex::new(MUL_PATTERN).expect("failed to compile MUL_PATTERN"),
        Regex::new(DO_PATTERN).expect("failed to compile DO_PATTERN"),
        Regex::new(DONT_PATTERN).expect("failed to compile DONT_PATTERN"),
    ];
    let regex_set = RegexSet::new([MUL_PATTERN, DO_PATTERN, DONT_PATTERN])
        .expect("patterns in set did not compile");
    let union_pattern = format!("{MUL_PATTERN}|{DO_PATTERN}|{DONT_PATTERN}");
    let union_regex = Regex::new(&union_pattern).expect("failed to compile the union pattern");
    union_regex
        .find_iter(input)
        .map(|match_| {
            let matched_pattern_idx = regex_set
                .matches(&input[match_.range()])
                .iter()
                .next()
                .expect("expected one match, found none");
            match matched_pattern_idx {
                0 => {
                    let captures = regexes[0]
                        .captures(&input[match_.range()])
                        .expect("match should also capture");
                    let l = captures
                        .name("l")
                        .expect("MUL_PATTERN captured without 'l'")
                        .as_str()
                        .parse::<u64>()
                        .expect("expected a number for 'l'");
                    let r = captures
                        .name("r")
                        .expect("MUL_PATTERN captured without 'r'")
                        .as_str()
                        .parse::<u64>()
                        .expect("expected a number for 'r'");
                    Instruction::Mul(l, r)
                }
                1 => Instruction::Do,
                2 => Instruction::Dont,
                3.. => panic!("unknown pattern matched, with index {matched_pattern_idx}"),
            }
        })
        .collect()
}

#[aoc(day3, part2)]
fn part2(input: &[Instruction]) -> u64 {
    let mut enabled = true;
    let mut total = 0;
    for instruction in input {
        match instruction {
            Instruction::Do => enabled = true,
            Instruction::Dont => enabled = false,
            Instruction::Mul(l, r) => total += enabled as u64 * l * r,
        }
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture_input_example() -> &'static str {
        const INPUT: &str =
            "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        INPUT
    }

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(fixture_input_example())), 161);
    }

    #[test]
    fn part1_parse_example() {
        assert_eq!(
            parse(fixture_input_example()),
            [(2, 4), (5, 5), (11, 8), (8, 5)]
        );
    }

    fn part2_fixture_input_example() -> &'static str {
        const INPUT: &'static str =
            "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        INPUT
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_pt2(part2_fixture_input_example())), 48);
    }

    #[test]
    fn part2_parse_example() {
        assert_eq!(
            parse_pt2(part2_fixture_input_example()),
            [
                Instruction::Mul(2, 4),
                Instruction::Dont,
                Instruction::Mul(5, 5),
                Instruction::Mul(11, 8),
                Instruction::Do,
                Instruction::Mul(8, 5),
            ]
        );
    }

    #[test]
    fn part2_parse_just_do() {
        assert_eq!(parse_pt2("do()"), [Instruction::Do])
    }

    #[test]
    fn part2_parse_just_dont() {
        assert_eq!(parse_pt2("don't()"), [Instruction::Dont])
    }
}
