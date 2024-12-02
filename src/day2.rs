use aoc_runner_derive::{aoc, aoc_generator};
#[aoc_generator(day2)]
fn parse(input: &str) -> Vec<Vec<u64>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|token| token.parse::<u64>().expect("only numbers or ws in inputs"))
                .collect()
        })
        .collect()
}

fn part_1_is_report_safe(report: &[u64]) -> bool {
    debug_assert!(report.len() > 1, "got report with less than two readings");
    let n_pairs = report.len() - 1;
    let sign = report[1] as i64 - report[0] as i64;
    for i in 0..n_pairs {
        let prev = report[i];
        let next = report[i + 1];
        if prev == next {
            return false;
        }
        if prev.abs_diff(next) > 3 {
            return false;
        };
        let this_sign = (next as i64 - prev as i64);
        if this_sign * sign < 0 {
            return false;
        }
    }
    true
}

#[aoc(day2, part1)]
fn part1(input: &[Vec<u64>]) -> u64 {
    input
        .iter()
        .map(|report| part_1_is_report_safe(report) as u64)
        .sum()
}

#[aoc(day2, part2)]
fn part2(input: &[Vec<u64>]) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn parse_part1_example() {
        let input = indoc! {"
            7 6 4 2 1
            1 2 7 8 9
            9 7 6 2 1
            1 3 2 4 5
            8 6 4 4 1
            1 3 6 7 9
        "};

        let expected = [
            [7u64, 6, 4, 2, 1],
            [1u64, 2, 7, 8, 9],
            [9u64, 7, 6, 2, 1],
            [1u64, 3, 2, 4, 5],
            [8u64, 6, 4, 4, 1],
            [1u64, 3, 6, 7, 9],
        ];

        assert_eq!(&parse(input), &expected);
    }

    #[test]
    fn part1_example_report1() {
        let report = [7u64, 6, 4, 2, 1];
        assert!(part_1_is_report_safe(&report));
    }

    #[test]
    fn part1_example_report2() {
        let report = [1u64, 2, 7, 8, 9];
        assert!(!part_1_is_report_safe(&report));
    }

    #[test]
    fn part1_example_report3() {
        let report = [9u64, 7, 6, 2, 1];
        assert!(!part_1_is_report_safe(&report));
    }

    #[test]
    fn part1_example_report4() {
        let report = [1u64, 3, 2, 4, 5];
        assert!(!part_1_is_report_safe(&report));
    }

    #[test]
    fn part1_example_report5() {
        let report = [8u64, 6, 4, 4, 1];
        assert!(!part_1_is_report_safe(&report));
    }

    #[test]
    fn part1_example_report6() {
        let report = [1u64, 3, 6, 7, 9];
        assert!(part_1_is_report_safe(&report));
    }

    #[test]
    fn part1_example() {
        let input = [
            vec![7u64, 6, 4, 2, 1],
            vec![1u64, 2, 7, 8, 9],
            vec![9u64, 7, 6, 2, 1],
            vec![1u64, 3, 2, 4, 5],
            vec![8u64, 6, 4, 4, 1],
            vec![1u64, 3, 6, 7, 9],
        ];

        assert_eq!(part1(&input), 2);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse("<EXAMPLE>")), "<RESULT>");
    }
}
