use std::ops::{Index, IndexMut};

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(PartialEq, Debug, Clone, Copy)]
enum Location {
    Clear,
    Obstacle,
}

#[derive(PartialEq, Debug, Clone, Copy)]
enum StopReason {
    Obstacle,
    EndOfMap,
}

impl PartialEq<StopReason> for Location {
    fn eq(&self, other: &StopReason) -> bool {
        match (self, other) {
            (Location::Obstacle, StopReason::Obstacle) => true,
            _ => false,
        }
    }
}

struct MapLab {
    height: usize,
    width: usize,
    buffer: Vec<Location>,
}

impl Index<&(usize, usize)> for MapLab {
    type Output = Location;

    fn index(&self, (x, y): &(usize, usize)) -> &Self::Output {
        debug_assert_eq!(
            self.height * self.width,
            self.buffer.len(),
            "inconsistent buffer size"
        );
        debug_assert!(*x < self.width, "x out of bounds");
        debug_assert!(*y < self.height, "y out of bounds");
        &self.buffer[y * self.width + x]
    }
}

impl MapLab {
    fn get(&self, (x, y): &(usize, usize)) -> Option<Location> {
        if *x < self.width && *y < self.height {
            Some(self[&(*x, *y)])
        } else {
            None
        }
    }

    fn until_obstacle(
        &self,
        from_pos: &(usize, usize),
        towards: Direction,
    ) -> ((usize, usize), StopReason) {
        debug_assert!(self.get(from_pos).is_some(), "from_pos out of bounds");
        let step = match towards {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        };
        let mut pos = *from_pos;
        loop {
            let next_pos = (
                (pos.0 as isize + step.0) as usize,
                (pos.1 as isize + step.1) as usize,
            );
            match self.get(&next_pos) {
                Some(Location::Obstacle) => return (pos, StopReason::Obstacle),
                Some(Location::Clear) => pos = next_pos,
                None => return (pos, StopReason::EndOfMap),
            }
        }
    }
}

#[derive(PartialEq, Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(PartialEq, Debug, Clone)]
struct GuardState {
    pos: (usize, usize),
    facing: Direction,
}

impl GuardState {
    fn walk(&self, map_lab: &MapLab) -> (GuardState, StopReason) {
        let (new_pos, stop_reason) = map_lab.until_obstacle(&self.pos, self.facing);
        let new_facing = match self.facing {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        };
        (
            GuardState {
                pos: new_pos,
                facing: new_facing,
            },
            stop_reason,
        )
    }
}

struct BeenThereDoneThat {
    height: usize,
    width: usize,
    buffer: Vec<bool>,
}

impl Index<&(usize, usize)> for BeenThereDoneThat {
    type Output = bool;

    fn index(&self, (x, y): &(usize, usize)) -> &Self::Output {
        debug_assert_eq!(
            self.height * self.width,
            self.buffer.len(),
            "inconsistent buffer size"
        );
        debug_assert!(*x < self.width, "x out of bounds");
        debug_assert!(*y < self.height, "y out of bounds");
        &self.buffer[y * self.width + x]
    }
}

impl IndexMut<&(usize, usize)> for BeenThereDoneThat {
    fn index_mut(&mut self, (x, y): &(usize, usize)) -> &mut Self::Output {
        debug_assert_eq!(
            self.height * self.width,
            self.buffer.len(),
            "inconsistent buffer size"
        );
        debug_assert!(*x < self.width, "x out of bounds");
        debug_assert!(*y < self.height, "y out of bounds");
        &mut self.buffer[y * self.width + x]
    }
}

impl BeenThereDoneThat {
    fn with_dimensions(width: usize, height: usize) -> Self {
        BeenThereDoneThat {
            height,
            width,
            buffer: vec![false; width * height],
        }
    }

    fn visit(&mut self, (x, y): &(usize, usize)) {
        self[&(*x, *y)] = true;
    }

    fn visit_between(&mut self, from: &(usize, usize), to: &(usize, usize)) {
        let step = (
            (to.0 as isize - from.0 as isize).signum(),
            (to.1 as isize - from.1 as isize).signum(),
        );
        debug_assert!(
            step.0 == 0 || step.1 == 0,
            "diagonal movement not supported"
        );
        let mut pos = *from;
        while pos != *to {
            self.visit(&pos);
            pos = (
                (pos.0 as isize + step.0) as usize,
                (pos.1 as isize + step.1) as usize,
            );
        }
        self.visit(to);
    }

    fn total(&self) -> u64 {
        self.buffer.iter().filter(|&&b| b).count() as u64
    }
}

#[aoc_generator(day6)]
fn parse(input: &str) -> (MapLab, GuardState) {
    let mut buffer = Vec::new();
    let mut height = 0;
    let mut width = 0;
    let mut guard = None;
    for (y, line) in input.trim().lines().enumerate() {
        height = y + 1;
        if y > 0 {
            debug_assert_eq!(line.len(), width, "inconsistent line length");
        }
        width = line.len();
        for (x, c) in line.chars().enumerate() {
            let location = match c {
                '.' => Location::Clear,
                '#' => Location::Obstacle,
                '^' | '>' | '<' | 'v' => {
                    debug_assert!(guard.is_none(), "multiple guards found");
                    guard = Some(GuardState {
                        pos: (x, y),
                        facing: match c {
                            '^' => Direction::Up,
                            '>' => Direction::Right,
                            '<' => Direction::Left,
                            'v' => Direction::Down,
                            _ => unreachable!(),
                        },
                    });
                    Location::Clear
                }
                _ => panic!("unexpected character: {}", c),
            };
            buffer.push(location);
        }
    }
    let guard = guard.expect("guard not found");
    (
        MapLab {
            height,
            width,
            buffer,
        },
        guard,
    )
}

#[aoc(day6, part1)]
fn part1((map_lab, initial_state): &(MapLab, GuardState)) -> u64 {
    let mut to_be_or_not_to_be = BeenThereDoneThat::with_dimensions(map_lab.width, map_lab.height);
    let mut guard_state = initial_state.clone();
    let mut stop_reason = StopReason::Obstacle;
    while stop_reason != StopReason::EndOfMap {
        let (new_state, new_reason) = guard_state.walk(&map_lab);
        to_be_or_not_to_be.visit_between(&guard_state.pos, &new_state.pos);
        guard_state = new_state;
        stop_reason = new_reason;
    }
    to_be_or_not_to_be.total()
}

#[aoc(day6, part2)]
fn part2((map_lab, initial_state): &(MapLab, GuardState)) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const PART_1_EXAMPLE: &str = indoc! {"
        ....#.....
        .........#
        ..........
        ..#.......
        .......#..
        ..........
        .#..^.....
        ........#.
        #.........
        ......#...
    "};

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(PART_1_EXAMPLE)), 41u64);
    }

    #[test]
    fn part1_parse_example() {
        let (map_lab, guard) = parse(PART_1_EXAMPLE);
        assert_eq!(map_lab.height, 10);
        assert_eq!(map_lab.width, 10);
        assert_eq!(map_lab.buffer.len(), 100);
        assert_eq!(
            map_lab
                .buffer
                .iter()
                .filter(|&l| *l == Location::Obstacle)
                .count(),
            8
        );
        assert_eq!(guard.pos.0, 4);
        assert_eq!(guard.pos.1, 6);
        assert_eq!(guard.facing, Direction::Up);
    }

    #[test]
    fn test_map_access_wide_line() {
        let map_lab = MapLab {
            height: 1,
            width: 5,
            buffer: vec![
                Location::Clear,
                Location::Obstacle,
                Location::Clear,
                Location::Clear,
                Location::Clear,
            ],
        };
        assert_eq!(map_lab.get(&(0, 0)), Some(Location::Clear));
        assert_eq!(map_lab.get(&(1, 0)), Some(Location::Obstacle));
        assert_eq!(map_lab.get(&(2, 0)), Some(Location::Clear));
        assert_eq!(map_lab.get(&(3, 0)), Some(Location::Clear));
        assert_eq!(map_lab.get(&(4, 0)), Some(Location::Clear));
        assert_eq!(map_lab.get(&(5, 0)), None);
        assert_eq!(map_lab.get(&(0, 1)), None);
        assert_eq!(map_lab.get(&(1, 1)), None);
    }

    #[test]
    fn test_map_access_tall_line() {
        let map_lab = MapLab {
            height: 3,
            width: 1,
            buffer: vec![Location::Obstacle, Location::Clear, Location::Obstacle],
        };
        assert_eq!(map_lab.get(&(0, 0)), Some(Location::Obstacle));
        assert_eq!(map_lab.get(&(0, 1)), Some(Location::Clear));
        assert_eq!(map_lab.get(&(0, 2)), Some(Location::Obstacle));
        assert_eq!(map_lab.get(&(0, 3)), None);
        assert_eq!(map_lab.get(&(1, 0)), None);
        assert_eq!(map_lab.get(&(1, 1)), None);
    }

    #[test]
    fn test_map_access_small_rectangle() {
        let map_lab = MapLab {
            height: 2,
            width: 3,
            buffer: vec![
                Location::Clear,
                Location::Obstacle,
                Location::Clear,
                Location::Obstacle,
                Location::Clear,
                Location::Obstacle,
            ],
        };
        assert_eq!(map_lab.get(&(0, 0)), Some(Location::Clear));
        assert_eq!(map_lab.get(&(1, 0)), Some(Location::Obstacle));
        assert_eq!(map_lab.get(&(2, 0)), Some(Location::Clear));
        assert_eq!(map_lab.get(&(0, 1)), Some(Location::Obstacle));
        assert_eq!(map_lab.get(&(1, 1)), Some(Location::Clear));
        assert_eq!(map_lab.get(&(2, 1)), Some(Location::Obstacle));
        assert_eq!(map_lab.get(&(3, 0)), None);
        assert_eq!(map_lab.get(&(0, 2)), None);
        assert_eq!(map_lab.get(&(1, 2)), None);
        assert_eq!(map_lab.get(&(3, 1)), None);
    }

    #[test]
    fn test_map_until_end_wide_line() {
        let map_lab = MapLab {
            height: 1,
            width: 5,
            buffer: vec![Location::Clear; 5],
        };
        assert_eq!(
            map_lab.until_obstacle(&(0, 0), Direction::Right),
            ((4, 0), StopReason::EndOfMap)
        );
        assert_eq!(
            map_lab.until_obstacle(&(4, 0), Direction::Left),
            ((0, 0), StopReason::EndOfMap)
        );
    }

    #[test]
    fn test_map_until_obstacle_wide_line() {
        let map_lab = MapLab {
            height: 1,
            width: 6,
            buffer: vec![
                Location::Clear,
                Location::Clear,
                Location::Obstacle,
                Location::Clear,
                Location::Clear,
                Location::Clear,
            ],
        };
        assert_eq!(
            map_lab.until_obstacle(&(4, 0), Direction::Left),
            ((3, 0), StopReason::Obstacle)
        );
        assert_eq!(
            map_lab.until_obstacle(&(0, 0), Direction::Right),
            ((1, 0), StopReason::Obstacle)
        );
    }

    #[test]
    fn test_map_until_end_tall_line() {
        let map_lab = MapLab {
            height: 3,
            width: 1,
            buffer: vec![Location::Clear; 3],
        };
        assert_eq!(
            map_lab.until_obstacle(&(0, 1), Direction::Down),
            ((0, 2), StopReason::EndOfMap)
        );
        assert_eq!(
            map_lab.until_obstacle(&(0, 1), Direction::Up),
            ((0, 0), StopReason::EndOfMap)
        );
    }

    #[test]
    fn test_map_until_obstacle_tall_line() {
        let map_lab = MapLab {
            height: 5,
            width: 1,
            buffer: vec![
                Location::Clear,
                Location::Clear,
                Location::Obstacle,
                Location::Clear,
                Location::Clear,
            ],
        };
        assert_eq!(
            map_lab.until_obstacle(&(0, 4), Direction::Up),
            ((0, 3), StopReason::Obstacle)
        );
        assert_eq!(
            map_lab.until_obstacle(&(0, 0), Direction::Down),
            ((0, 1), StopReason::Obstacle)
        );
    }

    #[test]
    fn test_map_until_end_small_rectangle() {
        let map_lab = MapLab {
            height: 2,
            width: 3,
            buffer: vec![
                Location::Clear,
                Location::Clear,
                Location::Clear,
                Location::Clear,
                Location::Clear,
                Location::Clear,
            ],
        };
        assert_eq!(
            map_lab.until_obstacle(&(0, 0), Direction::Right),
            ((2, 0), StopReason::EndOfMap)
        );
        assert_eq!(
            map_lab.until_obstacle(&(2, 0), Direction::Left),
            ((0, 0), StopReason::EndOfMap)
        );
        assert_eq!(
            map_lab.until_obstacle(&(0, 1), Direction::Right),
            ((2, 1), StopReason::EndOfMap)
        );
        assert_eq!(
            map_lab.until_obstacle(&(2, 1), Direction::Left),
            ((0, 1), StopReason::EndOfMap)
        );
    }

    #[test]
    fn test_map_until_obstacle_small_rectangle() {
        let map_lab = MapLab {
            height: 3,
            width: 4,
            buffer: vec![
                Location::Clear,
                Location::Clear,
                Location::Obstacle,
                Location::Clear,
                Location::Obstacle,
                Location::Clear,
                Location::Clear,
                Location::Clear,
                Location::Clear,
                Location::Clear,
                Location::Clear,
                Location::Obstacle,
            ],
        };
        assert_eq!(
            map_lab.until_obstacle(&(3, 0), Direction::Down),
            ((3, 1), StopReason::Obstacle)
        );
        assert_eq!(
            map_lab.until_obstacle(&(2, 2), Direction::Up),
            ((2, 1), StopReason::Obstacle)
        );
        assert_eq!(
            map_lab.until_obstacle(&(3, 1), Direction::Left),
            ((1, 1), StopReason::Obstacle)
        );
        assert_eq!(
            map_lab.until_obstacle(&(0, 2), Direction::Right),
            ((2, 2), StopReason::Obstacle)
        );
        assert_eq!(
            map_lab.until_obstacle(&(0, 0), Direction::Right),
            ((1, 0), StopReason::Obstacle)
        );
        assert_eq!(
            map_lab.until_obstacle(&(1, 1), Direction::Left),
            ((1, 1), StopReason::Obstacle)
        );
    }

    #[ignore]
    fn part2_example() {
        assert_eq!(part2(&parse(PART_1_EXAMPLE)), "<RESULT>");
    }
}
