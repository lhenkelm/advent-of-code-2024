use std::cmp::Ordering;

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, PartialEq)]
struct CalibEq {
    test_value: u64,
    leftmost: u64,
    other_operands: Vec<u64>,
}

impl CalibEq {
    fn from_operands<OpIt>(test_value: u64, operands: &mut OpIt) -> CalibEq
    where
        OpIt: Iterator<Item = u64>,
    {
        let leftmost = operands.next().expect("empty operands iterator");
        CalibEq {
            test_value,
            leftmost,
            other_operands: operands.collect(),
        }
    }

    fn from_line(line: &str) -> CalibEq {
        let mut part_it = line.split(':');
        let test_value = part_it
            .next()
            .expect("no test value found (empty line?)")
            .parse::<u64>()
            .expect("test value is not a number");
        let mut operands = part_it
            .next_back()
            .expect("no operands found")
            .split_ascii_whitespace()
            .map(|token| token.parse::<u64>().expect("NaN operand"));
        debug_assert!(part_it.next().is_none());
        CalibEq::from_operands(test_value, &mut operands)
    }
}

#[derive(Debug, PartialEq)]
struct CalibEqCheckBackwards {
    test_value: u64,
    operands: Vec<u64>,
}

impl CalibEqCheckBackwards {
    fn from_line(line: &str) -> CalibEqCheckBackwards {
        let mut part_it = line.split(':');
        let test_value = part_it
            .next()
            .expect("no test value found (empty line?)")
            .parse::<u64>()
            .expect("test value is not a number");
        let operands = part_it
            .next_back()
            .expect("no operands found")
            .split_ascii_whitespace()
            .map(|token| token.parse::<u64>().expect("NaN operand"))
            .collect();
        debug_assert!(part_it.next().is_none());
        CalibEqCheckBackwards {
            test_value,
            operands,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Operator {
    Add,
    Mul,
    Cat,
}

impl Operator {
    fn apply(&self, lhs: u64, rhs: u64) -> u64 {
        match self {
            Operator::Add => lhs + rhs,
            Operator::Mul => lhs * rhs,
            Operator::Cat => lhs * 10u64.pow(rhs.ilog10() + 1) + rhs,
        }
    }

    fn check_backwards(&self, target: u64, operand: u64) -> Option<u64> {
        match self {
            Operator::Add => {
                if target < operand {
                    None
                } else {
                    Some(target - operand)
                }
            }
            Operator::Mul => {
                if target % operand != 0 {
                    None
                } else {
                    Some(target / operand)
                }
            }
            Operator::Cat => {
                // Q: maybe there is a faster/smarter way to check this?
                let operand_digits = operand.ilog10() + 1;
                let diff = target.checked_sub(operand)?;
                if diff % 10u64.pow(operand_digits) != 0 {
                    None
                } else {
                    Some(diff / 10u64.pow(operand_digits))
                }
            }
        }
    }
}

fn is_possible_no_tally_check(
    calib_eq: &CalibEq,
    n_consumed: usize,
    tally: u64,
    allowed_operators: &[Operator],
) -> bool {
    match calib_eq.other_operands.len().cmp(&n_consumed) {
        Ordering::Less => panic!("consumed too many operands"),
        Ordering::Greater => {
            for &operator in allowed_operators {
                let new_tally = operator.apply(tally, calib_eq.other_operands[n_consumed]);
                if is_possible_no_tally_check(
                    calib_eq,
                    n_consumed + 1,
                    new_tally,
                    allowed_operators,
                ) {
                    return true;
                }
            }
            false
        }
        Ordering::Equal => tally == calib_eq.test_value,
    }
}

fn is_possible_tally_check_early_stop(
    calib_eq: &CalibEq,
    n_consumed: usize,
    tally: u64,
    allowed_operators: &[Operator],
) -> bool {
    match calib_eq.other_operands.len().cmp(&n_consumed) {
        Ordering::Less => panic!("consumed too many operands"),
        Ordering::Greater => {
            if tally > calib_eq.test_value {
                return false;
            }
            for &operator in allowed_operators {
                let new_tally = operator.apply(tally, calib_eq.other_operands[n_consumed]);
                if is_possible_tally_check_early_stop(
                    calib_eq,
                    n_consumed + 1,
                    new_tally,
                    allowed_operators,
                ) {
                    return true;
                }
            }
            false
        }
        Ordering::Equal => tally == calib_eq.test_value,
    }
}

fn is_possible_check_backwards(
    current_target: u64,
    operands: &[u64],
    allowed_operators: &[Operator],
) -> bool {
    debug_assert!(!operands.is_empty(), "need at least one operand");
    // base case
    if operands.len() == 1 {
        return operands[0] == current_target;
    }
    // recursive case
    let last_operand = operands.last().unwrap();
    for operator in allowed_operators {
        match operator.check_backwards(current_target, *last_operand) {
            Some(new_target) => {
                if is_possible_check_backwards(
                    new_target,
                    &operands[..operands.len() - 1],
                    allowed_operators,
                ) {
                    return true;
                }
            }
            None => continue,
        }
    }
    false
}

#[aoc_generator(day7, part1, check_fwd)]
fn parse(input: &str) -> Vec<CalibEq> {
    input.trim().lines().map(CalibEq::from_line).collect()
}

#[aoc_generator(day7, part2, check_fwd)]
fn parse_2(input: &str) -> Vec<CalibEq> {
    input.trim().lines().map(CalibEq::from_line).collect()
}

#[aoc_generator(day7, part1, check_backwards)]
fn parse_check_backwards(input: &str) -> Vec<CalibEqCheckBackwards> {
    input
        .trim()
        .lines()
        .map(CalibEqCheckBackwards::from_line)
        .collect()
}

#[aoc_generator(day7, part2, check_backwards)]
fn parse_check_backwards_p2(input: &str) -> Vec<CalibEqCheckBackwards> {
    input
        .trim()
        .lines()
        .map(CalibEqCheckBackwards::from_line)
        .collect()
}

#[aoc(day7, part1, check_fwd)]
fn part1(input: &[CalibEq]) -> u64 {
    input
        .iter()
        .filter_map(|calib_eq| {
            if is_possible_no_tally_check(
                calib_eq,
                0,
                calib_eq.leftmost,
                &[Operator::Add, Operator::Mul],
            ) {
                Some(calib_eq.test_value)
            } else {
                None
            }
        })
        .sum()
}

#[aoc(day7, part1, check_backwards)]
fn part1_check_backwards(input: &[CalibEqCheckBackwards]) -> u64 {
    input
        .iter()
        .filter_map(|calib_eq| {
            if is_possible_check_backwards(
                calib_eq.test_value,
                &calib_eq.operands,
                &[Operator::Add, Operator::Mul],
            ) {
                Some(calib_eq.test_value)
            } else {
                None
            }
        })
        .sum()
}

#[aoc(day7, part2, check_fwd)]
fn part2(input: &[CalibEq]) -> u64 {
    input
        .iter()
        .filter_map(|calib_eq| {
            if is_possible_tally_check_early_stop(
                calib_eq,
                0,
                calib_eq.leftmost,
                &[Operator::Add, Operator::Mul, Operator::Cat],
            ) {
                Some(calib_eq.test_value)
            } else {
                None
            }
        })
        .sum()
}

#[aoc(day7, part2, check_backwards)]
fn part2_check_backwards(input: &[CalibEqCheckBackwards]) -> u64 {
    input
        .iter()
        .filter_map(|calib_eq| {
            if is_possible_check_backwards(
                calib_eq.test_value,
                &calib_eq.operands,
                &[Operator::Add, Operator::Mul, Operator::Cat],
            ) {
                Some(calib_eq.test_value)
            } else {
                None
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    const PART_1_EXAMPLE_INPUT: &str = indoc! {"
        190: 10 19
        3267: 81 40 27
        83: 17 5
        156: 15 6
        7290: 6 8 6 15
        161011: 16 10 13
        192: 17 8 14
        21037: 9 7 18 13
        292: 11 6 16 20
    "};

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(PART_1_EXAMPLE_INPUT)), 3749u64);
    }

    #[test]
    fn part1_example_check_back() {
        assert_eq!(
            part1_check_backwards(&parse_check_backwards(PART_1_EXAMPLE_INPUT)),
            3749u64
        );
    }

    #[test]
    fn part1_example_parse() {
        let expect = vec![
            CalibEq {
                test_value: 190,
                leftmost: 10,
                other_operands: vec![19],
            },
            CalibEq {
                test_value: 3267,
                leftmost: 81,
                other_operands: vec![40, 27],
            },
            CalibEq {
                test_value: 83,
                leftmost: 17,
                other_operands: vec![5],
            },
            CalibEq {
                test_value: 156,
                leftmost: 15,
                other_operands: vec![6],
            },
            CalibEq {
                test_value: 7290,
                leftmost: 6,
                other_operands: vec![8, 6, 15],
            },
        ];
        for (line, exp) in PART_1_EXAMPLE_INPUT.trim().lines().zip(expect) {
            assert_eq!(parse(line), [exp]);
        }
    }

    #[test]
    fn part1_example_parse_check_back() {
        let expect = vec![
            CalibEqCheckBackwards {
                test_value: 190,
                operands: vec![10, 19],
            },
            CalibEqCheckBackwards {
                test_value: 3267,
                operands: vec![81, 40, 27],
            },
            CalibEqCheckBackwards {
                test_value: 83,
                operands: vec![17, 5],
            },
            CalibEqCheckBackwards {
                test_value: 156,
                operands: vec![15, 6],
            },
            CalibEqCheckBackwards {
                test_value: 7290,
                operands: vec![6, 8, 6, 15],
            },
        ];
        for (line, exp) in PART_1_EXAMPLE_INPUT.trim().lines().zip(expect) {
            assert_eq!(parse_check_backwards(line), [exp]);
        }
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_2(PART_1_EXAMPLE_INPUT)), 11387u64);
    }

    #[test]
    fn part2_example_check_back() {
        assert_eq!(
            part2_check_backwards(&parse_check_backwards_p2(PART_1_EXAMPLE_INPUT)),
            11387u64
        );
    }

    #[test]
    fn concat_op() {
        assert_eq!(Operator::Cat.apply(123, 456), 123456);
        assert_eq!(Operator::Cat.apply(1, 2), 12);
        assert_eq!(Operator::Cat.apply(34, 56), 3456);
    }

    #[test]
    fn concat_check_back() {
        assert_eq!(Operator::Cat.check_backwards(123456, 456), Some(123));
        assert_eq!(Operator::Cat.check_backwards(12346, 456), None);
        assert_eq!(Operator::Cat.check_backwards(12, 2), Some(1));
        assert_eq!(Operator::Cat.check_backwards(12, 3), None);
        assert_eq!(Operator::Cat.check_backwards(3456, 56), Some(34));
        assert_eq!(Operator::Cat.check_backwards(3456, 5), None);
    }
}
