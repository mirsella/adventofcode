use std::collections::{HashMap, HashSet};

use itertools::Itertools;

type Point = (usize, usize, usize);
type Brick = (Point, Point);
fn parse(input: &str) -> Vec<Brick> {
    input
        .lines()
        .map(|l| {
            let mut s = l
                .split(['~', ','])
                .map(|s| s.trim().parse::<usize>().unwrap());
            (
                s.by_ref().take(3).collect_tuple().unwrap(),
                s.collect_tuple().unwrap(),
            )
        })
        .collect_vec()
}

fn intersects((min1, max1): (usize, usize), (min2, max2): (usize, usize)) -> bool {
    (min1..=max1).contains(&min2)
        || (min1..=max1).contains(&max2)
        || (min2..=max2).contains(&min1)
        || (min2..=max2).contains(&max1)
}
/// Brings down the bricks and returns the number of bricks that were brought down.
/// Optionally take HashMap of the the bricks supporting a certain brick.
fn bring_down(
    bricks: &mut [Brick],
    mut supported_by: Option<&mut HashMap<usize, Vec<usize>>>,
) -> usize {
    bricks.sort_unstable_by_key(|b| b.0 .2);
    let mut in_place = HashMap::new();
    let mut count = HashSet::new();
    for i in 0..bricks.len() {
        while bricks[i].0 .2 > 1 {
            let mut falling = true;
            let ((x1, y1, z1), (x2, y2, _)) = &bricks[i];
            for &j in in_place.get(&(z1 - 1)).unwrap_or(&vec![]) {
                let ((xx1, yy1, _), (xx2, yy2, _)) = &bricks[j];
                if intersects((*x1, *x2), (*xx1, *xx2)) && intersects((*y1, *y2), (*yy1, *yy2)) {
                    if let Some(sb) = &mut supported_by {
                        sb.entry(i).or_default().push(j);
                    }
                    falling = false;
                }
            }
            if falling {
                let ((_, _, z1), (_, _, z2)) = &mut bricks[i];
                *z1 -= 1;
                *z2 -= 1;
                count.insert(i);
            } else {
                break;
            }
        }
        let highest = bricks[i].0 .2.max(bricks[i].1 .2);
        in_place.entry(highest).or_default().push(i);
    }
    count.len()
}
fn part1(input: &str) -> usize {
    let mut bricks = parse(input);
    let mut supported_by = HashMap::new();
    bring_down(&mut bricks, Some(&mut supported_by));
    (0..bricks.len())
        .filter(|i| !supported_by.values().any(|v| v.len() == 1 && v.contains(i)))
        .count()
}
fn part2(input: &str) -> usize {
    let mut bricks = parse(input);
    let mut supported_by = HashMap::new();
    bring_down(&mut bricks, Some(&mut supported_by));
    (0..bricks.len())
        .map(|i| {
            let mut bricks = bricks.clone();
            bricks.remove(i);
            bring_down(&mut bricks, None)
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
    const INPUT: &str = "1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9";
    #[test]
    fn part1() {
        assert_eq!(super::part1(INPUT), 5);
    }
    #[test]
    fn part2() {
        assert_eq!(super::part2(INPUT), 7);
    }
}
