use std::collections::VecDeque;

use aoc_runner_derive::{aoc, aoc_generator};
use rustc_hash::{FxHashMap, FxHashSet};

#[aoc_generator(day21)]
fn parse(input: &str) -> [([char; 4], u64); 5] {
    let input = input.trim().replace("\r\n", "\n");
    let mut parsed = [([' '; 4], 0); 5];
    for (i, line) in input.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            parsed[i].0[j] = c;
        }
        parsed[i].1 = line[..3].parse().unwrap();
    }

    parsed
}

#[aoc(day21, part1)]
fn part1(codes: &[([char; 4], u64); 5]) -> u64 {
    let mut path_len_cache = FxHashMap::default();
    let mut total_complexity = 0;
    for (code_seq, code_val) in codes {
        let mut pad_state = PadState::new();
        let mut code_len = 0;
        for &code in code_seq {
            let target_state = PadState::from_char(code);
            let len =
                len_shortest_path_between_states(pad_state, target_state, &mut path_len_cache);
            code_len += len;
            pad_state = target_state;
        }
        total_complexity += code_len * code_val;
    }
    total_complexity
}

#[aoc(day21, part2)]
fn part2(input: &[([char; 4], u64); 5]) -> String {
    todo!()
}

fn len_shortest_path_between_states(
    from: PadState,
    to: PadState,
    cache: &mut FxHashMap<(PadState, PadState), u64>,
) -> u64 {
    let mut queue = VecDeque::new();
    queue.push_back((from, 0));
    for (state, len) in cache.iter() {
        if state.0 == from {
            queue.push_back((state.1, *len));
        }
    }
    while let Some((state, len)) = queue.pop_front() {
        if state == to {
            return len;
        }
        cache.insert((from, state), len);
        for direction in [
            DirPad::Up,
            DirPad::Down,
            DirPad::Left,
            DirPad::Right,
            DirPad::A,
        ] {
            if let Some(next_state) = state.press(direction) {
                if cache.contains_key(&(from, next_state)) {
                    continue;
                }
                queue.push_back((next_state, len + 1));
            }
        }
    }
    panic!("No path found");
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct PadState {
    num: NumPad,
    dir0: DirPad,
    dir1: DirPad,
}

impl PadState {
    fn new() -> Self {
        Self {
            num: NumPad::A(false),
            dir0: DirPad::A,
            dir1: DirPad::A,
        }
    }

    fn from_char(ch: char) -> Self {
        PadState {
            num: NumPad::from_char(ch),
            dir0: DirPad::A,
            dir1: DirPad::A,
        }
    }

    fn press(self, direction: DirPad) -> Option<PadState> {
        let mut state = self;
        state.dir1 = state.dir1.move_(direction)?;
        if direction != DirPad::A {
            return Some(state);
        }
        state.dir0 = state.dir0.move_(state.dir1)?;
        if state.dir1 != DirPad::A {
            return Some(state);
        }
        state.num = state.num.move_(state.dir0)?;
        if state.dir0 != DirPad::A {
            return Some(state);
        }
        Some(state)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum NumPad {
    _0(bool),
    _1(bool),
    _2(bool),
    _3(bool),
    _4(bool),
    _5(bool),
    _6(bool),
    _7(bool),
    _8(bool),
    _9(bool),
    A(bool),
}

impl NumPad {
    fn from_char(ch: char) -> Self {
        match ch {
            '0' => Self::_0(true),
            '1' => Self::_1(true),
            '2' => Self::_2(true),
            '3' => Self::_3(true),
            '4' => Self::_4(true),
            '5' => Self::_5(true),
            '6' => Self::_6(true),
            '7' => Self::_7(true),
            '8' => Self::_8(true),
            '9' => Self::_9(true),
            'A' => Self::A(true),
            _ => panic!("Invalid char"),
        }
    }

    fn move_(self, direction: DirPad) -> Option<Self> {
        match (self, direction) {
            (Self::_0(_), DirPad::Up) => Some(Self::_2(false)),
            (Self::_0(_), DirPad::Left) => None,
            (Self::_0(_), DirPad::Right) => Some(Self::A(false)),
            (Self::_0(_), DirPad::Down) => None,
            (Self::_0(false), DirPad::A) => Some(Self::_0(true)),
            (Self::_0(true), DirPad::A) => Some(Self::_0(false)),
            (Self::_1(_), DirPad::Up) => Some(Self::_4(false)),
            (Self::_1(_), DirPad::Left) => None,
            (Self::_1(_), DirPad::Right) => Some(Self::_2(false)),
            (Self::_1(_), DirPad::Down) => None,
            (Self::_1(false), DirPad::A) => Some(Self::_1(true)),
            (Self::_1(true), DirPad::A) => Some(Self::_1(false)),
            (Self::_2(_), DirPad::Up) => Some(Self::_5(false)),
            (Self::_2(_), DirPad::Left) => Some(Self::_1(false)),
            (Self::_2(_), DirPad::Right) => Some(Self::_3(false)),
            (Self::_2(_), DirPad::Down) => Some(Self::_0(false)),
            (Self::_2(false), DirPad::A) => Some(Self::_2(true)),
            (Self::_2(true), DirPad::A) => Some(Self::_2(false)),
            (Self::_3(_), DirPad::Up) => Some(Self::_6(false)),
            (Self::_3(_), DirPad::Left) => Some(Self::_2(false)),
            (Self::_3(_), DirPad::Right) => None,
            (Self::_3(_), DirPad::Down) => Some(Self::A(false)),
            (Self::_3(false), DirPad::A) => Some(Self::_3(true)),
            (Self::_3(true), DirPad::A) => Some(Self::_3(false)),
            (Self::_4(_), DirPad::Up) => Some(Self::_7(false)),
            (Self::_4(_), DirPad::Left) => None,
            (Self::_4(_), DirPad::Right) => Some(Self::_5(false)),
            (Self::_4(_), DirPad::Down) => Some(Self::_1(false)),
            (Self::_4(false), DirPad::A) => Some(Self::_4(true)),
            (Self::_4(true), DirPad::A) => Some(Self::_4(false)),
            (Self::_5(_), DirPad::Up) => Some(Self::_8(false)),
            (Self::_5(_), DirPad::Left) => Some(Self::_4(false)),
            (Self::_5(_), DirPad::Right) => Some(Self::_6(false)),
            (Self::_5(_), DirPad::Down) => Some(Self::_2(false)),
            (Self::_5(false), DirPad::A) => Some(Self::_5(true)),
            (Self::_5(true), DirPad::A) => Some(Self::_5(false)),
            (Self::_6(_), DirPad::Up) => Some(Self::_9(false)),
            (Self::_6(_), DirPad::Left) => Some(Self::_5(false)),
            (Self::_6(_), DirPad::Right) => None,
            (Self::_6(_), DirPad::Down) => Some(Self::_3(false)),
            (Self::_6(false), DirPad::A) => Some(Self::_6(true)),
            (Self::_6(true), DirPad::A) => Some(Self::_6(false)),
            (Self::_7(_), DirPad::Up) => None,
            (Self::_7(_), DirPad::Left) => None,
            (Self::_7(_), DirPad::Right) => Some(Self::_8(false)),
            (Self::_7(_), DirPad::Down) => Some(Self::_4(false)),
            (Self::_7(false), DirPad::A) => Some(Self::_7(true)),
            (Self::_7(true), DirPad::A) => Some(Self::_7(false)),
            (Self::_8(_), DirPad::Up) => None,
            (Self::_8(_), DirPad::Left) => Some(Self::_7(false)),
            (Self::_8(_), DirPad::Right) => Some(Self::_9(false)),
            (Self::_8(_), DirPad::Down) => Some(Self::_5(false)),
            (Self::_8(false), DirPad::A) => Some(Self::_8(true)),
            (Self::_8(true), DirPad::A) => Some(Self::_8(false)),
            (Self::_9(_), DirPad::Up) => None,
            (Self::_9(_), DirPad::Left) => Some(Self::_8(false)),
            (Self::_9(_), DirPad::Right) => None,
            (Self::_9(_), DirPad::Down) => Some(Self::_6(false)),
            (Self::_9(false), DirPad::A) => Some(Self::_9(true)),
            (Self::_9(true), DirPad::A) => Some(Self::_9(false)),
            (Self::A(_), DirPad::Up) => Some(Self::_3(false)),
            (Self::A(_), DirPad::Left) => Some(Self::_0(false)),
            (Self::A(_), DirPad::Right) => None,
            (Self::A(_), DirPad::Down) => None,
            (Self::A(false), DirPad::A) => Some(Self::A(true)),
            (Self::A(true), DirPad::A) => Some(Self::A(false)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum DirPad {
    Up,
    Down,
    Left,
    Right,
    A,
}

impl DirPad {
    fn move_(self, direction: DirPad) -> Option<Self> {
        match (self, direction) {
            (Self::Up, DirPad::Up) => None,
            (Self::Up, DirPad::Left) => None,
            (Self::Up, DirPad::Right) => Some(Self::A),
            (Self::Up, DirPad::Down) => Some(Self::Down),
            (Self::Up, DirPad::A) => Some(Self::Up),
            (Self::Down, DirPad::Up) => Some(Self::Up),
            (Self::Down, DirPad::Left) => Some(Self::Left),
            (Self::Down, DirPad::Right) => Some(Self::Right),
            (Self::Down, DirPad::Down) => None,
            (Self::Down, DirPad::A) => Some(Self::Down),
            (Self::Left, DirPad::Up) => None,
            (Self::Left, DirPad::Left) => None,
            (Self::Left, DirPad::Right) => Some(Self::Down),
            (Self::Left, DirPad::Down) => None,
            (Self::Left, DirPad::A) => Some(Self::Left),
            (Self::Right, DirPad::Up) => Some(Self::A),
            (Self::Right, DirPad::Left) => Some(Self::Down),
            (Self::Right, DirPad::Right) => None,
            (Self::Right, DirPad::Down) => None,
            (Self::Right, DirPad::A) => Some(Self::Right),
            (Self::A, DirPad::Up) => None,
            (Self::A, DirPad::Left) => Some(Self::Up),
            (Self::A, DirPad::Right) => None,
            (Self::A, DirPad::Down) => Some(Self::Right),
            (Self::A, DirPad::A) => Some(Self::A),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const EXAMPLE: &str = indoc! {"
        029A
        980A
        179A
        456A
        379A
    "};

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), 126384);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE)), "<RESULT>");
    }
}
