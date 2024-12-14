use std::{
    cmp::Ordering,
    ops::{Add, Mul},
    result,
};

use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;
use rustc_hash::FxHashSet;

#[aoc_generator(day14)]
fn parse(input: &str) -> Vec<Robot> {
    let robo_regex = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();
    input
        .trim()
        .split('\n')
        .map(|line| {
            let (_, [px, py, vx, vy]) = robo_regex.captures(line).unwrap().extract();
            Robot {
                pos: Point {
                    x: px.parse().unwrap(),
                    y: py.parse().unwrap(),
                },
                vel: Velocity {
                    x: vx.parse().unwrap(),
                    y: vy.parse().unwrap(),
                },
            }
        })
        .collect()
}

#[aoc(day14, part1)]
fn part1(initial_state: &[Robot]) -> u64 {
    let (width, height) = match initial_state.iter().map(|r| r.pos.y).max() {
        None => panic!("empty input"),
        Some(0..7) => (11i64, 7i64),   // example
        Some(7..) => (101i64, 103i64), // real
        Some(..0) => panic!("negative space"),
    };
    let after_100s = initial_state
        .iter()
        .map(|initial_robot| initial_robot.walk_n_seconds(100, &(width, height)));

    let half_width = (width - 1) / 2;
    let half_height = (height - 1) / 2;
    let mut top_left = 0u64;
    let mut top_right = 0u64;
    let mut bottom_left = 0u64;
    let mut bottom_right = 0u64;
    for walked_robot in after_100s {
        let Robot {
            pos: Point { x, y },
            vel: _,
        } = walked_robot;
        let rel_to_half_height = y.cmp(&half_height);
        let rel_to_half_width = x.cmp(&half_width);

        match (rel_to_half_height, rel_to_half_width) {
            (Ordering::Less, Ordering::Less) => top_left += 1,
            (Ordering::Less, Ordering::Greater) => top_right += 1,
            (Ordering::Greater, Ordering::Less) => bottom_left += 1,
            (Ordering::Greater, Ordering::Greater) => bottom_right += 1,
            _ => (),
        }
    }
    top_left * top_right * bottom_left * bottom_right
}

#[aoc(day14, part2)]
fn part2(initial_state: &[Robot]) -> String {
    let width = 101i64;
    let height = 103i64;

    let secs_to_vars = (0..(width * height))
        .map(|seconds| {
            (
                seconds,
                position_variances(
                    &initial_state
                        .iter()
                        .map(|robot| robot.walk_n_seconds(seconds, &(width, height)))
                        .collect::<Vec<Robot>>(),
                ),
            )
        })
        .collect::<Vec<(i64, (f64, f64))>>();
    let (min_x_secs, (min_x_var, _)) = secs_to_vars
        .iter()
        .min_by(|(_, var_a), (_, var_b)| (var_a.0).partial_cmp(&(var_b.0)).unwrap())
        .unwrap();
    let (min_y_secs, (_, min_y_var)) = secs_to_vars
        .iter()
        .min_by(|(_, var_a), (_, var_b)| (var_a.1).partial_cmp(&(var_b.1)).unwrap())
        .unwrap();

    let mut out = format!(
        "@ {} seconds, have x variance: {}, yielding:\n",
        min_x_secs, min_x_var
    );
    out.push_str(&format_map(
        width,
        height,
        &initial_state
            .iter()
            .map(|robot| robot.walk_n_seconds(*min_x_secs, &(width, height)))
            .collect::<Vec<Robot>>(),
    ));
    out.push_str(&format!(
        "@ {} seconds, have y variance: {}, yielding:\n",
        min_y_secs, min_y_var
    ));
    out.push_str(&format_map(
        width,
        height,
        &initial_state
            .iter()
            .map(|robot| robot.walk_n_seconds(*min_x_secs, &(width, height)))
            .collect::<Vec<Robot>>(),
    ));
    out
}

fn format_map(width: i64, height: i64, robots: &[Robot]) -> String {
    let robot_positions: FxHashSet<Point> = robots.iter().map(|robot| robot.pos).collect();
    let mut result = String::with_capacity(((width + 1) * height) as usize);
    for y in 0..height {
        for x in 0..width {
            let pos = Point { x, y };
            if robot_positions.contains(&pos) {
                result.push('🤖');
            } else {
                result.push(' ');
            }
        }
        result.push('\n');
    }
    result
}

fn position_variances(robots: &[Robot]) -> (f64, f64) {
    let (mean_x, mean_y) = robots
        .iter()
        .map(|robot| (robot.pos.x as f64, robot.pos.y as f64))
        .fold((0f64, 0f64), |(partial_x, partial_y), (x, y)| {
            (
                partial_x + (x / robots.len() as f64),
                partial_y + (y / robots.len() as f64),
            )
        });
    robots
        .iter()
        .map(|robot| {
            (
                (robot.pos.x as f64 - mean_x).powi(2),
                (robot.pos.y as f64 - mean_y).powi(2),
            )
        })
        .fold((0f64, 0f64), |(partial_x, partial_y), (x, y)| {
            (
                partial_x + (x / robots.len() as f64),
                partial_y + (y / robots.len() as f64),
            )
        })
}

#[derive(Debug, Clone)]
struct Robot {
    pos: Point,
    vel: Velocity,
}

impl Robot {
    fn walk_n_seconds(&self, seconds: i64, dimensions: &(i64, i64)) -> Self {
        let new_pos = self.pos.modular_add(self.vel * seconds, dimensions);
        Self {
            pos: new_pos,
            vel: self.vel,
        }
    }
}

impl Add<Velocity> for Point {
    type Output = Point;

    fn add(self, rhs: Velocity) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn modular_add(&self, summand: Velocity, (width, height): &(i64, i64)) -> Self {
        let temp = *self + summand;
        Point {
            x: temp.x.rem_euclid(*width),
            y: temp.y.rem_euclid(*height),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Velocity {
    x: i64,
    y: i64,
}

impl Mul<i64> for Velocity {
    type Output = Velocity;

    fn mul(self, rhs: i64) -> Self::Output {
        Self::Output {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const EXAMPLE: &str = indoc! {"
            p=0,4 v=3,-3
            p=6,3 v=-1,-3
            p=10,3 v=-1,2
            p=2,0 v=2,-1
            p=0,0 v=1,3
            p=3,0 v=-2,-2
            p=7,6 v=-1,-3
            p=3,0 v=-1,-2
            p=9,3 v=2,3
            p=7,3 v=-1,2
            p=2,4 v=2,-3
            p=9,5 v=-3,-3
    "};

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), 12);
    }
}
