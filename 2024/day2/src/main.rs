use itertools::Itertools;

fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|l| {
            let levels = l
                .split_whitespace()
                .map(|n| n.parse::<usize>().unwrap())
                .collect_vec();
            let ordering = levels[0].cmp(&levels[1]);
            levels
                .into_iter()
                .tuple_windows()
                .all(|(a, b)| a.cmp(&b) == ordering && a.abs_diff(b) >= 1 && a.abs_diff(b) <= 3)
        })
        .filter(|v| *v)
        .count()
}
fn part2(input: &str) -> usize {
    input
        .lines()
        .map(|l| {
            let levels = l
                .split_whitespace()
                .map(|n| n.parse::<usize>().unwrap())
                .collect_vec();
            (-1..levels.len() as isize).any(|i| {
                let new_levels = if i < 0 {
                    levels.clone()
                } else {
                    let mut l = levels.clone();
                    l.remove(i as usize);
                    l
                };
                let ordering = new_levels[0].cmp(&new_levels[1]);
                new_levels
                    .into_iter()
                    .tuple_windows()
                    .all(|(a, b)| a.cmp(&b) == ordering && a.abs_diff(b) >= 1 && a.abs_diff(b) <= 3)
            })
        })
        .filter(|v| *v)
        .count()
}
fn main() {
    let input = include_str!("../input.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
    #[test]
    fn part1() {
        assert_eq!(super::part1(INPUT), 2);
    }
    #[test]
    fn part2() {
        assert_eq!(super::part2(INPUT), 4);
    }
}
