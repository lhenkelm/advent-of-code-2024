use std::ops::{Add, Index, IndexMut, Sub};

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
    let mut sides = FxHashMap::default();
    for (flat_idx, &region) in regions.data.iter().enumerate() {
        let plant_pos = regions.point_index(flat_idx).unwrap();
        let region_sides = sides.entry(region).or_insert_with(FxHashSet::default);
        let normals: Vec<&Direction> = DIRECTIONS
            .iter()
            .filter(|&d| match regions.get(plant_pos + d.step()) {
                None => true,
                Some(reg) => *reg != region,
            })
            .collect();
        let initial_side = match normals.len() {
            0 => {
                continue;
            }
            1 => Side::Edge(Edge {
                loc: plant_pos,
                normal: *normals[0],
            }),
            2 => {
                match &normals[0..2] {
                    // corners
                    [Direction::North, Direction::East] => Side::Corner(Corner {
                        loc: plant_pos,
                        nrm_in: Direction::North,
                        nrm_out: Direction::East,
                    }),
                    [Direction::East, Direction::South] => Side::Corner(Corner {
                        loc: plant_pos,
                        nrm_in: Direction::East,
                        nrm_out: Direction::South,
                    }),
                    [Direction::South, Direction::West] => Side::Corner(Corner {
                        loc: plant_pos,
                        nrm_in: Direction::South,
                        nrm_out: Direction::West,
                    }),
                    [Direction::North, Direction::West] => Side::Corner(Corner {
                        loc: plant_pos,
                        nrm_in: Direction::West,
                        nrm_out: Direction::North,
                    }),
                    // overlapping edges (width one region)
                    [Direction::North, Direction::South] => Side::Edge(Edge {
                        loc: plant_pos,
                        normal: Direction::North,
                    }),
                    [Direction::East, Direction::West] => Side::Edge(Edge {
                        loc: plant_pos,
                        normal: Direction::West,
                    }),
                    _ => unreachable!(),
                }
            }
            3 => {
                let (nrm_in, nrm_mid, nrm_out) = match &normals[0..3] {
                    // clockwise order is assured by DIRECTIONS order
                    [&i, &m, &o] => (i, m, o),
                    _ => unreachable!(),
                };
                Side::Triple(Triple {
                    loc: plant_pos,
                    nrm_in,
                    nrm_mid,
                    nrm_out,
                })
            }
            4 => {
                debug_assert!(region_sides.insert(Side::Full(Full { loc: plant_pos })));
                continue;
            }
            _ => unreachable!(),
        };
        walk_sides(initial_side, regions, region, region_sides);
    }

    let mut sides_counts = FxHashMap::default();
    for (region, reg_sides) in sides {
        *sides_counts.entry(region).or_insert(0u64) +=
            reg_sides.iter().map(|side| side.corners()).sum::<u64>();
    }
    sides_counts
}

fn walk_sides(
    side: Side,
    regions: &Grid<Point>,
    region: Point,
    reg_sides: &mut FxHashSet<Side>,
) -> bool {
    if let Some(r) = regions.get(side.loc()) {
        if *r != region {
            // Sides are inside the region
            return false;
        }
    } else {
        return false;
    }
    if reg_sides.contains(&side) {
        return true;
    }
    match side {
        Side::Edge(Edge { loc, normal }) => {
            if regions
                .get(loc + normal.step())
                .is_some_and(|reg| *reg == region)
            {
                return false;
            }
            let check_orths = match normal {
                Direction::North | Direction::South => [Direction::East, Direction::West],
                Direction::East | Direction::West => [Direction::North, Direction::South],
            };

            for check_dir in check_orths {
                let s = check_dir.step();
                if s.0 > loc.x as isize {
                    return false;
                }
                if s.1 > loc.y as isize {
                    return false;
                }
                match regions.get(loc - s) {
                    Some(reg) => {
                        if *reg != region {
                            return false;
                        }
                    }
                    None => {
                        return false;
                    }
                }
            }
        }
        Side::Corner(Corner {
            loc,
            nrm_in,
            nrm_out,
        }) => {
            for dir in [nrm_in, nrm_out] {
                if regions
                    .get(loc + dir.step())
                    .is_some_and(|reg| *reg == region)
                {
                    return false;
                }
                let s = dir.step();
                if s.0 > loc.x as isize {
                    return false;
                }
                if s.1 > loc.y as isize {
                    return false;
                }
                match regions.get(loc - s) {
                    Some(reg) => {
                        if *reg != region {
                            return false;
                        }
                    }
                    None => {
                        return false;
                    }
                }
            }
        }
        Side::Triple(Triple {
            loc,
            nrm_in,
            nrm_mid,
            nrm_out,
        }) => {
            for dir in [nrm_in, nrm_mid, nrm_out] {
                if regions
                    .get(loc + dir.step())
                    .is_some_and(|reg| *reg == region)
                {
                    return false;
                }
            }
            let s = nrm_mid.step();
            if s.0 > loc.x as isize {
                return false;
            }
            if s.1 > loc.y as isize {
                return false;
            }
            match regions.get(loc - s) {
                Some(reg) => {
                    if *reg != region {
                        return false;
                    }
                }
                None => {
                    return false;
                }
            }
        }
        Side::Full(_) => unreachable!(),
        Side::InnerCorner(_) => unreachable!(),
    }

    reg_sides.insert(side);

    // check the different next boundary segments
    let out_normal = side.out_normal();
    let fwd = match out_normal {
        Direction::North => Direction::East,
        Direction::East => Direction::South,
        Direction::South => Direction::West,
        Direction::West => Direction::North,
    };
    // a) Edges, corners, or triples where the in bound direction
    // is orthogonal to our normal/ aligned to our out-bound one
    // i.e. convex continuations
    let new_loc = side.loc() + fwd.step();
    if walk_sides(
        Side::Edge(Edge::with_in_normal(new_loc, out_normal)),
        regions,
        region,
        reg_sides,
    ) {
        return true;
    }
    if walk_sides(
        Side::Corner(Corner::with_in_normal(new_loc, out_normal)),
        regions,
        region,
        reg_sides,
    ) {
        return true;
    };
    if walk_sides(
        Side::Triple(Triple::with_in_normal(new_loc, out_normal)),
        regions,
        region,
        reg_sides,
    ) {
        return true;
    };

    // also check along concave continuations (inner corners)
    // e.g.:
    // ##
    // #x
    // where the '#' regions boundary continues around a concave corner between
    // (0,0) and  (1,1)
    let past_corner = new_loc + out_normal.step();
    let turned_normal = match out_normal {
        Direction::North => Direction::West,
        Direction::East => Direction::North,
        Direction::South => Direction::East,
        Direction::West => Direction::South,
    };
    // Innner corners are unlike all other boundary elements - they are purely a corner,
    // without any edge components. Their loc can be surrounded by same-region entries
    // on all sides, and where other corners/triples' normals may be modelled as a
    // (series of) clockwise turn (s), inner corners two normals are related by an anti-clockwise
    // turn
    let inner_corner = Side::InnerCorner(Corner::with_conv_normal(new_loc, turned_normal));
    if walk_sides(
        Side::Edge(Edge::with_in_normal(past_corner, turned_normal)),
        regions,
        region,
        reg_sides,
    ) {
        reg_sides.insert(inner_corner);
        return true;
    };
    if walk_sides(
        Side::Corner(Corner::with_in_normal(past_corner, turned_normal)),
        regions,
        region,
        reg_sides,
    ) {
        reg_sides.insert(inner_corner);
        return true;
    };
    if walk_sides(
        Side::Triple(Triple::with_in_normal(past_corner, turned_normal)),
        regions,
        region,
        reg_sides,
    ) {
        reg_sides.insert(inner_corner);
        return true;
    };
    false
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

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Side {
    Edge(Edge),
    Corner(Corner),
    InnerCorner(Corner),
    Triple(Triple),
    Full(Full),
}

impl Side {
    fn out_normal(&self) -> Direction {
        match self {
            Side::Edge(Edge { loc: _, normal }) => *normal,
            Side::Corner(Corner {
                loc: _,
                nrm_in: _,
                nrm_out,
            }) => *nrm_out,
            Side::Triple(Triple {
                loc: _,
                nrm_in: _,
                nrm_mid: _,
                nrm_out,
            }) => *nrm_out,
            Side::Full(full) => panic!("Side::Full::out_normal() called on {:?}!", full),
            Side::InnerCorner(inner) => {
                panic!("Side::InnerCorner:out_normal() called on {:?}!", inner)
            }
        }
    }

    fn corners(&self) -> u64 {
        match self {
            Side::Edge(_) => 0,
            Side::Corner(_) => 1,
            Side::InnerCorner(_) => 1,
            Side::Triple(_) => 2,
            Side::Full(_) => 4,
        }
    }
    fn loc(&self) -> Point {
        match self {
            Side::Edge(e) => e.loc,
            Side::Corner(c) => c.loc,
            Side::Triple(t) => t.loc,
            Side::Full(f) => f.loc,
            Side::InnerCorner(inner) => panic!("Side::InnerCorner::loc() called on {:?}!", inner),
        }
    }
}

/// Part of a straight-line edge
///
/// E.g:
///
/// --###
/// --###
/// --###
///
/// Here, the central point is an Edge with
/// normal: West
/// (from the point of view of the '#' region)
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Edge {
    loc: Point,
    normal: Direction,
}

impl Edge {
    fn with_in_normal(loc: Point, normal: Direction) -> Edge {
        Edge { loc, normal }
    }
}

/// Part of two edges: a corner
///
/// E.g:
///
/// -----
/// --###
/// --###
///
/// Here, the central point is an Corner with
/// nrm_in: West
/// nrm_out: North
/// (from the point of view of the '#' region)
/// There is also a diagonally opposite corner
/// in the '-' region, which is an 'inner' or 'concave' corner -
/// meaning the associated plant has no direct neighbours in another group,
/// and its normals apply to its neighbouring edge cells instead.
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Corner {
    loc: Point,
    nrm_in: Direction,
    nrm_out: Direction,
}

impl Corner {
    fn with_in_normal(loc: Point, normal: Direction) -> Corner {
        let (nrm_in, nrm_out) = match normal {
            Direction::North => (normal, Direction::East),
            Direction::East => (normal, Direction::South),
            Direction::South => (normal, Direction::West),
            Direction::West => (normal, Direction::North),
        };
        Corner {
            loc,
            nrm_in,
            nrm_out,
        }
    }

    fn with_conv_normal(loc: Point, normal: Direction) -> Corner {
        let (nrm_in, nrm_out) = match normal {
            Direction::North => (normal, Direction::West),
            Direction::East => (normal, Direction::North),
            Direction::South => (normal, Direction::East),
            Direction::West => (normal, Direction::South),
        };
        Corner {
            loc,
            nrm_in,
            nrm_out,
        }
    }
}

/// Maximum corner for non-singleton regions: Triple(-edge)
///
/// E.g.:
///
/// ---##
/// --###
/// ---##
///
/// Here, the central point is a corner with
/// nrm_in: South
/// nrm_mid: West
/// nrm_out: North
/// (from the point of view of the '#' region)
///
/// It seems that an alternative view of the Triple is as
/// two Corners sharing a location, with
/// (nrm_in, nrm_mid) and (nrm_mid, nrm_out)
/// as their resepective incoming and outgoing normals.
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Triple {
    loc: Point,
    nrm_in: Direction,
    nrm_mid: Direction,
    nrm_out: Direction,
}
impl Triple {
    fn with_in_normal(loc: Point, normal: Direction) -> Triple {
        let (nrm_in, nrm_mid, nrm_out) = match normal {
            Direction::North => (normal, Direction::East, Direction::South),
            Direction::East => (normal, Direction::South, Direction::West),
            Direction::South => (normal, Direction::West, Direction::North),
            Direction::West => (normal, Direction::North, Direction::East),
        };
        Triple {
            loc,
            nrm_in,
            nrm_mid,
            nrm_out,
        }
    }
}

/// A single isolated point is a "four-edge" corner
///
/// E.g.
///
/// -----
/// --#--
/// -----
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Full {
    loc: Point,
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

impl Sub<(isize, isize)> for Point {
    type Output = Point;

    fn sub(self, (dx, dy): (isize, isize)) -> Self::Output {
        assert!(dx <= self.x as isize);
        assert!(dy <= self.y as isize);

        Point {
            x: (self.x as isize - dx) as usize,
            y: (self.y as isize - dy) as usize,
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
}
