use std::collections::HashMap;

fn part(input: &str, count: u64) -> u64 {
    let mut stones = input
        .split_whitespace()
        .map(|s| (s.parse::<u64>().unwrap(), 1))
        .collect::<HashMap<_, _>>();
    for _ in 0..count {
        let mut new = HashMap::with_capacity(stones.len());
        for (k, v) in stones {
            let kstr = k.to_string();
            if k == 0 {
                *new.entry(1).or_default() += v;
            } else if kstr.len() % 2 == 0 {
                let (a, b) = kstr.split_at(kstr.len() / 2);
                *new.entry(a.parse().unwrap()).or_default() += v;
                *new.entry(b.parse().unwrap()).or_default() += v;
            } else {
                *new.entry(k * 2024).or_default() += v;
            }
        }
        stones = new;
    }
    stones.values().sum()
}

fn main() {
    let input = include_str!("../input.txt");
    println!("Part 1: {}", part(input, 25));
    println!("Part 2: {}", part(input, 75));
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "125 17";
    #[test]
    fn part1() {
        assert_eq!(super::part(INPUT, 25), 55312);
    }
}
