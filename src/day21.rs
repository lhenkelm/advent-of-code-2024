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
        total_complexity += dbg!(code_len) * dbg!(code_val);
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
            num: NumPad::A,
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
    _0,
    _1,
    _2,
    _3,
    _4,
    _5,
    _6,
    _7,
    _8,
    _9,
    A,
}

impl NumPad {
    fn from_char(ch: char) -> Self {
        match ch {
            '0' => Self::_0,
            '1' => Self::_1,
            '2' => Self::_2,
            '3' => Self::_3,
            '4' => Self::_4,
            '5' => Self::_5,
            '6' => Self::_6,
            '7' => Self::_7,
            '8' => Self::_8,
            '9' => Self::_9,
            'A' => Self::A,
            _ => panic!("Invalid char"),
        }
    }

    fn move_(self, direction: DirPad) -> Option<Self> {
        match (self, direction) {
            (Self::_0, DirPad::Up) => Some(Self::_2),
            (Self::_0, DirPad::Left) => None,
            (Self::_0, DirPad::Right) => Some(Self::A),
            (Self::_0, DirPad::Down) => None,
            (Self::_0, DirPad::A) => Some(Self::_0),
            (Self::_1, DirPad::Up) => Some(Self::_4),
            (Self::_1, DirPad::Left) => None,
            (Self::_1, DirPad::Right) => Some(Self::_2),
            (Self::_1, DirPad::Down) => None,
            (Self::_1, DirPad::A) => Some(Self::_1),
            (Self::_2, DirPad::Up) => Some(Self::_5),
            (Self::_2, DirPad::Left) => Some(Self::_1),
            (Self::_2, DirPad::Right) => Some(Self::_3),
            (Self::_2, DirPad::Down) => Some(Self::_0),
            (Self::_2, DirPad::A) => Some(Self::_2),
            (Self::_3, DirPad::Up) => Some(Self::_6),
            (Self::_3, DirPad::Left) => Some(Self::_2),
            (Self::_3, DirPad::Right) => None,
            (Self::_3, DirPad::Down) => Some(Self::A),
            (Self::_3, DirPad::A) => Some(Self::_3),
            (Self::_4, DirPad::Up) => Some(Self::_7),
            (Self::_4, DirPad::Left) => None,
            (Self::_4, DirPad::Right) => Some(Self::_5),
            (Self::_4, DirPad::Down) => Some(Self::_1),
            (Self::_4, DirPad::A) => Some(Self::_4),
            (Self::_5, DirPad::Up) => Some(Self::_8),
            (Self::_5, DirPad::Left) => Some(Self::_4),
            (Self::_5, DirPad::Right) => Some(Self::_6),
            (Self::_5, DirPad::Down) => Some(Self::_2),
            (Self::_5, DirPad::A) => Some(Self::_5),
            (Self::_6, DirPad::Up) => Some(Self::_9),
            (Self::_6, DirPad::Left) => Some(Self::_5),
            (Self::_6, DirPad::Right) => None,
            (Self::_6, DirPad::Down) => Some(Self::_3),
            (Self::_6, DirPad::A) => Some(Self::_6),
            (Self::_7, DirPad::Up) => None,
            (Self::_7, DirPad::Left) => None,
            (Self::_7, DirPad::Right) => Some(Self::_8),
            (Self::_7, DirPad::Down) => Some(Self::_4),
            (Self::_7, DirPad::A) => Some(Self::_7),
            (Self::_8, DirPad::Up) => None,
            (Self::_8, DirPad::Left) => Some(Self::_7),
            (Self::_8, DirPad::Right) => Some(Self::_9),
            (Self::_8, DirPad::Down) => Some(Self::_5),
            (Self::_8, DirPad::A) => Some(Self::_8),
            (Self::_9, DirPad::Up) => None,
            (Self::_9, DirPad::Left) => Some(Self::_8),
            (Self::_9, DirPad::Right) => None,
            (Self::_9, DirPad::Down) => Some(Self::_6),
            (Self::_9, DirPad::A) => Some(Self::_9),
            (Self::A, DirPad::Up) => Some(Self::_3),
            (Self::A, DirPad::Left) => Some(Self::_0),
            (Self::A, DirPad::Right) => None,
            (Self::A, DirPad::Down) => None,
            (Self::A, DirPad::A) => Some(Self::A),
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
