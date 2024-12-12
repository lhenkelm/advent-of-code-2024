use std::ops::{Add, Index, IndexMut};

use aoc_runner_derive::{aoc, aoc_generator};
use rustc_hash::{FxHashMap, FxHashSet};
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
    let regions = mark_regions_flood_fill(&input);
    let region_areas = measure_region_areas(&regions);
    let region_perimeters = measure_region_perimeters(&regions);

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

/// Mark connected regions using recursive flood-fill
///
/// The input is the map from points to the plant type at the point.
/// The output is the map from points to the most north-westerly point
/// in the connected region at which the plant can be found.
fn mark_regions_flood_fill(garden_map: &Grid<char>) -> Grid<Point> {
    let mut result = Grid {
        data: vec![
            Point {
                x: garden_map.width,
                y: garden_map.height
            };
            garden_map.data.len()
        ],
        width: garden_map.width,
        height: garden_map.height,
    };
    // TODO: could promote result to Grid<Option<Point>>, removing the need for a second grid.
    let mut visited = Grid {
        data: vec![false; garden_map.data.len()],
        width: garden_map.width,
        height: garden_map.height,
    };
    for flat_idx in 0..garden_map.data.len() {
        let plant_pos = garden_map.point_index(flat_idx).unwrap();
        flood_fill(plant_pos, garden_map, plant_pos, &mut result, &mut visited);
    }
    result
}

/// Implements the recursive step for flood-fill
///
/// Arguments:
///     - `fill_at`:
///         The point being investigated at the current step
///     - `garden`:
///         The map of points to plant types
///     - `region`:
///         The identifying point for this region of plants of it's type.
///         (in the first call, `region == fill_at`)
///     - `result`:
///         The map of points to region-identifying points. If the plant at `fill_at
///         matches that at region and is connected to `region`, `result` at `fill_at`
///         will be set to `region`. Otherwise it will be unchanged.
///     - `visited`:
///         A map of points to a flag indicating whether the point has already been visited.  
fn flood_fill(
    fill_at: Point,
    garden: &Grid<char>,
    region: Point,
    result: &mut Grid<Point>,
    visited: &mut Grid<bool>,
) {
    match visited.get(fill_at) {
        Some(dunnit) => {
            if *dunnit {
                return;
            }
        }
        None => {
            return;
        }
    }
    if garden[fill_at] != garden[region] {
        return;
    }
    result[fill_at] = region;
    visited[fill_at] = true;
    for direction in DIRECTIONS {
        flood_fill(fill_at + direction.step(), garden, region, result, visited);
    }
}

fn measure_region_areas(regions: &Grid<Point>) -> FxHashMap<Point, u64> {
    let mut region_areas = FxHashMap::default();
    for region in regions.data.iter() {
        *region_areas.entry(*region).or_insert(0u64) += 1;
    }
    region_areas
}

fn measure_region_perimeters(regions: &Grid<Point>) -> FxHashMap<Point, u64> {
    let mut perimeter_parts = Grid {
        data: vec![0u32; regions.height * regions.width],
        width: regions.width,
        height: regions.height,
    };
    for (flat_idx, plant_region) in regions.data.iter().enumerate() {
        let plant_pos = regions.point_index(flat_idx).unwrap();
        for direction in DIRECTIONS {
            let neighbour_pos = plant_pos + direction.step();
            match regions.get(neighbour_pos) {
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

    let mut region_perimeters = FxHashMap::default();
    for flat_idx in 0..perimeter_parts.data.len() {
        let region = regions.data[flat_idx];
        *region_perimeters.entry(region).or_insert(0u64) += perimeter_parts.data[flat_idx] as u64;
    }
    region_perimeters
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

    #[test]
    fn part1_concave_diag() {
        let input = indoc! {"
            123-
            45--
            6--7
            --89
        "};
        assert_eq!(part1(&parse(input)), 9 * 4 + 7 * (2 * 3 + 5 * 2));
    }

    #[test]
    fn part1_concave_u() {
        let input = indoc! {"
            #..#
            #..#
            ####
        "};
        assert_eq!(part1(&parse(input)), 32 + 8 * (4 + 2 * 3 + 3 * 2 + 2));
    }

    #[ignore]
    fn part2_example() {
        assert_eq!(part2(&parse("<EXAMPLE>")), "<RESULT>");
    }
}
