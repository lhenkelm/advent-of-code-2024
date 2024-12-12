use aoc_runner_derive::{aoc, aoc_generator};
use memoize::memoize;

#[aoc_generator(day11)]
fn parse(input: &str) -> Vec<u64> {
    input
        .split_whitespace()
        .map(|tok| tok.parse().unwrap())
        .collect()
}

#[aoc(day11, part1)]
fn part1(stones: &[u64]) -> u64 {
    stones
        .iter()
        .map(|&stone| part1_apply_rules_and_count(stone, 25))
        .sum()
}

#[aoc(day11, part2)]
fn part2(stones: &[u64]) -> u64 {
    // > 1s without memoization, lol
    stones
        .iter()
        .map(|&stone| part1_apply_rules_and_count(stone, 75))
        .sum()
}

#[memoize]
fn part1_apply_rules_and_count(stone: u64, times_remaining: u8) -> u64 {
    if times_remaining == 0 {
        return 1;
    }
    let times_remaining = times_remaining - 1;
    if stone == 0 {
        return part1_apply_rules_and_count(1, times_remaining);
    }
    if let Some((left_stone, right_stone)) = split_if_even_digits(stone) {
        return part1_apply_rules_and_count(left_stone, times_remaining)
            + part1_apply_rules_and_count(right_stone, times_remaining);
    }
    part1_apply_rules_and_count(
        stone
            .checked_mul(2024)
            .unwrap_or_else(|| panic!("fuck it doesn't fit in u64: {stone}*2024")),
        times_remaining,
    )
}

fn split_if_even_digits(stone: u64) -> Option<(u64, u64)> {
    let n_digits = stone.ilog10() + 1;
    if n_digits % 2 != 0 {
        return None;
    }
    let half_n_digits = n_digits / 2;
    let left_stone = stone / 10u64.pow(half_n_digits);
    let right_stone = stone % 10u64.pow(half_n_digits);
    Some((left_stone, right_stone))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_PART_1: &str = "125 17";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE_PART_1)), 55312);
    }
}
