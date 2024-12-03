use aoc_runner_derive::{aoc, aoc_generator};
#[aoc_generator(day3)]
fn parse(input: &str) -> String {
    todo!()
}

#[aoc(day3, part1)]
fn part1(input: &str) -> u64 {
    todo!()
}

#[aoc(day3, part2)]
fn part2(input: &str) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn part1_example() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!(part1(&parse(&input)), 161);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse("<EXAMPLE>")), "<RESULT>");
    }
}
