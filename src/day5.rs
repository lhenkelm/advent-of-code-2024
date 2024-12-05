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
    todo!()
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
}
