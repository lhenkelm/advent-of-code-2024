use std::ops::{Add, Index, IndexMut};

use aoc_runner_derive::{aoc, aoc_generator};
use rustc_hash::FxHashMap;
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
    let mut first_region_occurance = Grid {
        data: vec![
            Point {
                x: input.width,
                y: input.height
            };
            input.height * input.width
        ],
        width: input.width,
        height: input.height,
    };
    'outer: for (flat_idx, &plant) in input.data.iter().enumerate() {
        let plant_pos = input.point_index(flat_idx).unwrap();
        // only check back the directions we have already traversed through
        for direction in [Direction::West, Direction::North] {
            let neighbour_pos = plant_pos + direction.step();
            if let Some(neighbour_plant) = input.get(neighbour_pos) {
                if *neighbour_plant != plant {
                    continue;
                }
                let first_plant_pos = first_region_occurance[neighbour_pos];
                first_region_occurance[plant_pos] = first_plant_pos;
                continue 'outer;
            }
        }
        // if it is disconnected from its kind, its a new region, and its location
        // becomes the identifier of the region
        first_region_occurance[plant_pos] = plant_pos;
    }
    let first_region_occurance = first_region_occurance;
    debug_assert_eq!(first_region_occurance.data.len(), input.data.len());

    let mut perimeter_parts = Grid {
        data: vec![0u32; input.height * input.width],
        width: input.width,
        height: input.height,
    };
    for (flat_idx, plant_region) in first_region_occurance.data.iter().enumerate() {
        let plant_pos = first_region_occurance.point_index(flat_idx).unwrap();
        for direction in DIRECTIONS {
            let neighbour_pos = plant_pos + direction.step();
            match first_region_occurance.get(neighbour_pos) {
                Some(neighbour_region) => {
                    if neighbour_region != plant_region {
                        perimeter_parts[plant_pos] += 1;
                    }
                }
                None => perimeter_parts[plant_pos] += 1,
            }
        }
    }
    let perimeter_parts = perimeter_parts;

    let mut region_areas = FxHashMap::default();
    for region in first_region_occurance.data.iter() {
        *region_areas.entry(*region).or_insert(0u64) += 1;
    }
    let region_areas = region_areas;

    let mut region_perimeters = FxHashMap::default();
    for flat_idx in 0..perimeter_parts.data.len() {
        let region = first_region_occurance.data[flat_idx];
        *region_perimeters.entry(region).or_insert(0u64) += perimeter_parts.data[flat_idx] as u64;
    }
    let region_perimeters = region_perimeters;

    let mut total_price = 0;
    for (region, area) in region_areas {
        total_price += area * region_perimeters[&region];
    }
    total_price
}

#[aoc(day12, part2)]
fn part2(input: &Grid<char>) -> String {
    todo!()
}

#[derive(Debug)]
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

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
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
            Direction::West => (-1, 0),
            Direction::North => (0, -1),
            Direction::East => (1, 0),
            Direction::South => (0, 1),
        }
    }
}

const DIRECTIONS: [Direction; 4] = [
    Direction::East,
    Direction::South,
    Direction::West,
    Direction::North,
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

    #[test]
    fn part1_example_small() {
        assert_eq!(part1(&parse(EXAMPLE_INPUT_SMALL)), 140);
    }

    #[test]
    fn part1_example_islands() {
        assert_eq!(part1(&parse(EXAMPLE_INPUT_ISLANDS)), 772);
    }

    #[test]
    fn part1_example_large() {
        assert_eq!(part1(&parse(EXAMPLE_INPUT_LARGE)), 1930);
    }

    #[test]
    fn part1_mono_region() {
        let input = indoc! {"
            AA
            AA
        "};
        assert_eq!(part1(&parse(input)), 32);
    }

    #[test]
    fn part1_concave_corner() {
        let input = indoc! {"
            ez
            zz
        "};
        assert_eq!(part1(&parse(input)), 1 * 4 + 3 * (3 + 3 + 2));
    }

    #[ignore]
    fn part2_example() {
        assert_eq!(part2(&parse("<EXAMPLE>")), "<RESULT>");
    }
}
