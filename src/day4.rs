use aoc_runner_derive::{aoc, aoc_generator};
#[aoc_generator(day4)]
fn parse(input: &str) -> String {
    input.to_string()
}

const XMAS: &'static str = "XMAS";
const SAMX: &'static str = "SAMX";

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
        transposed.push('\n');
    }
    assert_eq!(transposed.len(), input.len());
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
    for offset in 0..n_rows {
        for i in 1..n_cols {
            let col_idx = i + offset;
            if col_idx >= n_cols {
                break;
            }
            diagonals.push_str(&lines[i][col_idx..col_idx + 1]);
        }
        diagonals.push('\n');
    }
    assert_eq!(diagonals.len(), input.len() + n_cols);
    let diagonals = diagonals;
    occurances += diagonals.matches(XMAS).count() as u64;
    occurances += diagonals.matches(SAMX).count() as u64;

    // TODO: orthogonals

    occurances
}

#[aoc(day4, part2)]
fn part2(input: &str) -> String {
    todo!()
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

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(PART_1_EXAMPLE)), 18);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse("<EXAMPLE>")), "<RESULT>");
    }
}
