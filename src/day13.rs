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
fn part1(claw_machines: &[ClawMachine]) -> u64 {
    let a_price = 3f64;
    let b_price = 1f64;
    claw_machines
        .iter()
        .map(|claw_machine| {
            let inverse = claw_machine
                .buttons
                .try_inverse()
                .unwrap_or_else(|| panic!("not invertible: {:?}", claw_machine));
            let combo = inverse * claw_machine.prize_location;
            // numbers of button presses must inherently be non-negative and integer
            let e = 1e-2;
            if (combo[(0, 0)].fract() > e && combo[(0, 0)].fract() < 1. - e)
                || (combo[(1, 0)].fract() > e && combo[(1, 0)].fract() < 1. - e)
                || combo[(0, 0)] < 0.0
                || combo[(1, 0)] < 0.0
                || combo[(0, 0)].max(combo[(1, 0)]) > 100.0
            {
                return 0;
            }
            (combo[(0, 0)] * a_price + combo[(1, 0)] * b_price).round() as u64
        })
        .sum()
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

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(PART_1_EXAMPLE_INPUT)), 480);
    }

    #[ignore]
    fn part2_example() {
        assert_eq!(part2(&parse(PART_1_EXAMPLE_INPUT)), "<RESULT>");
    }
}
