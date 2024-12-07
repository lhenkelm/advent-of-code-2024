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

#[aoc_generator(day7)]
fn parse(input: &str) -> Vec<CalibEq> {
    input.trim().lines().map(CalibEq::from_line).collect()
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
}

const OPERATOR_CHOICES: [Operator; 2] = [Operator::Add, Operator::Mul];

fn is_possible(calib_eq: &CalibEq, operators: &[Operator]) -> bool {
    match calib_eq.other_operands.len().cmp(&operators.len()) {
        Ordering::Less => panic!("too many operators for operands"),
        Ordering::Greater => {
            for operator in OPERATOR_CHOICES {
                let mut try_operators = operators.to_vec();
                try_operators.push(operator);
                if is_possible(calib_eq, &try_operators) {
                    return true;
                }
            }
            false
        }
        Ordering::Equal => {
            let mut result = calib_eq.leftmost;
            for (operand, operator) in calib_eq.other_operands.iter().zip(operators) {
                result = operator.apply(result, *operand)
            }
            result == calib_eq.test_value
        }
    }
}

#[aoc(day7, part1)]
fn part1(input: &[CalibEq]) -> u64 {
    input
        .iter()
        .filter_map(|calib_eq| {
            if is_possible(calib_eq, &[]) {
                Some(calib_eq.test_value)
            } else {
                None
            }
        })
        .sum()
}

#[aoc(day7, part2)]
fn part2(input: &[CalibEq]) -> u64 {
    todo!()
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

    #[ignore]
    fn part2_example() {
        assert_eq!(part2(&parse(PART_1_EXAMPLE_INPUT)), 11387u64);
    }

    #[test]
    fn concat_op() {
        assert_eq!(Operator::Cat.apply(123, 456), 123456);
        assert_eq!(Operator::Cat.apply(1, 2), 12);
        assert_eq!(Operator::Cat.apply(34, 56), 3456);
    }
}
