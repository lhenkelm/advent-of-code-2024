use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
fn parse_pt1(input: &str) -> (Vec<u32>, Vec<u32>) {
    let mut line_it = input.lines().map(|line: &str| {
        line.split_whitespace()
            .map(|token| token.parse::<u32>().expect("expected only numbers and ws"))
    });
    // idea: could use likely ascii-only nature of inputs + utf8-len of str
    // to guess a good initial capacity quickly
    let mut left_parsed = Vec::new();
    let mut right_parsed = Vec::new();
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
    0
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
}
