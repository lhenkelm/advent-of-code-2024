use aoc_runner_derive::{aoc, aoc_generator};
#[aoc_generator(day4)]
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

#[aoc(day4, part2)]
fn part2(input: &str) -> String {
    String::new()
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
        assert_eq!(part2(&parse("<EXAMPLE>")), "<RESULT>");
    }
}
