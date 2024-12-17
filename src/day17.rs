use aoc_runner_derive::{aoc, aoc_generator};
#[aoc_generator(day17)]
fn parse(input: &str) -> String {
    todo!()
}

#[aoc(day17, part1)]
fn part1(input: &str) -> String {
    todo!()
}

#[aoc(day17, part2)]
fn part2(input: &str) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const EXAMPLE: &str = indoc! {"
        Register A: 729
        Register B: 0
        Register C: 0

        Program: 0,1,5,4,3,0
    "};

    #[ignore]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), "4,6,3,5,6,3,5,2,1,0");
    }

    #[ignore]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE)), "<RESULT>");
    }
}
