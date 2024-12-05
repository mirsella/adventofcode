use std::{cmp::Ordering, collections::BTreeMap};

use itertools::Itertools;

fn part1(input: &str) -> usize {
    let split = input.split_once("\n\n").unwrap();
    // all values in the vec must appear before the key
    let ordering = {
        let mut map = BTreeMap::<usize, Vec<usize>>::new();
        split.0.lines().for_each(|l| {
            let s = l.split_once('|').unwrap();
            map.entry(s.0.parse::<usize>().unwrap())
                .or_default()
                .push(s.1.parse::<usize>().unwrap());
        });
        map
    };
    let updates = split.1.lines().map(|l| {
        l.split(',')
            .map(|s| s.parse::<usize>().unwrap())
            .collect_vec()
    });
    updates
        .filter_map(|update| {
            for i in 0..update.len() {
                for next in &update[i + 1..] {
                    if let Some(vec) = ordering.get(next) {
                        if vec.contains(&update[i]) {
                            return None;
                        }
                    }
                }
            }
            Some(update[update.len() / 2])
        })
        .sum()
}
fn part2(input: &str) -> usize {
    let split = input.split_once("\n\n").unwrap();
    // all values in the vec must appear before the key
    let ordering = {
        let mut map = BTreeMap::<usize, Vec<usize>>::new();
        split.0.lines().for_each(|l| {
            let s = l.split_once('|').unwrap();
            map.entry(s.0.parse::<usize>().unwrap())
                .or_default()
                .push(s.1.parse::<usize>().unwrap());
        });
        map
    };
    let updates = split.1.lines().map(|l| {
        l.split(',')
            .map(|s| s.parse::<usize>().unwrap())
            .collect_vec()
    });
    let unsortedupdates = updates
        .filter(|update| {
            for i in 0..update.len() {
                for next in &update[i + 1..] {
                    if let Some(vec) = ordering.get(next) {
                        if vec.contains(&update[i]) {
                            return true;
                        }
                    }
                }
            }
            false
        })
        .collect_vec();
    unsortedupdates
        .into_iter()
        .map(|mut update| {
            update.sort_unstable_by(|a, b| {
                if let Some(vec) = ordering.get(a) {
                    if vec.contains(b) {
                        return Ordering::Less;
                    }
                }
                Ordering::Greater
            });
            update[update.len() / 2]
        })
        .sum()
}
fn main() {
    let input = include_str!("../input.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "47|53
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
97,13,75,29,47";
    #[test]
    fn part1() {
        assert_eq!(super::part1(INPUT), 143);
    }
    #[test]
    fn part2() {
        assert_eq!(super::part2(INPUT), 123);
    }
}
