use std::{
    fmt::{self, Display, Formatter},
    ops::{Add, Index, IndexMut, Mul},
};

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools; // for next_tuple

#[aoc_generator(day15)]
fn parse(input: &str) -> (Grid, Vec<Direction>) {
    let (map_str, instruct_str) = input.trim().split("\n\n").next_tuple().unwrap();

    let mut map = Vec::with_capacity(input.len());
    let mut width = None;
    let mut height = 0;
    for line in map_str.lines() {
        height += 1;
        if width.is_none() {
            width = Some(line.len());
        }
        for c in line.chars() {
            map.push(
                Occupant::from_char(c)
                    .unwrap_or_else(|| panic!("Unexpected warehouse occupant: '{}'", c)),
            );
        }
    }
    let width = width.unwrap();
    debug_assert_eq!(width * height, map.len());
    let grid = Grid {
        data: map,
        height,
        width,
    };

    let instructions = instruct_str
        .chars()
        .filter(|c| !c.is_whitespace())
        .map(|c| {
            Direction::from_char(c).unwrap_or_else(|| panic!("Unexpected robot instruction: {}", c))
        })
        .collect();
    (grid, instructions)
}

#[aoc(day15, part1)]
fn part1((initial_warehouse, instructions): &(Grid, Vec<Direction>)) -> u64 {
    let mut warehouse = initial_warehouse.clone();
    for dir in instructions {
        println!("{}, then {}", &warehouse, dir);
        // FIXME: optimize here by having `robo_at` be mutable state between iterations,
        // only searching once, initially?
        let robo_at = warehouse.robot_pos();
        let next_at = robo_at.clone() + dir.vector();
        match warehouse[next_at.clone()] {
            Occupant::Empty => {
                warehouse[next_at] = Occupant::Robot;
                warehouse[robo_at] = Occupant::Empty;
            }
            Occupant::Wall => (),
            Occupant::Box => {
                if let Some(next_empty) =
                    warehouse.find_towards(next_at.clone(), dir.vector(), Occupant::Empty)
                {
                    debug_assert_eq!(warehouse[next_empty.clone()], Occupant::Empty);
                    warehouse[next_empty] = Occupant::Box;
                    warehouse[next_at] = Occupant::Robot;
                    warehouse[robo_at] = Occupant::Empty;
                }
            }
            Occupant::Robot => unreachable!(),
        }
    }
    println!("{}", &warehouse);

    warehouse
        .enumerate_occupants()
        .filter(|(_, occ)| *occ == Occupant::Box)
        .map(|(point, _)| (point.y * 100 + point.x) as u64)
        .sum()
}

#[aoc(day15, part2)]
fn part2((initial_warehouse, instructions): &(Grid, Vec<Direction>)) -> String {
    todo!()
}

#[derive(Clone)]
struct Grid {
    data: Vec<Occupant>,
    height: usize,
    width: usize,
}

impl Index<Point> for Grid {
    type Output = Occupant;

    fn index(&self, index: Point) -> &Self::Output {
        &self.data[self.flat_index(index)]
    }
}

impl IndexMut<Point> for Grid {
    fn index_mut(&mut self, index: Point) -> &mut Self::Output {
        let idx = self.flat_index(index);
        &mut self.data[idx]
    }
}

impl Grid {
    fn flat_index(&self, index: Point) -> usize {
        debug_assert!(index.x < self.width);
        debug_assert!(index.y < self.height);
        index.x + index.y * self.width
    }

    fn point_index(&self, index: usize) -> Point {
        debug_assert!(index < self.data.len());
        Point {
            x: index % self.width,
            y: index / self.width,
        }
    }

    fn robot_pos(&self) -> Point {
        // FIXME: this is quite suboptimal, makes me prefer the keeping-the-agent-
        // -separately approach from Guard Gallivant
        let flat_idx = self
            .data
            .iter()
            .enumerate()
            .find(|(_, occ)| **occ == Occupant::Robot)
            .unwrap()
            .0;
        self.point_index(flat_idx)
    }

    fn find_towards(&self, from: Point, towards: Vector, occupant: Occupant) -> Option<Point> {
        let found = self
            .enumerate_towards(from, towards)
            .find(|(_, occ)| *occ == occupant)?;
        Some(found.0)
    }

    fn enumerate_towards(
        &self,
        from: Point,
        towards: Vector,
    ) -> impl Iterator<Item = (Point, Occupant)> + '_ {
        self.iter_towards(from.clone(), towards.clone())
            .enumerate()
            .map(move |(steps, occupant)| (from.clone() + towards.clone() * steps, *occupant))
    }

    fn iter_towards(&self, from: Point, towards: Vector) -> VectorIterator<'_> {
        VectorIterator {
            index: from,
            towards,
            grid: self,
        }
    }

    fn enumerate_occupants(&self) -> impl Iterator<Item = (Point, Occupant)> + '_ {
        self.data
            .iter()
            .enumerate()
            .map(|(idx, &occ)| (self.point_index(idx), occ))
    }

    fn format(&self) -> String {
        let mut result = String::with_capacity(self.height * (self.width + 1));
        for (idx, occupant) in self.data.iter().enumerate() {
            if idx % self.width == 0 && idx > 0 {
                result.push('\n');
            }
            result.push(occupant.char());
        }
        result.push('\n');
        result
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.format())
    }
}

struct VectorIterator<'a> {
    index: Point,
    towards: Vector,
    grid: &'a Grid,
}

impl<'a> Iterator for VectorIterator<'a> {
    type Item = &'a Occupant;

    fn next(&mut self) -> Option<Self::Item> {
        let pt = self.index.checked_add(self.towards.clone())?;
        if pt.x >= self.grid.width {
            return None;
        }
        if pt.y >= self.grid.width {
            return None;
        }
        self.index = pt;
        Some(&self.grid[self.index.clone()])
    }
}

#[derive(Debug, Clone)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn checked_add(&self, rhs: Vector) -> Option<Point> {
        let x = self.x as isize + rhs.dx;
        let y = self.y as isize + rhs.dy;
        if x < 0 {
            return None;
        }
        if y < 0 {
            return None;
        }
        let x = x.try_into().unwrap();
        let y = y.try_into().unwrap();
        Some(Point { x, y })
    }
}

impl Add<Vector> for Point {
    type Output = Point;

    fn add(self, rhs: Vector) -> Self::Output {
        let x = self.x as isize + rhs.dx;
        let y = self.y as isize + rhs.dy;
        debug_assert!(x > -1);
        debug_assert!(y > -1);
        let x = x as usize;
        let y = y as usize;
        Point { x, y }
    }
}

#[derive(Debug, Clone)]
struct Vector {
    dx: isize,
    dy: isize,
}

impl Mul<usize> for Vector {
    type Output = Vector;

    fn mul(self, rhs: usize) -> Self::Output {
        Self::Output {
            dx: self.dx * rhs as isize,
            dy: self.dy * rhs as isize,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Occupant {
    Wall,
    Box,
    Empty,
    Robot,
}

impl Occupant {
    fn char(&self) -> char {
        match self {
            Self::Wall => '#',
            Self::Box => 'O',
            Self::Empty => '.',
            Self::Robot => '@',
        }
    }
    fn from_char(ch: char) -> Option<Self> {
        match ch {
            '#' => Some(Self::Wall),
            'O' => Some(Self::Box),
            '.' => Some(Self::Empty),
            '@' => Some(Self::Robot),
            _ => None,
        }
    }
}

enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn vector(&self) -> Vector {
        match self {
            Self::North => Vector { dx: 0, dy: -1 },
            Self::East => Vector { dx: 1, dy: 0 },
            Self::South => Vector { dx: 0, dy: 1 },
            Self::West => Vector { dx: -1, dy: 0 },
        }
    }

    fn char(&self) -> char {
        match self {
            Self::North => '^',
            Self::East => '>',
            Self::South => 'v',
            Self::West => '<',
        }
    }
    fn from_char(ch: char) -> Option<Self> {
        match ch {
            '^' => Some(Self::North),
            '>' => Some(Self::East),
            'v' => Some(Self::South),
            '<' => Some(Self::West),
            _ => None,
        }
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.char())
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const BIG_EXAMPLE: &str = indoc! {"
            ##########
            #..O..O.O#
            #......O.#
            #.OO..O.O#
            #..O@..O.#
            #O#..O...#
            #O..O..O.#
            #.OO.O.OO#
            #....O...#
            ##########

            <vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
            vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
            ><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
            <<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
            ^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
            ^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
            >^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
            <><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
            ^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
            v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
    "};

    const SMALL_EXAMPLE: &str = indoc! {"
            ########
            #..O.O.#
            ##@.O..#
            #...O..#
            #.#.O..#
            #...O..#
            #......#
            ########

            <^^>>>vv<v>>v<<
    "};

    #[test]
    fn part1_big_example() {
        assert_eq!(part1(&parse(BIG_EXAMPLE)), 10092);
    }

    #[test]
    fn part1_small_example() {
        assert_eq!(part1(&parse(SMALL_EXAMPLE)), 2028);
    }

    #[ignore]
    fn part2_example() {
        assert_eq!(part2(&parse("<EXAMPLE>")), "<RESULT>");
    }
}
