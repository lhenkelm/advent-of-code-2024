use std::usize;

use aoc_runner_derive::{aoc, aoc_generator};
#[aoc_generator(day4, part1)]
fn parse(input: &str) -> String {
    input.trim().to_string()
}

const XMAS: &'static str = "XMAS";
const SAMX: &'static str = "SAMX";

fn count_non_whitespace(s: &str) -> usize {
    s.chars().filter(|c| !c.is_whitespace()).count()
}

#[aoc(day4, part1)]
fn part1(input: &str) -> u64 {
    let mut occurances = input.matches(XMAS).count() as u64;
    occurances += input.matches(SAMX).count() as u64;

    let lines: Vec<&str> = input.lines().collect();
    let n_cols = lines[0].len();
    let n_rows = lines.len();

    let mut transposed = String::with_capacity(input.len());
    for col in 0..n_cols {
        for &line_ in lines.iter() {
            transposed.push_str(&line_[col..col + 1]);
        }
        if col < n_cols - 1 {
            transposed.push('\n');
        }
    }
    assert_eq!(
        count_non_whitespace(&transposed),
        count_non_whitespace(&input)
    );
    let transposed = transposed;
    occurances += transposed.matches(XMAS).count() as u64;
    occurances += transposed.matches(SAMX).count() as u64;

    let mut diagonals = String::with_capacity(input.len() + n_cols);
    for offset in (0..n_rows).rev() {
        for i in 0..n_cols {
            let row_idx = i + offset;
            if row_idx >= n_rows {
                break;
            }
            diagonals.push_str(&lines[row_idx][i..i + 1]);
        }
        diagonals.push('\n');
    }
    for offset in 1..n_cols {
        for i in 0..n_rows {
            let col_idx = i + offset;
            if col_idx >= n_cols || i > n_rows - 1 {
                break;
            }
            diagonals.push_str(&lines[i][col_idx..col_idx + 1]);
        }
        if offset < n_cols - 1 {
            diagonals.push('\n');
        }
    }
    assert_eq!(
        count_non_whitespace(&diagonals),
        count_non_whitespace(&input)
    );
    let diagonals = diagonals;
    occurances += diagonals.matches(XMAS).count() as u64;
    occurances += diagonals.matches(SAMX).count() as u64;

    let mut orthogonals = String::with_capacity(input.len() + n_cols);
    for offset in (0..n_rows).rev() {
        for i in 0..n_cols {
            let row_idx = i + offset;
            if row_idx >= n_rows {
                break;
            }
            let rev_i = n_cols - 1 - i;
            orthogonals.push_str(&lines[row_idx][rev_i..rev_i + 1]);
        }
        orthogonals.push('\n');
    }
    for offset in 1..n_cols {
        for i in 0..n_rows {
            let col_idx = i + offset;
            if col_idx >= n_cols || i > n_rows - 1 {
                break;
            }
            let rev_col_idx = n_cols - 1 - col_idx;
            orthogonals.push_str(&lines[i][rev_col_idx..rev_col_idx + 1]);
        }
        if offset < n_cols - 1 {
            orthogonals.push('\n');
        }
    }
    assert_eq!(
        count_non_whitespace(&orthogonals),
        count_non_whitespace(&input)
    );
    let orthogonals = orthogonals;
    occurances += orthogonals.matches(XMAS).count() as u64;
    occurances += orthogonals.matches(SAMX).count() as u64;

    occurances
}

struct CharMat {
    n_rows: usize,
    n_cols: usize,
    buffer: String,
}

impl CharMat {
    fn from_str(source: &str) -> CharMat {
        let mut lines = source.lines();
        let line_0 = lines.next().expect("got empty source");
        let n_cols = line_0.trim().len(); // assumes ASCII, assumes no ws inside lines
        let n_rows = 1 + lines.count();
        let res = CharMat {
            n_rows,
            n_cols,
            buffer: source.split_ascii_whitespace().collect(),
        };
        assert_eq!(res.n_rows * res.n_cols, res.buffer.len());
        res
    }

    fn get(&self, row: usize, col: usize) -> &str {
        let idx = self.flat_idx(col, row);
        match idx {
            // silly default to avoid standard option handling
            None => "",
            // assumes ASCII, silly default to avoid standard option handling
            Some(idx) => &self.buffer.get(idx..idx + 1).unwrap_or(""),
        }
    }

    fn flat_idx(&self, row: usize, col: usize) -> Option<usize> {
        if col >= self.n_cols {
            return None;
        }
        if row >= self.n_rows {
            return None;
        }
        Some(col * self.n_rows + row)
    }

    fn deep_idx(&self, flat_idx: usize) -> (usize, usize) {
        assert!(flat_idx < self.buffer.len());
        let row = flat_idx / self.n_cols;
        let col = flat_idx % self.n_cols;
        debug_assert!(row < self.n_rows);
        (row, col)
    }

    // clean solution would return iterator with lifetime to avoid alloc
    fn match_indices(&self, pat: &str) -> Vec<(usize, usize)> {
        self.buffer
            .match_indices(pat)
            .map(|(match_idx, _)| self.deep_idx(match_idx))
            .collect()
    }
}

#[aoc_generator(day4, part2)]
fn parse_part2(input: &str) -> CharMat {
    CharMat::from_str(input.trim())
}

#[aoc(day4, part2)]
fn part2(input: &CharMat) -> u64 {
    let a_matches = input.match_indices("A");
    a_matches
        .iter()
        .filter(|(r, c)| {
            r > &0_usize
                && c > &0_usize
                && ((input.get(r - 1, c - 1) == "M" && input.get(r + 1, c + 1) == "S")
                    || (input.get(r - 1, c - 1) == "S" && input.get(r + 1, c + 1) == "M"))
                && ((input.get(r - 1, c + 1) == "M" && input.get(r + 1, c - 1) == "S")
                    || (input.get(r - 1, c + 1) == "S" && input.get(r + 1, c - 1) == "M"))
        })
        .count() as u64
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    const PART_1_EXAMPLE: &'static str = indoc! {"
        MMMSXXMASM
        MSAMXMSMSA
        AMXSXMAAMM
        MSAMASMSMX
        XMASAMXAMM
        XXAMMXXAMA
        SMSMSASXSS
        SAXAMASAAA
        MAMMMXMMMM
        MXMXAXMASX
    "};

    const PART_1_SMALL_EXAMPLE: &'static str = indoc! {"
        ..X...
        .SAMX.
        .A..A.
        XMAS.S
        .X....
    "};

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(PART_1_EXAMPLE)), 18);
    }

    #[test]
    fn part1_small_example() {
        assert_eq!(part1(&parse(PART_1_SMALL_EXAMPLE)), 4);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_part2(PART_1_EXAMPLE)), 9);
    }

    #[test]
    fn part2_xmas1() {
        let input = indoc! {"
            M.M
            .A.
            S.S
        "};
        assert_eq!(part2(&parse_part2(&input)), 1);
    }
    #[test]
    fn part2_xmas2() {
        let input = indoc! {"
            M.S
            .A.
            M.S
        "};
        assert_eq!(part2(&parse_part2(&input)), 1);
    }

    #[test]
    fn part2_xmas3() {
        let input = indoc! {"
            S.M
            .A.
            S.M
        "};
        assert_eq!(part2(&parse_part2(&input)), 1);
    }
    #[test]
    fn part2_xmas4() {
        let input = indoc! {"
            S.S
            .A.
            M.M
        "};
        assert_eq!(part2(&parse_part2(&input)), 1);
    }
}
