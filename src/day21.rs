use aoc_runner_derive::{aoc, aoc_generator};
#[aoc_generator(day21)]
fn parse(input: &str) -> [([char; 4], u32); 5] {
    let input = input.trim().replace("\r\n", "\n");
    let mut parsed = [([' '; 4], 0); 5];
    for (i, line) in input.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            parsed[i].0[j] = c;
        }
        parsed[i].1 = line[..3].parse().unwrap();
    }

    parsed
}

#[aoc(day21, part1)]
fn part1(codes: &[([char; 4], u32); 5]) -> u64 {
    todo!()
}

#[aoc(day21, part2)]
fn part2(input: &[([char; 4], u32); 5]) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const EXAMPLE: &str = indoc! {"
        029A
        980A
        179A
        456A
        379A
    "};

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), 126384);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE)), "<RESULT>");
    }
}
