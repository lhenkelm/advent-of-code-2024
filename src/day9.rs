use std::iter;

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DenseDiskValue {
    Empty(u8),
    Full(u8),
}

#[aoc_generator(day9)]
fn parse(input: &str) -> Vec<DenseDiskValue> {
    input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).expect("Invalid digit") as u8)
        .enumerate()
        .filter_map(|(idx, n_blocks)| match (idx % 2, n_blocks) {
            (_, 0) => None,
            (0, n_blocks) => Some(DenseDiskValue::Full(n_blocks)),
            (1, n_blocks) => Some(DenseDiskValue::Empty(n_blocks)),
            _ => unreachable!(),
        })
        .collect()
}

#[aoc(day9, part1)]
fn part1(input: &[DenseDiskValue]) -> u64 {
    // a) Expand the compressed disk map representation
    // 5 is not optimised, just guessed (files/empties can have beetween 1 and 9 blocks)
    let mut sparse_disk_map = Vec::with_capacity(input.len() * 5);
    let mut current_file_id = 0u64;
    for val in input {
        match val {
            DenseDiskValue::Full(n_blocks) => {
                sparse_disk_map
                    .extend(iter::repeat(Some(current_file_id)).take(*n_blocks as usize));
                current_file_id += 1;
            }
            DenseDiskValue::Empty(n_blocks) => {
                sparse_disk_map.extend(iter::repeat(None).take(*n_blocks as usize));
            }
        }
    }
    let sparse_disk_map = sparse_disk_map;

    // b) Find the compact allocation of files to blocks

    // this one walks the disk map from the start, to either copy existing file IDs,
    // or to identify a gap where a block from a file on the back may be inserted
    let mut fwd_iter = sparse_disk_map.iter().enumerate();
    // this one walks the disk map from the end, to find blocks of files that can be moved
    // earlier to compress the used space
    let mut bwd_iter = fwd_iter
        .clone()
        .filter_map(|(idx_bwd, id_opt)| (*id_opt).map(|id| (idx_bwd, id)));
    // another vector to store the result of file-compacting.
    // Maybe there is a way to avoid the extra allocation?
    // 3 is another uninformed guess based on what the example looks like
    let mut compressed_disk_map = Vec::with_capacity(input.len() * 3);
    'outer: loop {
        let (idx_bwd, next_file_back) = bwd_iter
            .next_back()
            .expect("should never retreat this one that far");

        loop {
            let (idx_fwd, next_fwd) = fwd_iter
                .next()
                .expect("should never advance this one that far");
            match next_fwd {
                None => (),
                Some(file_id) => compressed_disk_map.push(*file_id),
            }
            if idx_fwd >= idx_bwd {
                break 'outer;
            } else if next_fwd.is_none() {
                break;
            }
        }
        compressed_disk_map.push(next_file_back);
    }

    // c) compute the checksum
    compressed_disk_map
        .iter()
        .enumerate()
        .fold(0u64, |partial, (next_idx, next_file_id)| {
            partial + next_idx as u64 * next_file_id
        })
}

#[aoc(day9, part2)]
fn part2(input: &[DenseDiskValue]) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    const PART_1_EXAMPLE: &str = indoc! {"
        2333133121414131402
    "};

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(PART_1_EXAMPLE)), 1928u64);
    }

    #[test]
    fn part_1_example_parse() {
        use DenseDiskValue::*;
        assert_eq!(
            parse(PART_1_EXAMPLE),
            vec![
                Full(2),
                Empty(3),
                Full(3),
                Empty(3),
                Full(1),
                Empty(3),
                Full(3),
                Empty(1),
                Full(2),
                Empty(1),
                Full(4),
                Empty(1),
                Full(4),
                Empty(1),
                Full(3),
                Empty(1),
                Full(4),
                // Empty(0) or Full(0) can be skipped in this representation
                Full(2),
            ],
        );
    }

    #[ignore]
    fn part2_example() {
        assert_eq!(part2(&parse(PART_1_EXAMPLE)), "<RESULT>");
    }
}
