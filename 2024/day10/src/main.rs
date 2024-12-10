use pathfinding::matrix::Matrix;
use std::collections::{HashMap, HashSet};

fn part1(input: &str) -> usize {
    let m = Matrix::from_rows(
        input
            .lines()
            .map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as u8)),
    )
    .unwrap();
    fn parkour(m: &Matrix<u8>, trailends: &mut HashSet<(usize, usize)>, pos: (usize, usize)) {
        if m[pos] == 9 {
            trailends.insert(pos);
            return;
        }
        for neighbor in m.neighbours(pos, false) {
            if m[neighbor] == m[pos] + 1 {
                parkour(m, trailends, neighbor);
            }
        }
    }
    let mut trailheads = HashMap::new();
    let mut tmp_trailends = HashSet::new();
    for (pos, &char) in m.items() {
        if char != 0 {
            continue;
        }
        parkour(&m, &mut tmp_trailends, pos);
        trailheads.insert(pos, tmp_trailends.len());
        tmp_trailends.clear();
    }
    trailheads.values().sum()
}
fn part2(input: &str) -> usize {
    let m = Matrix::from_rows(
        input
            .lines()
            .map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as u8)),
    )
    .unwrap();
    fn parkour(
        m: &Matrix<u8>,
        trailends: &mut HashSet<Vec<(usize, usize)>>,
        mut current: Vec<(usize, usize)>,
        pos: (usize, usize),
    ) {
        current.push(pos);
        if m[pos] == 9 {
            trailends.insert(current);
            return;
        }
        for neighbor in m.neighbours(pos, false) {
            if m[neighbor] == m[pos] + 1 {
                parkour(m, trailends, current.clone(), neighbor);
            }
        }
    }
    let mut trailheads = HashMap::new();
    let mut tmp_trailends = HashSet::new();
    for (pos, &char) in m.items() {
        if char != 0 {
            continue;
        }
        parkour(&m, &mut tmp_trailends, Vec::new(), pos);
        trailheads.insert(pos, tmp_trailends.len());
        tmp_trailends.clear();
    }
    trailheads.values().sum()
}

fn main() {
    let input = include_str!("../input.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";
    #[test]
    fn part1() {
        assert_eq!(super::part1(INPUT), 36);
    }
    #[test]
    fn part2() {
        assert_eq!(super::part2(INPUT), 81);
    }
}
