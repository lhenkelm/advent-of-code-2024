use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;

#[aoc_generator(day3)]
fn parse(input: &str) -> Vec<(u64, u64)> {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").expect("failed to compile regex");
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
    todo!()
}

#[aoc(day3, part2)]
fn part2(input: &[(u64, u64)]) -> String {
    todo!()
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

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse("<EXAMPLE>")), "<RESULT>");
    }
}
