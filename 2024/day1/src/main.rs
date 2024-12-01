use itertools::Itertools;

fn part1(input: &str) -> usize {
    let (mut a, mut b): (Vec<usize>, Vec<usize>) = input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|n| n.parse::<usize>().unwrap())
                .next_tuple::<(_, _)>()
                .unwrap()
        })
        .unzip();
    a.sort_unstable();
    b.sort_unstable();
    a.into_iter()
        .zip(b)
        .map(|(a, b)| (b as isize - a as isize).unsigned_abs())
        .sum()
}
fn part2(input: &str) -> usize {
    let (a, b): (Vec<usize>, Vec<usize>) = input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|n| n.parse::<usize>().unwrap())
                .next_tuple::<(_, _)>()
                .unwrap()
        })
        .unzip();
    a.into_iter()
        .map(|va| va * b.iter().filter(|&&vb| vb == va).count())
        .sum()
}
fn main() {
    let input = include_str!("../input.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "3   4
4   3
2   5
1   3
3   9
3   3";
    #[test]
    fn part1() {
        assert_eq!(super::part1(INPUT), 11);
    }
    #[test]
    fn part2() {
        assert_eq!(super::part2(INPUT), 31);
    }
}
