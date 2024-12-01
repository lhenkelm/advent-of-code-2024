use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
fn parse_pt1(input: &str) -> (Vec<u32>, Vec<u32>) {
    (vec![0], vec![0])
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
