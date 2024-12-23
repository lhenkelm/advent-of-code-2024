use std::collections::VecDeque;
use std::fmt::Display;
use std::iter;

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use memoize::memoize;
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
    const N_DIR_PAD_ROBOTS: usize = 2;
    let mut path_len_cache = FxHashMap::default();
    let mut total_complexity = 0;
    for (code_seq, code_val) in codes {
        let mut pad_state = PadState::new(N_DIR_PAD_ROBOTS);
        let mut code_len = 0;
        for &code in code_seq {
            let target_state = PadState::from_char(code, N_DIR_PAD_ROBOTS);
            let len =
                len_shortest_path_between_states(pad_state, &target_state, &mut path_len_cache);
            code_len += len;
            pad_state = target_state;
        }
        total_complexity += code_len * code_val;
    }
    total_complexity
}

#[aoc(day21, part2)]
fn part2(input: &[([char; 4], u64); 5]) -> u64 {
    const N_DIR_PAD_ROBOTS: usize = 5;

    // 1) Find shortest path (not its length) between all immediately connected pairs
    // of a NumPad and its nearest DirPad, in terms of the button presses required from the direction
    // pad four levels above. The result is stored to be used later, to guide a greedy algorithm.
    let mut shortest_paths_numpad = FxHashMap::default();
    //  2 because we measure by button presses four levels above, and the "highest level"
    // is modelled as the lowest index in the PadState::dir vector.
    let dir_index_above = 2;
    for from in NumPad::all() {
        // TODO: Optimisation: since we visit every state every time, just store the results for the different `to`s
        // in a map, and then use that map to find the shortest path to the `to`s.
        for to in NumPad::all() {
            let mut todo = VecDeque::new();
            todo.push_back((
                PadState {
                    num: from,
                    dir: vec![DirPad::A; 3],
                },
                vec![],
            ));
            let mut visited = FxHashSet::default();
            let mut candidates = Vec::new();
            while let Some((state, path)) = todo.pop_front() {
                if !visited.insert(state.clone()) {
                    continue;
                }
                if state.num == to {
                    candidates.push((state.clone(), path.clone()));
                }
                for direction in DirPad::all() {
                    if let Some(next_state) = state.clone().press(direction) {
                        let mut new_path = path.clone();
                        new_path.push(direction);
                        todo.push_back((next_state, new_path));
                    }
                }
            }
            let shortest_path = candidates
                .iter()
                .min_by_key(|(_, path)| path.len())
                .unwrap()
                .1
                .clone();
            let mut path_one_level_up = Vec::new();
            let mut state = PadState {
                num: from,
                dir: vec![DirPad::A; 3],
            };
            for dir in shortest_path.iter() {
                state = state.clone().press(*dir).unwrap();
                if dir == &DirPad::A && state.dir[..dir_index_above].iter().all(|d| d == &DirPad::A)
                {
                    path_one_level_up.push(state.dir[dir_index_above]);
                }
            }
            shortest_paths_numpad.insert((from, to), path_one_level_up);
        }
    }
    let shortest_paths_numpad = shortest_paths_numpad;
    println!("Example best numpad paths:");
    for (ft, tt) in [
        (NumPad::A(false), NumPad::A(false)),
        (NumPad::A(false), NumPad::A(true)),
        (NumPad::A(false), NumPad::_0(false)),
        (NumPad::A(false), NumPad::_0(true)),
        (NumPad::A(false), NumPad::_3(false)),
        (NumPad::A(false), NumPad::_3(true)),
        (NumPad::A(false), NumPad::_8(true)),
        (NumPad::_1(true), NumPad::_0(false)),
    ] {
        println!(
            "{}\t=>{}\t: {}",
            ft,
            tt,
            shortest_paths_numpad[&(ft, tt)].iter().format("")
        );
    }

    // 2) Find shortest path (not its length) between all immediately connected pairs
    // of a DirPad and its nearest DirPad, in terms of the button presses required from the direction
    // pad four levels above. The result is stored to be used later, to guide a greedy algorithm.
    let mut shortest_paths_dirpad = FxHashMap::default();
    // 2 is then the index of the direction pad "immediately above" the relevant direction pad
    let dir_index_above = 2;
    for from in DirPad::all()
        .into_iter()
        .map(|d| PressableDirPad::from_dirpad(d, false))
    {
        for to in DirPad::all()
            .into_iter()
            .map(|d| PressableDirPad::from_dirpad(d, true))
        {
            let mut todo = VecDeque::new();
            todo.push_back((DirPadStack::with_lowest(from, 3), vec![]));
            let mut visited = FxHashSet::default();
            let mut candidates = Vec::new();
            while let Some((state, path)) = todo.pop_front() {
                if !visited.insert(state.clone()) {
                    continue;
                }
                if state.lowest == to {
                    candidates.push((state.clone(), path.clone()));
                }
                for direction in DirPad::all() {
                    if let Some(next_state) = state.clone().press(direction) {
                        let mut new_path = path.clone();
                        new_path.push(direction);
                        todo.push_back((next_state, new_path));
                    }
                }
            }
            let shortest_path = candidates
                .iter()
                .min_by_key(|(_, path)| path.len())
                .unwrap()
                .1
                .clone();
            let mut path_one_level_up = Vec::new();
            let mut state = DirPadStack::with_lowest(from, 3);
            for dir in shortest_path.iter() {
                state = state.clone().press(*dir).unwrap();
                if dir == &DirPad::A
                    && state.others[..dir_index_above]
                        .iter()
                        .all(|d| d == &DirPad::A)
                {
                    path_one_level_up.push(state.others[dir_index_above]);
                }
            }
            shortest_paths_dirpad.insert((from.into_dirpad(), to.into_dirpad()), path_one_level_up);
        }
    }
    let shortest_paths_dirpad = shortest_paths_dirpad;
    for (ft, tt) in [
        (DirPad::A, DirPad::A),
        (DirPad::A, DirPad::Up),
        (DirPad::A, DirPad::Right),
        (DirPad::A, DirPad::Left),
        (DirPad::Left, DirPad::Up),
    ] {
        println!(
            " {} \t=> {} \t: {}",
            ft,
            tt,
            shortest_paths_dirpad[&(ft, tt)].iter().format("")
        );
    }

    println!("line: {}", line!());
    // 3) Greedy algorithm to find the shortest paths producing the given codes.
    let mut total_complexity = 0;
    for (code_seq, code_val) in input {
        let mut code_complexity = 0;
        let mut pad_state = PadState::new(N_DIR_PAD_ROBOTS);
        for &code in code_seq {
            let target_state = PadState::from_char(code, N_DIR_PAD_ROBOTS);
            let mut path = shortest_paths_numpad[&(pad_state.num, target_state.num)].clone();
            let deep_len = length_of_path_n_robots_deep(
                path.clone(),
                N_DIR_PAD_ROBOTS,
                &shortest_paths_dirpad,
            );
            if N_DIR_PAD_ROBOTS < 6 {
                for _ in 0..N_DIR_PAD_ROBOTS {
                    let new_path = iter::once(DirPad::A)
                        .chain(path.into_iter())
                        .tuple_windows()
                        .flat_map(|(from_dir, to_dir)| {
                            shortest_paths_dirpad[&(from_dir, to_dir)].clone()
                        })
                        .collect();
                    path = new_path;
                }
                println!("path: {}", path.iter().format(""));
                for direction in path.iter() {
                    pad_state = pad_state.press(*direction).unwrap();
                }
                assert_eq!(pad_state, target_state);
                assert_eq!(path.len(), deep_len);
            }
            code_complexity += deep_len as u64;
        }
        total_complexity += code_complexity * code_val;
    }
    total_complexity
}

#[memoize(Ignore: shortest_paths_dirpad)]
fn length_of_path_n_robots_deep(
    path: Vec<DirPad>,
    n: usize,
    shortest_paths_dirpad: &FxHashMap<(DirPad, DirPad), Vec<DirPad>>,
) -> usize {
    if n == 0 {
        return path.len();
    }
    iter::once(DirPad::A)
        .chain(path.iter().copied())
        .tuple_windows()
        .map(|(from_dir, to_dir)| {
            length_of_path_n_robots_deep(
                shortest_paths_dirpad[&(from_dir, to_dir)].clone(),
                n - 1,
                shortest_paths_dirpad,
            )
        })
        .sum()
}

fn len_shortest_path_between_states(
    from: PadState,
    to: &PadState,
    cache: &mut FxHashMap<(PadState, PadState), u64>,
) -> u64 {
    let mut queue = VecDeque::new();
    queue.push_back((from.clone(), 0));
    for (states, len) in cache.iter() {
        if states.0 == from {
            queue.push_back((states.1.clone(), *len));
        }
    }
    while let Some((state, len)) = queue.pop_front() {
        if state == *to {
            return len;
        }
        cache.insert((from.clone(), state.clone()), len);
        for direction in [
            DirPad::Up,
            DirPad::Down,
            DirPad::Left,
            DirPad::Right,
            DirPad::A,
        ] {
            if let Some(next_state) = state.clone().press(direction) {
                if cache.contains_key(&(from.clone(), next_state.clone())) {
                    continue;
                }
                queue.push_back((next_state, len + 1));
            }
        }
    }
    panic!("No path found");
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct PadState {
    num: NumPad,
    dir: Vec<DirPad>,
}

impl PadState {
    fn new(n_dir_pads: usize) -> Self {
        Self {
            num: NumPad::A(false),
            dir: vec![DirPad::A; n_dir_pads],
        }
    }

    fn from_char(ch: char, n_dir_pads: usize) -> Self {
        PadState {
            num: NumPad::from_char(ch),
            dir: vec![DirPad::A; n_dir_pads],
        }
    }

    fn press(self, direction: DirPad) -> Option<PadState> {
        let mut state = self;
        *state.dir.first_mut().unwrap() = state.dir.first().unwrap().move_(direction)?;
        if direction != DirPad::A {
            return Some(state);
        }
        for i in 1..state.dir.len() {
            state.dir[i] = state.dir[i].move_(state.dir[i - 1])?;
            if state.dir[i - 1] != DirPad::A {
                return Some(state);
            }
        }
        let last_dir = *state.dir.last().unwrap();
        state.num = state.num.move_(last_dir)?;
        Some(state)
    }
}

impl Display for PadState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "@[{}|{}]", self.num, self.dir.iter().format(""))
    }
}

// TODO: clean this up by unifying with the dirpad part of PadState
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct DirPadStack {
    lowest: PressableDirPad,
    others: Vec<DirPad>,
}

impl DirPadStack {
    fn with_lowest(lowest: PressableDirPad, n: usize) -> Self {
        Self {
            lowest,
            others: vec![DirPad::A; n],
        }
    }

    fn press(self, direction: DirPad) -> Option<Self> {
        let mut state = self;
        *state.others.first_mut().unwrap() = state.others.first().unwrap().move_(direction)?;
        if direction != DirPad::A {
            return Some(state);
        }
        for i in 1..state.others.len() {
            state.others[i] = state.others[i].move_(state.others[i - 1])?;
            if state.others[i - 1] != DirPad::A {
                return Some(state);
            }
        }
        let last_other = *state.others.last().unwrap();
        state.lowest = state.lowest.move_(last_other)?;
        Some(state)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum PressableDirPad {
    Up(bool),
    Down(bool),
    Left(bool),
    Right(bool),
    A(bool),
}

impl PressableDirPad {
    fn from_dirpad(dirpad: DirPad, pressed: bool) -> Self {
        match dirpad {
            DirPad::Up => Self::Up(pressed),
            DirPad::Down => Self::Down(pressed),
            DirPad::Left => Self::Left(pressed),
            DirPad::Right => Self::Right(pressed),
            DirPad::A => Self::A(pressed),
        }
    }

    fn into_dirpad(self) -> DirPad {
        match self {
            Self::Up(_) => DirPad::Up,
            Self::Down(_) => DirPad::Down,
            Self::Left(_) => DirPad::Left,
            Self::Right(_) => DirPad::Right,
            Self::A(_) => DirPad::A,
        }
    }

    fn move_(self, direction: DirPad) -> Option<Self> {
        match (self, direction) {
            (Self::Up(_), DirPad::Up) => None,
            (Self::Up(_), DirPad::Left) => None,
            (Self::Up(_), DirPad::Right) => Some(Self::A(false)),
            (Self::Up(_), DirPad::Down) => Some(Self::Down(false)),
            (Self::Up(false), DirPad::A) => Some(Self::Up(true)),
            (Self::Up(true), DirPad::A) => Some(Self::Up(false)),
            (Self::Down(_), DirPad::Up) => Some(Self::Up(false)),
            (Self::Down(_), DirPad::Left) => Some(Self::Left(false)),
            (Self::Down(_), DirPad::Right) => Some(Self::Right(false)),
            (Self::Down(_), DirPad::Down) => None,
            (Self::Down(false), DirPad::A) => Some(Self::Down(true)),
            (Self::Down(true), DirPad::A) => Some(Self::Down(false)),
            (Self::Left(_), DirPad::Up) => None,
            (Self::Left(_), DirPad::Left) => None,
            (Self::Left(_), DirPad::Right) => Some(Self::Down(false)),
            (Self::Left(_), DirPad::Down) => None,
            (Self::Left(false), DirPad::A) => Some(Self::Left(true)),
            (Self::Left(true), DirPad::A) => Some(Self::Left(false)),
            (Self::Right(_), DirPad::Up) => Some(Self::A(false)),
            (Self::Right(_), DirPad::Left) => Some(Self::Down(false)),
            (Self::Right(_), DirPad::Right) => None,
            (Self::Right(_), DirPad::Down) => None,
            (Self::Right(false), DirPad::A) => Some(Self::Right(true)),
            (Self::Right(true), DirPad::A) => Some(Self::Right(false)),
            (Self::A(_), DirPad::Up) => None,
            (Self::A(_), DirPad::Left) => Some(Self::Up(false)),
            (Self::A(_), DirPad::Right) => None,
            (Self::A(_), DirPad::Down) => Some(Self::Right(false)),
            (Self::A(false), DirPad::A) => Some(Self::A(true)),
            (Self::A(true), DirPad::A) => Some(Self::A(false)),
        }
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

    fn format(&self) -> &str {
        match self {
            Self::_0(false) => "(0)",
            Self::_0(true) => " 0 ",
            Self::_1(false) => "(1)",
            Self::_1(true) => " 1 ",
            Self::_2(false) => "(2)",
            Self::_2(true) => " 2 ",
            Self::_3(false) => "(3)",
            Self::_3(true) => " 3 ",
            Self::_4(false) => "(4)",
            Self::_4(true) => " 4 ",
            Self::_5(false) => "(5)",
            Self::_5(true) => " 5 ",
            Self::_6(false) => "(6)",
            Self::_6(true) => " 6 ",
            Self::_7(false) => "(7)",
            Self::_7(true) => " 7 ",
            Self::_8(false) => "(8)",
            Self::_8(true) => " 8 ",
            Self::_9(false) => "(9)",
            Self::_9(true) => " 9 ",
            Self::A(false) => "(A)",
            Self::A(true) => " A ",
        }
    }

    fn all() -> [Self; 22] {
        [
            Self::_0(false),
            Self::_0(true),
            Self::_1(false),
            Self::_1(true),
            Self::_2(false),
            Self::_2(true),
            Self::_3(false),
            Self::_3(true),
            Self::_4(false),
            Self::_4(true),
            Self::_5(false),
            Self::_5(true),
            Self::_6(false),
            Self::_6(true),
            Self::_7(false),
            Self::_7(true),
            Self::_8(false),
            Self::_8(true),
            Self::_9(false),
            Self::_9(true),
            Self::A(false),
            Self::A(true),
        ]
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

impl Display for NumPad {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.format())
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
    fn all() -> [Self; 5] {
        // sorted by reverse distance from A, such that expensive ops
        // are resolved first
        [Self::Left, Self::Down, Self::Up, Self::Right, Self::A]
    }

    fn char(&self) -> char {
        match self {
            Self::Up => '^',
            Self::Down => 'v',
            Self::Left => '<',
            Self::Right => '>',
            Self::A => 'A',
        }
    }

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

impl Display for DirPad {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.char())
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
}
