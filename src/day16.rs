use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use rustc_hash::{FxHashMap, FxHashSet};
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::ops::{Add, Index};

#[aoc_generator(day16)]
fn parse(input: &str) -> Maze {
    let mut width = 0;
    let mut height = 0;
    let mut data = Vec::with_capacity(input.len());
    for (y, line) in input.trim().lines().enumerate() {
        height = y;
        for (x, c) in line.chars().enumerate() {
            width = x;
            data.push(match c {
                '#' => Location::Wall,
                '.' => Location::Empty,
                'S' => Location::Start,
                'E' => Location::End,
                _ => panic!("Invalid character at ({}, {}): {}", x, y, c),
            });
        }
    }
    width += 1;
    height += 1;
    debug_assert_eq!(data.len(), width * height);
    Maze {
        data,
        width,
        height,
    }
}

#[aoc(day16, part1)]
fn part1(maze: &Maze) -> u64 {
    let (distances, _) = kinda_edsger(maze);
    let end = maze.find_end();
    [
        Direction::North,
        Direction::East,
        Direction::South,
        Direction::West,
    ]
    .iter()
    .map(|&d| distances[&Reindeer { at: end, to: d }])
    .min()
    .unwrap()
}

#[aoc(day16, part2)]
fn part2(maze: &Maze) -> u64 {
    let (distances, mut previous) = kinda_edsger(maze);
    let end = maze.find_end();
    let opti_deer = [
        Direction::North,
        Direction::East,
        Direction::South,
        Direction::West,
    ]
    .iter()
    .map(|&d| {
        let rudi = Reindeer { at: end, to: d };
        QueueItem {
            reindeer: rudi,
            distance: distances[&rudi],
        }
    })
    // confusingly, this ignores the Ord implementation of QueueItem
    .min_set();

    let helper = opti_deer.iter().map(|x| x.distance).min().unwrap();

    let mut optimal_path_seats = FxHashSet::default();
    let mut todo = Vec::new();
    for end_deer in opti_deer {
        todo.push(end_deer.reindeer);
    }
    while let Some(best_reindeer) = todo.pop() {
        debug_assert!(distances[&best_reindeer] <= helper);
        optimal_path_seats.insert(best_reindeer.at);
        if let Some(previous_reindeer) = previous.get_mut(&best_reindeer) {
            for previous_reindeer in previous_reindeer.drain(..) {
                debug_assert!(distances[&previous_reindeer] < distances[&best_reindeer]);
                todo.push(previous_reindeer);
            }
        }
    }
    // print the optimal paths
    for y in 0..maze.height {
        for x in 0..maze.width {
            let point = Point { x, y };
            let c = match maze[point] {
                Location::Wall => '#',
                Location::Empty => '.',
                Location::Start => 'S',
                Location::End => 'E',
            };
            let c = if optimal_path_seats.contains(&point) {
                'O'
            } else {
                c
            };
            print!("{}", c);
        }
        println!();
    }

    optimal_path_seats.len() as u64
}

fn kinda_edsger(maze: &Maze) -> (FxHashMap<Reindeer, u64>, FxHashMap<Reindeer, Vec<Reindeer>>) {
    let mut queue = BinaryHeap::new();
    let mut distances = FxHashMap::default();
    let mut previous = FxHashMap::default();

    let start = maze.find_start();
    let start = Reindeer {
        at: start,
        to: Direction::East,
    };

    distances.insert(start, 0);
    queue.push(QueueItem {
        reindeer: start,
        distance: 0,
    });

    while let Some(current_best) = queue.pop() {
        for nearest_reindeer in current_best.reindeer.reachable() {
            if maze[nearest_reindeer.at] == Location::Wall {
                continue;
            }
            let new_distance = match nearest_reindeer.to == current_best.reindeer.to {
                true => current_best.distance + 1,     // walking costs 1
                false => current_best.distance + 1000, // turning costs 1000
            };
            let ord = new_distance.cmp(distances.get(&nearest_reindeer).unwrap_or(&u64::MAX));
            if ord == Ordering::Greater {
                continue;
            }
            if ord == Ordering::Less {
                distances.insert(nearest_reindeer, new_distance);
                queue.push(QueueItem {
                    reindeer: nearest_reindeer,
                    distance: new_distance,
                });
                previous.insert(nearest_reindeer, vec![current_best.reindeer]);
            }
            if ord == Ordering::Equal {
                // it is possible that more than one optimal path leads to the same location,
                // so we need to keep track of all previous Reindeer that lead to the same location
                previous
                    .get_mut(&nearest_reindeer)
                    .unwrap()
                    .push(current_best.reindeer);
            }
        }
    }
    (distances, previous)
}

struct Maze {
    data: Vec<Location>,
    width: usize,
    height: usize,
}

impl Index<Point> for Maze {
    type Output = Location;

    fn index(&self, point: Point) -> &Self::Output {
        &self.data[self.flat_index(point)]
    }
}

impl Maze {
    fn flat_index(&self, Point { x, y }: Point) -> usize {
        debug_assert!(x < self.width);
        debug_assert!(y < self.height);
        y * self.width + x
    }

    fn point_index(&self, index: usize) -> Point {
        debug_assert!(index < self.data.len());
        Point {
            x: index % self.width,
            y: index / self.width,
        }
    }

    fn find_start(&self) -> Point {
        self.data
            .iter()
            .position(|&l| l == Location::Start)
            .map(|i| self.point_index(i))
            .unwrap()
    }

    fn find_end(&self) -> Point {
        self.data
            .iter()
            .position(|&l| l == Location::End)
            .map(|i| self.point_index(i))
            .unwrap()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Location {
    Empty,
    Wall,
    Start,
    End,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct QueueItem {
    reindeer: Reindeer,
    distance: u64,
}

impl Ord for QueueItem {
    fn cmp(&self, other: &Self) -> Ordering {
        match other.distance.cmp(&self.distance) {
            // flip the ordering such that reindeer with less distance are considered "greater"
            // to make the max-heap-by-default std::collections::BinaryHeap a min-heap
            Ordering::Greater => Ordering::Less,
            Ordering::Less => Ordering::Greater,
            Ordering::Equal => Ordering::Equal,
        }
    }
}

impl PartialOrd for QueueItem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Reindeer {
    at: Point,
    to: Direction,
}

impl Reindeer {
    fn turn_left(&self) -> Self {
        Reindeer {
            at: self.at,
            to: self.to.turn_left(),
        }
    }

    fn turn_right(&self) -> Self {
        Reindeer {
            at: self.at,
            to: self.to.turn_right(),
        }
    }

    fn walk(&self) -> Self {
        Reindeer {
            at: self.at + self.to.vector(),
            to: self.to,
        }
    }

    fn reachable(&self) -> [Reindeer; 3] {
        [self.turn_left(), self.turn_right(), self.walk()]
    }
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

    fn turn_left(&self) -> Self {
        match self {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        }
    }

    fn turn_right(&self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
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

    const EXAMPLE_1: &str = indoc! {"
            ###############
            #.......#....E#
            #.#.###.#.###.#
            #.....#.#...#.#
            #.###.#####.#.#
            #.#.#.......#.#
            #.#.#####.###.#
            #...........#.#
            ###.#.#####.#.#
            #...#.....#.#.#
            #.#.#.###.#.#.#
            #.....#...#.#.#
            #.###.#.#.#.#.#
            #S..#.....#...#
            ###############
    "};

    const EXAMPLE_2: &str = indoc! {"
            #################
            #...#...#...#..E#
            #.#.#.#.#.#.#.#.#
            #.#.#.#...#...#.#
            #.#.#.#.###.#.#.#
            #...#.#.#.....#.#
            #.#.#.#.#.#####.#
            #.#...#.#.#.....#
            #.#.#####.#.###.#
            #.#.#.......#...#
            #.#.###.#####.###
            #.#.#...#.....#.#
            #.#.#.#####.###.#
            #.#.#.........#.#
            #.#.#.#########.#
            #S#.............#
            #################
    "};

    #[test]
    fn part1_example_1() {
        assert_eq!(part1(&parse(EXAMPLE_1)), 7036);
    }

    #[test]
    fn part1_example_2() {
        assert_eq!(part1(&parse(EXAMPLE_2)), 11048);
    }

    #[test]
    fn part2_example_1() {
        assert_eq!(part2(&parse(EXAMPLE_1)), 45);
    }

    #[test]
    fn part2_example_2() {
        assert_eq!(part2(&parse(EXAMPLE_2)), 64);
    }
}
