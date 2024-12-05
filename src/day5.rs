use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day5)]
fn parse(input: &str) -> (Vec<(u64, u64)>, Vec<Vec<u64>>) {
    let mut rules = Vec::with_capacity(input.len() / 2);
    let mut in_rules = true;
    let mut updates = Vec::new();
    for line in input.trim().lines() {
        if line.is_empty() {
            in_rules = false;
            continue;
        }
        if in_rules {
            let mut num_iter = line
                .split("|")
                .map(|c| c.parse::<u64>().expect("not a number"));
            let first = num_iter.next().unwrap();
            let second = num_iter.next().unwrap();
            rules.push((first, second));
        } else {
            let pages: Vec<u64> = line
                .split(",")
                .map(|c| c.parse::<u64>().expect("not a number"))
                .collect();
            updates.push(pages);
        }
    }
    (rules, updates)
}

fn part_1_check_update(update: &[u64], rules: &[(u64, u64)]) -> bool {
    let mut page_position_by_id = HashMap::with_capacity(update.len());
    for (pos, &id) in update.iter().enumerate() {
        page_position_by_id.insert(id, pos);
    }
    let page_position_by_id = page_position_by_id;

    for (first_id, second_id) in rules {
        let first_pos = match page_position_by_id.get(first_id) {
            Some(pos) => pos,
            None => continue,
        };
        let second_pos = match page_position_by_id.get(second_id) {
            Some(pos) => pos,
            None => continue,
        };

        if first_pos > second_pos {
            return false;
        }
    }
    true
}

#[aoc(day5, part1)]
fn part1((rules, updates): &(Vec<(u64, u64)>, Vec<Vec<u64>>)) -> u64 {
    updates
        .iter()
        .filter(|&update| part_1_check_update(update, rules))
        .map(|update| update[update.len() / 2])
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const PART_1_EXAMPLE: &str = indoc! {"
        47|53
        97|13
        97|61
        97|47
        75|29
        61|13
        75|53
        29|13
        97|29
        53|29
        61|53
        97|53
        61|29
        47|13
        75|47
        97|75
        47|61
        75|61
        47|29
        75|13
        53|13

        75,47,61,53,29
        97,61,53,29,13
        75,29,13
        75,97,47,61,53
        61,13,29
        97,13,75,29,47
    "};

    #[test]
    fn part1_parse_example() {
        let expected_rules = vec![
            (47, 53),
            (97, 13),
            (97, 61),
            (97, 47),
            (75, 29),
            (61, 13),
            (75, 53),
            (29, 13),
            (97, 29),
            (53, 29),
            (61, 53),
            (97, 53),
            (61, 29),
            (47, 13),
            (75, 47),
            (97, 75),
            (47, 61),
            (75, 61),
            (47, 29),
            (75, 13),
            (53, 13),
        ];
        let expected_updates = vec![
            vec![75, 47, 61, 53, 29],
            vec![97, 61, 53, 29, 13],
            vec![75, 29, 13],
            vec![75, 97, 47, 61, 53],
            vec![61, 13, 29],
            vec![97, 13, 75, 29, 47],
        ];
        let (rules, updates) = parse(PART_1_EXAMPLE);
        assert_eq!(rules, expected_rules);
        assert_eq!(updates, expected_updates);
    }

    #[test]
    fn part1_example() {
        let input = parse(PART_1_EXAMPLE);
        assert_eq!(part1(&input), 143);
    }
}
