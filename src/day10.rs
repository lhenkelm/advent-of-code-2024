use std::ops::{Add, Index, IndexMut};

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day10)]
fn parse(input: &str) -> String {
    todo!()
}

#[aoc(day10, part1)]
fn part1(input: &str) -> u64 {
    todo!()
}

#[aoc(day10, part2)]
fn part2(input: &str) -> String {
    todo!()
}

#[derive(Debug)]
struct Grid<Item> {
    data: Vec<Item>,
    width: usize,
    height: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: usize,
    y: usize,
}

impl Add<(isize, isize)> for Point {
    type Output = Point;

    fn add(self, (dx, dy): (isize, isize)) -> Self::Output {
        Point {
            x: (self.x as isize + dx) as usize,
            y: (self.y as isize + dy) as usize,
        }
    }
}

impl<Item> Index<Point> for Grid<Item> {
    type Output = Item;

    fn index(&self, point: Point) -> &Self::Output {
        &self.data[point.y * self.width + point.x]
    }
}

impl<Item> IndexMut<Point> for Grid<Item> {
    fn index_mut(&mut self, point: Point) -> &mut Self::Output {
        &mut self.data[point.y * self.width + point.x]
    }
}

enum Direction {
    North,
    East,
    South,
    West,
}
impl Direction {
    fn step(&self) -> (isize, isize) {
        match self {
            Direction::North => (0, -1),
            Direction::East => (1, 0),
            Direction::South => (0, 1),
            Direction::West => (-1, 0),
        }
    }
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    const EXAMPLE_PART_1: &str = indoc! {"
        89010123
        78121874
        87430965
        96549874
        45678903
        32019012
        01329801
        10456732
    "};

    #[ignore]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE_PART_1)), 36);
    }

    #[ignore]
    fn part2_example() {
        assert_eq!(part2(&parse("<EXAMPLE>")), "<RESULT>");
    }
}
