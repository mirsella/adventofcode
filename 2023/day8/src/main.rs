use std::collections::HashMap;

use num::integer::lcm;

enum Direction {
    Left,
    Right,
}

impl From<char> for Direction {
    fn from(value: char) -> Self {
        match value {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => panic!("Invalid direction"),
        }
    }
}
struct Map<'a> {
    pub network: HashMap<&'a str, (&'a str, &'a str)>,
    pub directions: Vec<Direction>,
}
impl<'a> Map<'a> {
    pub fn parse(input: &'a str) -> Self {
        let mut lines = input.lines();
        let directions = lines
            .next()
            .unwrap()
            .chars()
            .map(Direction::from)
            .collect::<Vec<_>>();
        // let network = HashMap::new();
        let network = lines
            .skip(1)
            .map(|s| {
                let parts = s.split_once('=').unwrap();
                let map = parts.1.split_once(',').unwrap();
                (
                    parts.0.trim(),
                    (
                        map.0.trim_matches([' ', '('].as_slice()),
                        map.1.trim_matches([' ', ')'].as_slice()),
                    ),
                )
            })
            .collect();
        Map {
            network,
            directions,
        }
    }
}
fn part1(input: &str) -> usize {
    let map = Map::parse(input);
    let mut current: &str = "AAA";
    let mut directions = map.directions.iter().cycle();
    let mut count: usize = 0;
    while current != "ZZZ" {
        let node = map.network.get(current).unwrap();
        count += 1;
        match directions.next().unwrap() {
            Direction::Left => current = node.0,
            Direction::Right => current = node.1,
        }
    }
    count
}

/// from the subreddit, we can use the LCM algorithm (couldn't figure it out myself this time)
fn part2(input: &str) -> usize {
    let map = Map::parse(input);
    let currents = map
        .network
        .keys()
        .filter(|k| k.ends_with('A'))
        .copied()
        .collect::<Vec<_>>();
    let mut lowest: usize = 0;
    for mut current in currents {
        let mut directions = map.directions.iter().cycle();
        let mut count = 0;
        while !current.ends_with('Z') {
            let node = map.network.get(current).unwrap();
            count += 1;
            match directions.next().unwrap() {
                Direction::Left => current = node.0,
                Direction::Right => current = node.1,
            }
        }
        match lowest {
            0 => lowest = count,
            _ => lowest = lcm(lowest, count),
        }
    }
    lowest
}
fn main() {
    let input = include_str!("../input.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT1: &str = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";
    const INPUT2: &str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
    const INPUT3: &str = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

    #[test]
    fn part1_1() {
        assert_eq!(part1(INPUT1), 2);
    }
    #[test]
    fn part1_2() {
        assert_eq!(part1(INPUT2), 6);
    }
    #[test]
    fn part2_1() {
        assert_eq!(part2(INPUT3), 6);
    }
}
