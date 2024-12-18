use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use rustc_hash::{FxHashMap, FxHashSet};
use std::{
    cmp::Ordering,
    collections::BinaryHeap,
    fmt::Display,
    ops::{Add, Index, IndexMut},
}; // next_tuple

#[aoc_generator(day18)]
fn parse(input: &str) -> Vec<Point> {
    let input = input.trim().replace("\r\n", "\n");
    input
        .lines()
        .map(|line| {
            let (x, y) = line
                .split(',')
                .map(|s| s.parse().unwrap())
                .next_tuple()
                .unwrap();
            Point { x, y }
        })
        .collect()
}

#[aoc(day18, part1)]
fn part1(input: &[Point]) -> u64 {
    let (width, height, n_fallen) = if input
        .iter()
        .flat_map(|&Point { x, y }| [x, y])
        .max()
        .unwrap()
        > 6
    {
        (71, 71, 1024)
    } else {
        (7, 7, 12)
    };
    let memory = Memory::new(width, height).with_corrupted(&input[..n_fallen]);

    let end = Point {
        x: width - 1,
        y: height - 1,
    };
    let distances = shortest_path_distances(&memory, end);
    distances[&end]
}

#[aoc(day18, part2)]
fn part2(input: &[Point]) -> Point {
    const LOUD: bool = false;
    let (width, height) = if input
        .iter()
        .flat_map(|&Point { x, y }| [x, y])
        .max()
        .unwrap()
        > 6
    {
        (71, 71)
    } else {
        (7, 7)
    };

    if LOUD {
        println!(
            "Have {} bytes falling into memory of dims.: width: {}, height: {}",
            input.len(),
            width,
            height
        );
    }

    let mut memory = Memory::new(width, height).with_corrupted(&input[..width]);
    let end = Point {
        x: width - 1,
        y: height - 1,
    };

    let mut visited = FxHashSet::default();
    for i in 0..width {
        visited.insert(Point { x: i, y: i });
    }
    for point in input {
        if LOUD {
            println!("Filling {}", point);
        }
        memory[*point] = State::Corrupted;
        if !visited.contains(point) {
            continue;
        }
        visited = shortest_path_visited(&memory, end);
        if LOUD {
            println!("Visited: {}", visited.len());
        }
        if !visited.contains(&end) {
            return *point;
        }
    }
    panic!("End is never unreachable");
}

fn shortest_path_distances(memory: &Memory, end: Point) -> FxHashMap<Point, u64> {
    let mut queue = BinaryHeap::new();
    let mut distances = FxHashMap::default();
    let start = Point { x: 0, y: 0 };
    let mut min_end = u64::MAX;

    distances.insert(start, 0);
    queue.push(QueueItem {
        point: start,
        distance: 0,
    });

    while let Some(current_best) = queue.pop() {
        if current_best.point == end && current_best.distance < min_end {
            min_end = current_best.distance;
        }
        if current_best.distance > min_end {
            continue;
        }
        for neighbour in current_best.point.reachable() {
            if memory.get(neighbour).is_none_or(|s| s == State::Corrupted) {
                continue;
            }
            let new_distance = current_best.distance + 1;
            if new_distance < *distances.get(&neighbour).unwrap_or(&u64::MAX) {
                distances.insert(neighbour, new_distance);
                queue.push(QueueItem {
                    point: neighbour,
                    distance: new_distance,
                });
            }
        }
    }
    distances
}

fn shortest_path_visited(memory: &Memory, end: Point) -> FxHashSet<Point> {
    let mut queue = BinaryHeap::new();
    let mut distances = FxHashMap::default();
    let mut visited = FxHashSet::default();
    let start = Point { x: 0, y: 0 };
    let mut min_end = u64::MAX;

    distances.insert(start, 0);
    queue.push(QueueItem {
        point: start,
        distance: 0,
    });

    while let Some(current_best) = queue.pop() {
        if current_best.point == end && current_best.distance < min_end {
            min_end = current_best.distance;
        }
        if current_best.distance > min_end {
            continue;
        }
        for neighbour in current_best.point.reachable() {
            if memory.get(neighbour).is_none_or(|s| s == State::Corrupted) {
                continue;
            }
            let new_distance = current_best.distance + 1;
            match new_distance.cmp(distances.get(&neighbour).unwrap_or(&u64::MAX)) {
                Ordering::Greater | Ordering::Equal => continue,
                Ordering::Less => {
                    distances.insert(neighbour, new_distance);
                    queue.push(QueueItem {
                        point: neighbour,
                        distance: new_distance,
                    });
                    visited.insert(neighbour);
                }
            }
        }
    }
    visited
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct QueueItem {
    point: Point,
    distance: u64,
}

impl Ord for QueueItem {
    // We stick these into a std::collections::BinaryHeap above, to implement
    // the queue in Dijkstra's shortest path algo. However, the aforementioned struct
    // implements max-heap, and we want a min-heap. So this custom impl. flips
    // when Ordering::Less is returned vs Ordering::Greater, by flipping
    // callee (self) and arg (other) when forwarding to u64::cmp.
    fn cmp(&self, other: &Self) -> Ordering {
        other.distance.cmp(&self.distance)
    }
}

impl PartialOrd for QueueItem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct Memory {
    data: Vec<State>,
    width: usize,
    height: usize,
}

impl Index<Point> for Memory {
    type Output = State;

    fn index(&self, point: Point) -> &Self::Output {
        &self.data[self.flat_index(point)]
    }
}

impl IndexMut<Point> for Memory {
    fn index_mut(&mut self, point: Point) -> &mut Self::Output {
        let flat_index = self.flat_index(point);
        &mut self.data[flat_index]
    }
}

impl Memory {
    fn flat_index(&self, Point { x, y }: Point) -> usize {
        debug_assert!(x < self.width);
        debug_assert!(y < self.height);
        y * self.width + x
    }

    fn get(&self, point: Point) -> Option<State> {
        if point.x >= self.width || point.y >= self.height {
            None
        } else {
            Some(self[point])
        }
    }

    fn new(width: usize, height: usize) -> Self {
        Self {
            data: vec![State::Safe; width * height],
            width,
            height,
        }
    }

    fn with_corrupted(mut self, corrupted: &[Point]) -> Self {
        for &point in corrupted {
            self[point] = State::Corrupted;
        }
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum State {
    Safe,
    Corrupted,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: usize,
    y: usize,
}

impl Add<Vector> for Point {
    type Output = Point;

    fn add(self, Vector { dx, dy }: Vector) -> Self::Output {
        let x = self.x as isize + dx;
        let y = self.y as isize + dy;
        debug_assert!(x >= 0);
        debug_assert!(y >= 0);
        Point {
            x: x as usize,
            y: y as usize,
        }
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{}", self.x, self.y)
    }
}

impl Point {
    fn checked_add(&self, Vector { dx, dy }: Vector) -> Option<Self> {
        let x = self.x as isize + dx;
        let y = self.y as isize + dy;
        if x < 0 || y < 0 {
            None
        } else {
            Some(Self {
                x: x as usize,
                y: y as usize,
            })
        }
    }

    fn reachable(&self) -> impl Iterator<Item = Point> + '_ {
        [
            Direction::North,
            Direction::East,
            Direction::South,
            Direction::West,
        ]
        .iter()
        .filter_map(move |&direction| self.checked_add(direction.vector()))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn vector(&self) -> Vector {
        match self {
            Direction::North => Vector { dx: 0, dy: -1 },
            Direction::East => Vector { dx: 1, dy: 0 },
            Direction::South => Vector { dx: 0, dy: 1 },
            Direction::West => Vector { dx: -1, dy: 0 },
        }
    }
}

struct Vector {
    dx: isize,
    dy: isize,
}
#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const EXAMPLE: &str = indoc! {"
        5,4
        4,2
        4,5
        3,0
        2,1
        6,3
        2,4
        1,5
        0,6
        3,3
        2,6
        5,1
        1,2
        5,5
        2,5
        6,5
        1,4
        0,4
        6,4
        1,1
        6,1
        1,0
        0,5
        1,6
        2,0
    "};

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), 22);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE)), Point { x: 6, y: 1 });
    }
}
