use std::collections::BinaryHeap;

use aoc_runner_derive::{aoc, aoc_generator};
use rustc_hash::FxHashMap;

#[aoc_generator(day20)]
fn parse(input: &str) -> RaceTrack {
    let input = input.trim().replace("\r\n", "\n");

    let mut data = Vec::new();
    let mut width = 0;
    let mut height = 0;
    let mut start = None;
    let mut end = None;

    for (y, line) in input.lines().enumerate() {
        height = y + 1;
        width = line.len();
        for (x, c) in line.chars().enumerate() {
            let location = match c {
                '#' => Location::Wall,
                '.' => Location::Empty,
                'S' => {
                    start = Some(Point { x, y });
                    Location::Empty
                }
                'E' => {
                    end = Some(Point { x, y });
                    Location::Empty
                }
                _ => panic!("invalid character: {}", c),
            };
            data.push(location);
        }
    }
    debug_assert_eq!(data.len(), width * height);
    let start = start.expect("start not found");
    let end = end.expect("end not found");

    RaceTrack {
        data,
        width,
        height,
        start,
        end,
    }
}

#[aoc(day20, part1)]
fn part1(race_track: &RaceTrack) -> u64 {
    let min_gain = match race_track.width {
        15 => 1,  // example
        _ => 100, // real input
    };
    count_cheats(race_track, 2, min_gain)
}

#[aoc(day20, part2)]
fn part2(race_track: &RaceTrack) -> u64 {
    let min_gain = match race_track.width {
        15 => 50, // example
        _ => 100, // real input
    };
    count_cheats(race_track, 20, min_gain)
}

fn count_cheats(race_track: &RaceTrack, cheat_duration: usize, min_gain: isize) -> u64 {
    let distances = distances_from_start(race_track);
    let end_distance = distances[&race_track.end];
    let mut cheats = FxHashMap::default();
    let relevant = |&(_, &d): &(&Point, &usize)| d <= end_distance;
    for (&p1, &d1) in distances.iter().filter(relevant) {
        for (&p2, &d2) in distances.iter().filter(relevant) {
            if race_track.flat_index(p1) >= race_track.flat_index(p2) {
                continue;
            }
            let distance = p1.manhattan_distance(&p2);
            if distance < 2 || distance > cheat_duration {
                continue;
            }
            let gain = d1.abs_diff(d2) as isize - distance as isize;
            if gain >= min_gain {
                let (p1, p2) = if d1 < d2 { (p1, p2) } else { (p2, p1) };
                cheats.insert((p1, p2), gain);
            }
        }
    }
    cheats.len() as u64
}

fn distances_from_start(race_track: &RaceTrack) -> FxHashMap<Point, usize> {
    let mut distances = FxHashMap::default();
    let at = race_track.start;
    let distance = 0;
    distances.insert(at, distance);
    let mut frontier = BinaryHeap::new();
    frontier.push(QueueItem {
        point: at,
        distance,
    });
    while let Some(QueueItem {
        point: at,
        distance,
    }) = frontier.pop()
    {
        for neighbour in at.neighbours() {
            if race_track.get(neighbour) == Some(Location::Empty) {
                let cand_dist = distance + 1;
                if *distances.get(&neighbour).unwrap_or(&usize::MAX) <= cand_dist {
                    continue;
                }
                distances.insert(neighbour, cand_dist);
                frontier.push(QueueItem {
                    point: neighbour,
                    distance: cand_dist,
                });
            }
        }
    }
    distances
}
struct RaceTrack {
    data: Vec<Location>,
    width: usize,
    height: usize,
    start: Point,
    end: Point,
}

impl RaceTrack {
    fn get(&self, Point { x, y }: Point) -> Option<Location> {
        if x < self.width && y < self.height {
            Some(self.data[self.flat_index(Point { x, y })])
        } else {
            None
        }
    }

    fn flat_index(&self, Point { x, y }: Point) -> usize {
        y * self.width + x
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct QueueItem {
    point: Point,
    distance: usize,
}

impl Ord for QueueItem {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.distance.cmp(&self.distance)
    }
}

impl PartialOrd for QueueItem {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Location {
    Wall,
    Empty,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn checked_add(&self, vector: &Vector) -> Option<Point> {
        let x = self.x as isize + vector.dx;
        let y = self.y as isize + vector.dy;
        if x >= 0 && y >= 0 {
            Some(Point {
                x: x as usize,
                y: y as usize,
            })
        } else {
            None
        }
    }

    fn neighbours(&self) -> Vec<Point> {
        let mut neighbours = vec![];
        for direction in Direction::all().into_iter() {
            if let Some(pt) = self.checked_add(&direction.vector()) {
                neighbours.push(pt);
            }
        }
        neighbours
    }

    fn manhattan_distance(&self, other: &Point) -> usize {
        let dx = self.x.abs_diff(other.x);
        let dy = self.y.abs_diff(other.y);
        dx + dy
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn all() -> [Direction; 4] {
        [
            Direction::North,
            Direction::East,
            Direction::South,
            Direction::West,
        ]
    }

    fn vector(&self) -> Vector {
        match self {
            Direction::North => Vector { dx: 0, dy: -1 },
            Direction::East => Vector { dx: 1, dy: 0 },
            Direction::South => Vector { dx: 0, dy: 1 },
            Direction::West => Vector { dx: -1, dy: 0 },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Vector {
    dx: isize,
    dy: isize,
}

#[cfg(test)]
mod tests {
    use super::*;

    use indoc::indoc;

    const EXAMPLE: &str = indoc! {"
        ###############
        #...#...#.....#
        #.#.#.#.#.###.#
        #S#...#.#.#...#
        #######.#.#.###
        #######.#.#...#
        #######.#.###.#
        ###..E#...#...#
        ###.#######.###
        #...###...#...#
        #.#####.#.###.#
        #.#...#.#.#...#
        #.#.#.#.#.#.###
        #...#...#...###
        ###############
    "};

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), 44);
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            part2(&parse(EXAMPLE)),
            32 + 31 + 29 + 39 + 25 + 23 + 20 + 19 + 12 + 14 + 12 + 22 + 4 + 3
        );
    }
}
