use std::ops::{Add, Index, IndexMut};

use aoc_runner_derive::{aoc, aoc_generator};
#[aoc_generator(day12)]
fn parse(input: &str) -> Grid<char> {
    let mut data = Vec::with_capacity(input.len());
    let mut height = 0;
    let mut width = 0;
    for line in input.trim().lines() {
        height += 1;
        width = line.len();
        for character in line.chars() {
            data.push(character);
        }
    }
    debug_assert_eq!(data.len(), height * width);

    Grid {
        data,
        width,
        height,
    }
}

#[aoc(day12, part1)]
fn part1(input: &Grid<char>) -> u64 {
    todo!()
}

#[aoc(day12, part2)]
fn part2(input: &Grid<char>) -> String {
    todo!()
}

struct Grid<Item> {
    data: Vec<Item>,
    width: usize,
    height: usize,
}

impl<Item> Grid<Item> {
    fn flat_index(&self, point: Point) -> Option<usize> {
        if point.x < self.width && point.y < self.height {
            Some(point.y * self.width + point.x)
        } else {
            None
        }
    }

    fn point_index(&self, flat_index: usize) -> Option<Point> {
        if flat_index >= self.data.len() {
            return None;
        }
        Some(Point {
            x: flat_index % self.width,
            y: flat_index / self.width,
        })
    }

    fn get(&self, point: Point) -> Option<&Item> {
        self.flat_index(point).map(|idx| &self.data[idx])
    }
}

#[derive(Clone, Copy)]
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

#[derive(Clone, Copy)]
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

const DIRECTIONS: [Direction; 4] = [
    Direction::North,
    Direction::East,
    Direction::South,
    Direction::West,
];
#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const EXAMPLE_INPUT_SMALL: &str = indoc! {"
        AAAA
        BBCD
        BBCC
        EEEC
    "};

    const EXAMPLE_INPUT_ISLANDS: &str = indoc! {"
        OOOOO
        OXOXO
        OOOOO
        OXOXO
        OOOOO
    "};

    const EXAMPLE_INPUT_LARGE: &str = indoc! {"
        RRRRIICCFF
        RRRRIICCCF
        VVRRRCCFFF
        VVRCCCJFFF
        VVVVCJJCFE
        VVIVCCJJEE
        VVIIICJJEE
        MIIIIIJJEE
        MIIISIJEEE
        MMMISSJEEE
    "};

    #[ignore]
    fn part1_example_small() {
        assert_eq!(part1(&parse(EXAMPLE_INPUT_SMALL)), 140);
    }

    #[ignore]
    fn part1_example_islands() {
        assert_eq!(part1(&parse(EXAMPLE_INPUT_ISLANDS)), 772);
    }

    #[ignore]
    fn part1_example_large() {
        assert_eq!(part1(&parse(EXAMPLE_INPUT_LARGE)), 1930);
    }

    #[ignore]
    fn part2_example() {
        assert_eq!(part2(&parse("<EXAMPLE>")), "<RESULT>");
    }
}
