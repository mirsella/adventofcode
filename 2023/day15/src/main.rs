use std::{collections::HashMap, ops::Index};

use indexmap::IndexMap;

fn hash(input: &str) -> usize {
    input
        .chars()
        .fold(0, |acc, c| ((acc + c as usize) * 17) % 256)
}
fn part1(input: &str) -> usize {
    input.trim().split(',').map(hash).sum()
}
fn part2(input: &str) -> usize {
    let mut boxes = Vec::with_capacity(256);
    for _ in 0..256 {
        boxes.push(IndexMap::new())
    }
    for word in input.trim().split(',') {
        let operation = word.find(['=', '-']).unwrap();
        let (lens, value_with_del) = word.split_at(operation);
        let value = &value_with_del[1..];
        let h = hash(lens);
        if word.chars().nth(operation).unwrap() == '-' {
            boxes[h].shift_remove(lens);
        } else {
            boxes[h].insert(lens, value);
        }
    }
    boxes
        .iter()
        .enumerate()
        .map(|(boxi, h)| {
            h.values()
                .enumerate()
                .map(|(p, value)| (boxi + 1) * (p + 1) * value.parse::<usize>().expect(value))
                .sum::<usize>()
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
    const INPUT: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
    #[test]
    fn part1() {
        assert_eq!(super::part1(INPUT), 1320);
    }
    #[test]
    fn part2() {
        assert_eq!(super::part2(INPUT), 145);
    }
}
