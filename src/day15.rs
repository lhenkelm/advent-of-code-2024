use std::{
    fmt::{self, Display, Formatter},
    iter,
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
    let mut robo_at = warehouse.robot_pos();
    for dir in instructions {
        let next_at = robo_at + dir.vector();
        match warehouse[next_at] {
            Occupant::Empty => {
                warehouse[next_at] = Occupant::Robot;
                warehouse[robo_at] = Occupant::Empty;
                robo_at = next_at;
            }
            Occupant::Wall => {
                continue;
            }
            Occupant::Box => {
                if let Some(next_empty) =
                    warehouse.find_towards(next_at, dir.vector(), |(_, occupant)| {
                        *occupant == Occupant::Empty || *occupant == Occupant::Wall
                    })
                {
                    let empty_cand = warehouse[next_empty];
                    if empty_cand == Occupant::Wall {
                        continue;
                    }
                    debug_assert_eq!(empty_cand, Occupant::Empty);
                    warehouse[next_empty] = Occupant::Box;
                    warehouse[next_at] = Occupant::Robot;
                    warehouse[robo_at] = Occupant::Empty;
                    robo_at = next_at;
                }
            }
            Occupant::Robot => unreachable!(),
        }
    }

    warehouse
        .enumerate_occupants()
        .filter(|(_, occ)| *occ == Occupant::Box)
        .map(|(point, _)| (point.y * 100 + point.x) as u64)
        .sum()
}

#[aoc(day15, part2)]
fn part2((initial_warehouse, instructions): &(Grid, Vec<Direction>)) -> u64 {
    let mut warehouse = GridPt2::from_grid(initial_warehouse.clone());
    let mut robo_at = warehouse.robot_pos();
    for &dir in instructions {
        let next_at = robo_at + dir.vector();
        match warehouse[next_at] {
            OccPt2::Empty => {
                warehouse[next_at] = OccPt2::Robot;
            }
            OccPt2::Wall => continue,
            OccPt2::LBox | OccPt2::RBox => {
                if !can_push(next_at, false, dir, &warehouse) {
                    continue;
                }
                push(next_at, false, dir, OccPt2::Robot, &mut warehouse);
            }
            OccPt2::Robot => unreachable!(),
        }
        warehouse[robo_at] = OccPt2::Empty;
        robo_at = next_at;
    }
    warehouse
        .enumerate_occupants()
        .filter(|(_, occ)| *occ == OccPt2::LBox)
        .map(|(point, _)| (point.y * 100 + point.x) as u64)
        .sum()
}

fn can_push(push_at: Point, from_this_box: bool, dir: Direction, warehouse: &GridPt2) -> bool {
    let occupant = warehouse[push_at];
    match occupant {
        OccPt2::Empty => true,
        OccPt2::Wall => false,
        OccPt2::LBox | OccPt2::RBox => {
            let check_towards = match occupant {
                OccPt2::LBox => Direction::East,
                OccPt2::RBox => Direction::West,
                _ => unreachable!(),
            };
            let pushing_towards_other = check_towards == dir;
            let behind = can_push(
                push_at + dir.vector(),
                pushing_towards_other,
                dir,
                warehouse,
            );
            if from_this_box || pushing_towards_other {
                return behind;
            }
            let other_half = can_push(push_at + check_towards.vector(), true, dir, warehouse);
            behind && other_half
        }
        OccPt2::Robot => unreachable!(),
    }
}

fn push(
    push_at: Point,
    from_this_box: bool,
    dir: Direction,
    from_occ: OccPt2,
    warehouse: &mut GridPt2,
) {
    debug_assert_ne!(from_occ, OccPt2::Wall);
    let occupant = warehouse[push_at];
    match occupant {
        OccPt2::Empty => (),
        OccPt2::LBox | OccPt2::RBox => {
            let also_push = match occupant {
                OccPt2::LBox => Direction::East,
                OccPt2::RBox => Direction::West,
                _ => unreachable!(),
            };
            let pushing_towards_other = also_push == dir;
            push(
                push_at + dir.vector(),
                pushing_towards_other,
                dir,
                occupant,
                warehouse,
            );
            if !(from_this_box || pushing_towards_other) {
                push(
                    push_at + also_push.vector(),
                    true,
                    dir,
                    OccPt2::Empty,
                    warehouse,
                );
            }
        }
        OccPt2::Wall => unreachable!(),
        OccPt2::Robot => unreachable!(),
    }
    warehouse[push_at] = from_occ
}

#[derive(Clone)]
struct GridPt2 {
    data: Vec<OccPt2>,
    height: usize,
    width: usize,
}

impl Index<Point> for GridPt2 {
    type Output = OccPt2;

    fn index(&self, index: Point) -> &Self::Output {
        &self.data[self.flat_index(index)]
    }
}

impl IndexMut<Point> for GridPt2 {
    fn index_mut(&mut self, index: Point) -> &mut Self::Output {
        let idx = self.flat_index(index);
        &mut self.data[idx]
    }
}

impl GridPt2 {
    fn from_grid(grid_pt1: Grid) -> GridPt2 {
        let data: Vec<OccPt2> = grid_pt1
            .data
            .iter()
            .flat_map(|&occ| OccPt2::from_occupant(occ))
            .collect();
        let width = grid_pt1.width * 2;
        let height = grid_pt1.height;
        debug_assert_eq!(data.len(), width * height);
        GridPt2 {
            data,
            width,
            height,
        }
    }

    fn robot_pos(&self) -> Point {
        // FIXME: this is quite suboptimal, makes me prefer the keeping-the-agent-
        // -separately approach from Guard Gallivant
        let flat_idx = self
            .data
            .iter()
            .enumerate()
            .find(|(_, occ)| **occ == OccPt2::Robot)
            .unwrap()
            .0;
        self.point_index(flat_idx)
    }

    fn point_index(&self, index: usize) -> Point {
        debug_assert!(index < self.data.len());
        Point {
            x: index % self.width,
            y: index / self.width,
        }
    }

    fn flat_index(&self, index: Point) -> usize {
        debug_assert!(index.x < self.width);
        debug_assert!(index.y < self.height);
        index.x + index.y * self.width
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

    fn enumerate_occupants(&self) -> impl Iterator<Item = (Point, OccPt2)> + '_ {
        self.data
            .iter()
            .enumerate()
            .map(|(idx, &occ)| (self.point_index(idx), occ))
    }
}

impl Display for GridPt2 {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.format())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum OccPt2 {
    Wall,
    Empty,
    LBox,
    RBox,
    Robot,
}

impl OccPt2 {
    fn from_occupant(occ_pt1: Occupant) -> impl Iterator<Item = OccPt2> {
        match occ_pt1 {
            Occupant::Wall => iter::once(OccPt2::Wall).chain(iter::once(OccPt2::Wall)),
            Occupant::Empty => iter::once(OccPt2::Empty).chain(iter::once(OccPt2::Empty)),
            Occupant::Box => iter::once(OccPt2::LBox).chain(iter::once(OccPt2::RBox)),
            Occupant::Robot => iter::once(OccPt2::Robot).chain(iter::once(OccPt2::Empty)),
        }
    }

    fn char(&self) -> char {
        match self {
            OccPt2::Empty => '.',
            OccPt2::Wall => '#',
            OccPt2::LBox => '[',
            OccPt2::RBox => ']',
            OccPt2::Robot => '@',
        }
    }
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

    fn find_towards<P>(&self, from: Point, towards: Vector, predicate: P) -> Option<Point>
    where
        P: FnMut(&(Point, Occupant)) -> bool,
    {
        let found = self.enumerate_towards(from, towards).find(predicate)?;
        Some(found.0)
    }

    fn enumerate_towards(
        &self,
        from: Point,
        towards: Vector,
    ) -> impl Iterator<Item = (Point, Occupant)> + '_ {
        self.iter_towards(from, towards)
            .enumerate()
            .map(move |(steps, occupant)| (from + towards * steps, *occupant))
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
        let pt = self.index.checked_add(self.towards)?;
        if pt.x >= self.grid.width {
            return None;
        }
        if pt.y >= self.grid.width {
            return None;
        }
        let result = &self.grid[self.index];
        self.index = pt;
        Some(result)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

#[derive(Debug, Clone, Copy)]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(BIG_EXAMPLE)), 9021);
    }
}
