use std::ops::{Add, Index, IndexMut, Sub};

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
    let regions = mark_regions_flood_fill(input);
    let region_areas = measure_region_areas(&regions);
    let region_perimeters = measure_region_perimeters(&regions);

    let mut total_price = 0;
    for (region, area) in region_areas {
        total_price += area * region_perimeters[&region];
    }
    total_price
}

#[aoc(day12, part2)]
fn part2(input: &Grid<char>) -> u64 {
    let regions = mark_regions_flood_fill(input);
    let region_areas = measure_region_areas(&regions);
    let region_side_counts = count_region_sides(&regions);

    let mut total_price = 0;
    for (region, area) in region_areas {
        total_price += area * region_side_counts[&region];
    }
    total_price
}

/// Mark connected regions using recursive flood-fill
///
/// The input is the map from points to the plant type at the point.
/// The output is the map from points to the most north-westerly point
/// in the connected region at which the plant can be found.
fn mark_regions_flood_fill(garden_map: &Grid<char>) -> Grid<Point> {
    let mut result = Grid {
        data: vec![None; garden_map.data.len()],
        width: garden_map.width,
        height: garden_map.height,
    };
    for flat_idx in 0..garden_map.data.len() {
        let plant_pos = garden_map.point_index(flat_idx).unwrap();
        flood_fill(plant_pos, garden_map, plant_pos, &mut result);
    }
    Grid {
        data: result.data.iter().map(|opt_pt| opt_pt.unwrap()).collect(),
        width: result.width,
        height: result.height,
    }
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
    result: &mut Grid<Option<Point>>,
) {
    match result.get(fill_at) {
        None => {
            // out-of-bounds
            return;
        }
        Some(Some(_)) => {
            // already visited
            return;
        }
        Some(None) => (), // in-bounds, not yet visited
    }
    if garden[fill_at] != garden[region] {
        return;
    }
    result[fill_at] = Some(region);
    for direction in DIRECTIONS {
        flood_fill(fill_at + direction.step(), garden, region, result);
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

fn count_region_sides(regions: &Grid<Point>) -> FxHashMap<Point, u64> {
    let mut side_counts = FxHashMap::default();
    for (flat_index, region) in regions.data.iter().enumerate() {
        let plant_pos = regions.point_index(flat_index).unwrap();
        for diag in [Diagonal::NE, Diagonal::NW, Diagonal::SE, Diagonal::SW] {
            let mut diag_is_in_region = false;
            if let Some(diag_neighbour) = plant_pos.checked_add(diag.step()) {
                if let Some(r) = regions.get(diag_neighbour) {
                    if r == region {
                        diag_is_in_region = true;
                    }
                }
            }
            // if the diagonal neigbour is not of our own region,
            // but the plants (counter-)clock wise either both are, or both
            // are not, it is a corner
            let clockwise_in_region = match plant_pos.checked_add(diag.step_cw()) {
                None => false,
                Some(pos) => match regions.get(pos) {
                    None => false,
                    Some(r) => r == region,
                },
            };
            let counter_clockwise_in_region = match plant_pos.checked_add(diag.step_ccw()) {
                None => false,
                Some(pos) => match regions.get(pos) {
                    None => false,
                    Some(r) => r == region,
                },
            };
            // note that even if the diagonal is of our region,
            // it can have "snuck around" and may not be directly connected,
            // in the case of the outward corner.
            let is_corner = match (clockwise_in_region, counter_clockwise_in_region) {
                (true, true) => !diag_is_in_region, // inner corner (concave)
                (false, false) => true,             // outward corner (convex)
                _ => false,
            };
            if is_corner {
                *side_counts.entry(*region).or_insert(0) += 1
            }
        }
    }
    side_counts
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

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn checked_add(&self, (dx, dy): (isize, isize)) -> Option<Point> {
        if dx < 0 && dx.unsigned_abs() > self.x {
            return None;
        }
        if dy < 0 && dx.unsigned_abs() > self.y {
            return None;
        }
        Some(*self + (dx, dy))
    }
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

impl Sub<(isize, isize)> for Point {
    type Output = Point;

    fn sub(self, (dx, dy): (isize, isize)) -> Self::Output {
        debug_assert!(dx <= self.x as isize);
        debug_assert!(dy <= self.y as isize);

        Point {
            x: (self.x as isize - dx) as usize,
            y: (self.y as isize - dy) as usize,
        }
    }
}

#[derive(Debug)]
enum Diagonal {
    NE,
    SE,
    SW,
    NW,
}
impl Diagonal {
    fn step(&self) -> (isize, isize) {
        match self {
            Diagonal::NE => (1, -1),
            Diagonal::SE => (1, 1),
            Diagonal::SW => (-1, 1),
            Diagonal::NW => (-1, -1),
        }
    }

    fn step_cw(&self) -> (isize, isize) {
        match self {
            Diagonal::NE => Direction::East.step(),
            Diagonal::SE => Direction::South.step(),
            Diagonal::SW => Direction::West.step(),
            Diagonal::NW => Direction::North.step(),
        }
    }

    fn step_ccw(&self) -> (isize, isize) {
        match self {
            Diagonal::NE => Direction::North.step(),
            Diagonal::SE => Direction::East.step(),
            Diagonal::SW => Direction::South.step(),
            Diagonal::NW => Direction::West.step(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
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

    #[test]
    fn part2_example_small() {
        assert_eq!(part2(&parse(EXAMPLE_INPUT_SMALL)), 80);
    }

    #[test]
    fn part2_example_islands() {
        assert_eq!(part2(&parse(EXAMPLE_INPUT_ISLANDS)), 436);
    }

    #[test]
    fn part2_example_large() {
        assert_eq!(part2(&parse(EXAMPLE_INPUT_LARGE)), 1206);
    }

    #[test]
    #[allow(non_snake_case)]
    fn part2_example_EXE() {
        let input = indoc! {"
            EEEEE
            EXXXX
            EEEEE
            EXXXX
            EEEEE
        "};
        assert_eq!(part2(&parse(input)), 236);
    }

    #[test]
    #[allow(non_snake_case)]
    fn part2_example_B_islands_diag() {
        let input = indoc! {"
            AAAAAA
            AAABBA
            AAABBA
            ABBAAA
            ABBAAA
            AAAAAA
        "};
        assert_eq!(part2(&parse(input)), 368);
    }

    #[test]
    fn part2_is_this_it() {
        let input = indoc! {"
            AB
            BA
        "};
        assert_eq!(part2(&parse(input)), 4 * 4);
    }

    #[test]
    fn part2_smaller_islands() {
        let input = indoc! {"
            ~~~~
            ~~#~
            ~#~~
            ~~~~
        "};
        assert_eq!(part2(&parse(input)), 2 * 4 + (4 * 4 - 2) * (3 * 4));
    }

    #[test]
    fn part2_is_it_the_x() {
        let input = indoc! {"
            ~~#
            ~#~
            ~~~
        "};
        assert_eq!(part2(&parse(input)), 2 * 4 + 7 * 10);
    }

    #[test]
    fn part2_example_minimal() {
        let input = indoc! {"
            a
        "};
        assert_eq!(part2(&parse(input)), 4);
    }

    #[test]
    fn part2_example_minimal_corner() {
        let input = indoc! {"
            ab
            bb
        "};
        assert_eq!(part2(&parse(input)), 4 + 3 * 6);
    }

    #[test]
    fn part2_another_try() {
        let input = indoc! {"
            OOOOO
            OXOXO
            OXXXO
        "};
        assert_eq!(part2(&parse(input)), 160);
    }

    #[test]
    fn part2_kerma() {
        let input = indoc! {"
            .....
            .AAA.
            .A.A.
            .AA..
            .A.A.
            .AAA.
            .....
        "};
        assert_eq!(part2(&parse(input)), 452);
    }

    #[test]
    fn part2_maybe() {
        let input = indoc! {"
            XXXXXXX
            XTTTTTX
            XXXTXXX
            XXXTXXX
            XXXTXXX
            XXXXXXX
        "};
        assert_eq!(part2(&parse(input)), 8 * 8 + 12 * (7 * 6 - 8));
    }
}
