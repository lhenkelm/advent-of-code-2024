use std::ops::{Add, Index, IndexMut};

use aoc_runner_derive::{aoc, aoc_generator};
use rustc_hash::FxHashSet;

#[aoc_generator(day10)]
fn parse(input: &str) -> Grid<u8> {
    let mut buffer = Vec::with_capacity(input.len());
    let mut width = None;
    let mut height = 0;
    for line in input.trim().lines() {
        if let Some(width) = width {
            debug_assert_eq!(line.len(), width);
        }
        width = Some(line.len());

        buffer.extend(
            line.chars()
                .map(|c| c.to_digit(10).expect("non-digit char found") as u8),
        );
        height += 1;
    }
    let width = width.expect("empty input");
    Grid::<u8> {
        data: buffer,
        width,
        height,
    }
}

#[aoc(day10, part1)]
fn part1(input: &Grid<u8>) -> u64 {
    // shy flow flows downhill like a liquid, but is scared of heights (only flows when diff is == -1)
    // the vector collects the directions from which this point will be shyly-flowed-into
    let mut shy_flows = Grid::<Vec<Direction>> {
        data: vec![Vec::with_capacity(4); input.data.len()],
        width: input.width,
        height: input.height,
    };
    // while doing the first N iterations, we can already collect the points by height
    // which will allow us to build the complete paths from top to bottom in a known, fixed,
    // 9 scans of the grid
    let mut point_idx_by_height = [
        Vec::with_capacity(input.data.len() / 10),
        Vec::with_capacity(input.data.len() / 10),
        Vec::with_capacity(input.data.len() / 10),
        Vec::with_capacity(input.data.len() / 10),
        Vec::with_capacity(input.data.len() / 10),
        Vec::with_capacity(input.data.len() / 10),
        Vec::with_capacity(input.data.len() / 10),
        Vec::with_capacity(input.data.len() / 10),
        Vec::with_capacity(input.data.len() / 10),
    ];
    for (idx, &height) in input.data.iter().enumerate() {
        debug_assert!(height < 10);
        // no flows end in the peak
        if height == 9 {
            continue;
        }
        point_idx_by_height[height as usize].push(idx);
        let point = input.point_index(idx).unwrap();
        for direction in DIRECTIONS {
            let nearest_neighbour = point + direction.step();
            if let Some(&neighbour_height) = input.get(nearest_neighbour) {
                if neighbour_height == height + 1 {
                    shy_flows[point].push(direction);
                }
            }
        }
    }
    let shy_flows = shy_flows;
    let point_idx_by_height = point_idx_by_height;

    let mut flow_sources_ending_at = Grid::<FxHashSet<usize>> {
        data: vec![FxHashSet::default(); input.data.len()],
        width: input.width,
        height: input.height,
    };
    // for height 8 (one below the highest) the sources _are_ the higher neighbours
    for flat_idx in &point_idx_by_height[8] {
        let sink_point = shy_flows.point_index(*flat_idx).unwrap();
        for dir in &shy_flows[sink_point] {
            let source_point = sink_point + dir.step();
            flow_sources_ending_at[sink_point].insert(shy_flows.flat_index(source_point).unwrap());
        }
    }
    // for the other heights, we can step down height by height and aggregate the sources,
    // according to the local flow (and we skip 8 because we already did it)
    for isoheight_indices in point_idx_by_height.iter().rev().skip(1) {
        for flat_idx in isoheight_indices {
            let sink_point = shy_flows.point_index(*flat_idx).unwrap();
            for dir in &shy_flows[sink_point] {
                let source_point = sink_point + dir.step();
                // temp borrow is required for borrow checker (I think)
                // since otherwise both both extend and iter would borrow flow_sources_ending_at,
                // which is not allowed since the extend-call is a requires a mutable reference.
                let mut temp = flow_sources_ending_at[source_point].clone();
                flow_sources_ending_at[sink_point].extend(temp.drain());
            }
        }
    }
    let flow_sources_ending_at = flow_sources_ending_at;

    // now we remember that the inverse of the shy flow is the hinking trail,
    // so where the flow ends (height-0 sinks) is where the trail starts - the trail-head,
    // and the number of elements (sources, i.e. 9-height peaks) reachable from the trail-head
    // are its score, which is now just a matter of counting.
    point_idx_by_height[0]
        .iter()
        .map(|flat_idx| flow_sources_ending_at.data[*flat_idx].len() as u64)
        .sum::<u64>()
}

#[aoc(day10, part2)]
fn part2(input: &Grid<u8>) -> String {
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

#[derive(Debug, Clone, Copy)]
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

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE_PART_1)), 36);
    }

    #[ignore]
    fn part2_example() {
        assert_eq!(part2(&parse("<EXAMPLE>")), "<RESULT>");
    }
}
