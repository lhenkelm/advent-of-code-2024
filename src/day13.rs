use aoc_runner_derive::{aoc, aoc_generator};
use nalgebra::{Matrix2, Vector2};
use regex::Regex;

#[aoc_generator(day13)]
fn parse(input: &str) -> Vec<ClawMachine> {
    let button_regex = Regex::new(r"Button [AB]: X\+(\d+), Y\+(\d+)").unwrap();
    let prize_regex = Regex::new(r"Prize: X=(\d+), Y=(\d+)").unwrap();
    input
        .trim()
        .split("\n\n")
        .map(|para| {
            let mut lines = para.lines();
            let (_, button_a) = button_regex
                .captures(lines.next().unwrap())
                .unwrap()
                .extract();
            let button_a: [f64; 2] = button_a.map(|tok| tok.parse::<f64>().unwrap());
            let (_, button_b) = button_regex
                .captures(lines.next().unwrap())
                .unwrap()
                .extract();
            let button_b: [f64; 2] = button_b.map(|tok| tok.parse::<f64>().unwrap());
            let (_, prize) = prize_regex
                .captures(lines.next().unwrap())
                .unwrap()
                .extract();
            let prize: [f64; 2] = prize.map(|tok| tok.parse::<f64>().unwrap());
            ClawMachine {
                buttons: Matrix2::new(button_a[0], button_b[0], button_a[1], button_b[1]),
                prize_location: Vector2::new(prize[0], prize[1]),
            }
        })
        .collect()
}

#[aoc(day13, part1)]
fn part1(claw_machines: &[ClawMachine]) -> u32 {
    todo!()
}

#[aoc(day13, part2)]
fn part2(claw_machines: &[ClawMachine]) -> String {
    todo!()
}

#[derive(Debug)]
struct ClawMachine {
    buttons: Matrix2<f64>,
    prize_location: Vector2<f64>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const PART_1_EXAMPLE_INPUT: &str = indoc! {"
            Button A: X+94, Y+34
            Button B: X+22, Y+67
            Prize: X=8400, Y=5400

            Button A: X+26, Y+66
            Button B: X+67, Y+21
            Prize: X=12748, Y=12176

            Button A: X+17, Y+86
            Button B: X+84, Y+37
            Prize: X=7870, Y=6450

            Button A: X+69, Y+23
            Button B: X+27, Y+71
            Prize: X=18641, Y=10279
    "};

    #[ignore]
    fn part1_example() {
        assert_eq!(part1(&parse(PART_1_EXAMPLE_INPUT)), 480);
    }

    #[ignore]
    fn part2_example() {
        assert_eq!(part2(&parse(PART_1_EXAMPLE_INPUT)), "<RESULT>");
    }
}
