use std::collections::{HashMap, HashSet};

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

#[aoc(day5, part1)]
fn part1((rules, updates): &(Vec<(u64, u64)>, Vec<Vec<u64>>)) -> u64 {
    updates
        .iter()
        .filter(|&update| part_1_check_update(update, rules))
        .map(|update| update[update.len() / 2])
        .sum()
}

fn part_1_check_update(update: &[u64], rules: &[(u64, u64)]) -> bool {
    let page_position_by_id = update_to_page_map(update);
    check_page_map(&page_position_by_id, rules)
}

fn update_to_page_map(update: &[u64]) -> HashMap<u64, usize> {
    let mut page_position_by_id = HashMap::with_capacity(update.len());
    for (pos, &id) in update.iter().enumerate() {
        page_position_by_id.insert(id, pos);
    }
    page_position_by_id
}

fn check_page_map(page_position_by_id: &HashMap<u64, usize>, rules: &[(u64, u64)]) -> bool {
    for (first_id, second_id) in rules {
        let &first_pos = match page_position_by_id.get(first_id) {
            Some(pos) => pos,
            None => continue,
        };
        let &second_pos = match page_position_by_id.get(second_id) {
            Some(pos) => pos,
            None => continue,
        };

        if first_pos > second_pos {
            return false;
        }
    }
    true
}

#[aoc(day5, part1, sorting)]
fn part1_sorting((rules, updates): &(Vec<(u64, u64)>, Vec<Vec<u64>>)) -> u64 {
    updates
        .iter()
        .filter(|&update| part_1_sorting_check_update(update, rules))
        .map(|update| update[update.len() / 2])
        .sum()
}

fn part_1_sorting_check_update(update: &[u64], rules: &[(u64, u64)]) -> bool {
    sort_update(update, rules) == update
}

fn sort_update(update: &[u64], rules: &[(u64, u64)]) -> Vec<u64> {
    let relevant_rules = rules
        .iter()
        .filter(|(a, b)| update.contains(a) && update.contains(b));
    let mut rule_map = vec![vec![]; (*update.iter().max().unwrap()) as usize + 1];
    for (before, after) in relevant_rules {
        rule_map[*before as usize].push(*after);
    }
    let rule_map = rule_map;
    let mut sorted = update.to_vec();
    sorted.sort_by(|l, r| {
        if rule_map[*r as usize].contains(l) {
            std::cmp::Ordering::Greater
        } else {
            std::cmp::Ordering::Less
        }
    });
    sorted
}

#[aoc(day5, part2)]
fn part2((rules, updates): &(Vec<(u64, u64)>, Vec<Vec<u64>>)) -> u64 {
    let mut rules_by_pre = HashMap::with_capacity(rules.len());
    for (pre, post) in rules {
        rules_by_pre.entry(pre).or_insert_with(Vec::new).push(post);
    }
    updates
        .iter()
        .filter(|&update| !part_1_check_update(update, rules))
        .map(|update| part2_fix_update(update, rules))
        .map(|update| update[update.len() / 2])
        .sum()
}

fn part2_fix_update(update: &[u64], rules: &[(u64, u64)]) -> Vec<u64> {
    let mut page_position_by_id = update_to_page_map(update);
    let mut skip_rules = HashSet::with_capacity(rules.len());
    {
        for (first_id, second_id) in rules {
            if skip_rules.contains(&(*first_id, *second_id)) {
                continue;
            }
            let &first_pos = match page_position_by_id.get(first_id) {
                Some(pos) => pos,
                None => {
                    skip_rules.insert((*first_id, *second_id));
                    continue;
                }
            };
            let &second_pos = match page_position_by_id.get(second_id) {
                Some(pos) => pos,
                None => {
                    skip_rules.insert((*first_id, *second_id));
                    continue;
                }
            };

            if first_pos > second_pos {
                page_position_by_id.insert(*first_id, second_pos);
                page_position_by_id.insert(*second_id, first_pos);
            }
        }
    }
    let skip_rules = skip_rules;
    let relevant_rules = if skip_rules.is_empty() {
        rules.to_vec()
    } else {
        rules
            .iter()
            .filter(|(first, second)| !skip_rules.contains(&(*first, *second)))
            .copied()
            .collect::<Vec<_>>()
    };
    loop {
        let mut changed = false;
        for (first_id, second_id) in &relevant_rules {
            let first_pos = page_position_by_id[first_id];
            let second_pos = page_position_by_id[second_id];
            if first_pos > second_pos {
                changed = true;
                page_position_by_id.insert(*first_id, second_pos);
                page_position_by_id.insert(*second_id, first_pos);
            }
        }
        if !changed {
            break;
        }
    }
    let mut new_update = vec![0u64; update.len()];
    for (id, pos) in page_position_by_id {
        new_update[pos] = id;
    }
    new_update
}

#[aoc(day5, part2, sorting)]
fn part2_sorting((rules, updates): &(Vec<(u64, u64)>, Vec<Vec<u64>>)) -> u64 {
    let mut rules_by_pre = HashMap::with_capacity(rules.len());
    for (pre, post) in rules {
        rules_by_pre.entry(pre).or_insert_with(Vec::new).push(post);
    }
    updates
        .iter()
        .filter(|&update| !part_1_check_update(update, rules))
        .map(|update| sort_update(update, rules))
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

    #[test]
    fn part1_example_sorting() {
        let input = parse(PART_1_EXAMPLE);
        assert_eq!(part1_sorting(&input), 143);
    }

    #[test]
    fn part2_example() {
        let input = parse(PART_1_EXAMPLE);
        assert_eq!(part2(&input), 123);
    }
    #[test]
    fn part2_example_sorting() {
        let input = parse(PART_1_EXAMPLE);
        assert_eq!(part2_sorting(&input), 123);
    }
}
