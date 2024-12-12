use hashbrown::HashSet;
use itertools::Itertools;
use std::collections::BTreeSet;
// use std::collections::HashSet;
use pathfinding::{
    matrix::{directions::DIRECTIONS_4, Matrix},
    prelude::bfs_reach,
};

type Regions = HashSet<BTreeSet<(usize, usize)>>;
fn part1(input: &str) -> usize {
    let m = Matrix::from_rows(input.lines().map(str::chars)).unwrap();
    let mut regions: Regions = Default::default();
    for pos in m.keys() {
        if regions.iter().any(|r| r.contains(&pos)) {
            continue;
        }
        let region = bfs_reach(pos, |&p| m.neighbours(p, false).filter(|&n| m[n] == m[pos]))
            .collect::<BTreeSet<_>>();
        regions.insert(region);
    }
    let mut price = 0;
    for region in regions.iter() {
        let mut fences = 0;
        for &pos in region {
            let neighbours = m.neighbours(pos, false).collect_vec();
            fences += neighbours.iter().filter(|n| !region.contains(n)).count();
            fences += 4 - neighbours.len();
        }
        price += fences * region.len();
    }
    price
}

fn part2(input: &str) -> usize {
    let m = Matrix::from_rows(input.lines().map(str::chars)).unwrap();
    let mut regions: Regions = Default::default();
    for pos in m.keys() {
        if regions.iter().any(|r| r.contains(&pos)) {
            continue;
        }
        let region = bfs_reach(pos, |&p| m.neighbours(p, false).filter(|&n| m[n] == m[pos]))
            .collect::<BTreeSet<_>>();
        regions.insert(region);
    }
    let mut price = 0;
    for region in regions.iter() {
        let mut corners = 0;
        for dir in DIRECTIONS_4 {
            let mut sides = HashSet::new();
            for pos in region {
                let tmp = (pos.0 as isize + dir.0, pos.1 as isize + dir.1);
                if !region.contains(&(tmp.0 as usize, tmp.1 as usize)) {
                    sides.insert(tmp);
                }
            }
            let mut extra_sides = HashSet::new();
            for side in &sides {
                let mut tmp = (side.0 + dir.1, side.1 + dir.0);
                while sides.contains(&tmp) {
                    extra_sides.insert(tmp);
                    tmp = (tmp.0 + dir.1, tmp.1 + dir.0);
                }
            }
            corners += sides.len() - extra_sides.len();
        }
        price += corners * region.len();
    }
    price
}

fn main() {
    let input = include_str!("../input.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";
    #[test]
    fn part1() {
        assert_eq!(super::part1(INPUT), 1930);
    }
    #[test]
    fn part2() {
        assert_eq!(super::part2(INPUT), 1206);
    }
    #[test]
    fn part2_2() {
        assert_eq!(
            super::part2(
                "AAAA
BBCD
BBCC
EEEC"
            ),
            80
        );
    }
    #[test]
    fn part2_3() {
        assert_eq!(
            super::part2(
                "EEEEE
EXXXX
EEEEE
EXXXX
EEEEE"
            ),
            236
        );
    }

    #[test]
    fn part2_4() {
        assert_eq!(
            super::part2(
                "AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA"
            ),
            368
        );
    }
}
