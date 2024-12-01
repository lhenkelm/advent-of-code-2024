use std::{collections::HashMap, iter::zip};

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
fn parse_pt1(input: &str) -> (Vec<u32>, Vec<u32>) {
    let line_it = input.lines().map(|line: &str| {
        line.split_whitespace()
            .map(|token| token.parse::<u32>().expect("expected only numbers and ws"))
    });
    let estimated_line_no = input.len()
        / input
            .lines()
            .next()
            .expect("expected at least one line of input")
            .len();
    let mut left_parsed = Vec::with_capacity(estimated_line_no);
    let mut right_parsed = Vec::with_capacity(estimated_line_no);
    for parsed_it in line_it {
        let mut parsed_it = parsed_it;
        left_parsed.push(
            parsed_it
                .next()
                .expect("expected two numbers per line, got none"),
        );
        right_parsed.push(
            parsed_it
                .next()
                .expect("expected two numbers per line, got one"),
        );
        if !parsed_it.next().is_none() {
            panic!("expected two numbers per line, got more")
        }
    }
    (left_parsed, right_parsed)
}

#[aoc(day1, part1)]
fn solve_pt1(left_and_right: &(Vec<u32>, Vec<u32>)) -> u32 {
    let (mut left, mut right) = left_and_right.clone();
    left.sort_unstable();
    right.sort_unstable();
    zip(left, right).map(|(l, r)| l.abs_diff(r)).sum::<u32>()
}

fn count_unique(numbers: &[u32]) -> HashMap<u32, u32> {
    let estimated_unique_numbers = numbers.len();
    let mut counts = HashMap::with_capacity(estimated_unique_numbers);
    for &number in numbers {
        let count_ref = counts.entry(number).or_insert(0u32);
        *count_ref += 1;
    }
    counts
}

#[aoc(day1, part2)]
fn solve_pt2(left_and_right: &(Vec<u32>, Vec<u32>)) -> u32 {
    let (left, right) = left_and_right;
    let left_counts = count_unique(&left);
    let right_counts = count_unique(&right);

    let mut sim_score = 0u32;
    for (number, left_freq) in left_counts {
        sim_score += number * left_freq * right_counts.get(&number).unwrap_or(&0);
    }
    sim_score
}
#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    #[test]
    fn parse_example_pt1() {
        let input = indoc! {"
            3   4
            4   3
            2   5
            1   3
            3   9
            3   3
            "};
        let expected_left = [3u32, 4, 2, 1, 3, 3];
        let expected_right = [4u32, 3, 5, 3, 9, 3];

        let parsed = parse_pt1(&input);

        assert_eq!(parsed.0, expected_left);
        assert_eq!(parsed.1, expected_right);
    }

    #[test]
    fn solve_example_pt1() {
        let left_and_right = (vec![3u32, 4, 2, 1, 3, 3], vec![4u32, 3, 5, 3, 9, 3]);

        let solved = solve_pt1(&left_and_right);

        assert_eq!(solved, 11u32);
    }

    #[test]
    fn solve_example_pt2() {
        let left_and_right = (vec![3u32, 4, 2, 1, 3, 3], vec![4u32, 3, 5, 3, 9, 3]);

        let solved = solve_pt2(&left_and_right);

        assert_eq!(solved, 31u32);
    }
}
