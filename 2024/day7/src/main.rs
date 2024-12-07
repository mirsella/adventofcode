use itertools::Itertools;

fn part1(input: &str) -> usize {
    input
        .lines()
        .filter_map(|l| {
            let mut s = l
                .split_whitespace()
                .map(|s| s.trim_end_matches(':').parse::<usize>().unwrap());
            let total = s.next().unwrap();
            let values = s.collect_vec();
            fn try_operator(total: usize, values: &[usize], acc: usize) -> bool {
                let add = values[0] + acc;
                let mul = values[0] * acc;
                match values.len() {
                    1 => add == total || mul == total,
                    _ => {
                        try_operator(total, &values[1..], add)
                            || try_operator(total, &values[1..], mul)
                    }
                }
            }
            try_operator(total, &values[1..], values[0]).then_some(total)
        })
        .sum()
}

fn part2(input: &str) -> usize {
    input
        .lines()
        .filter_map(|l| {
            let mut s = l
                .split_whitespace()
                .map(|s| s.trim_end_matches(':').parse::<usize>().unwrap());
            let total = s.next().unwrap();
            let values = s.collect_vec();
            fn try_operator(total: usize, values: &[usize], acc: usize) -> bool {
                let add = values[0] + acc;
                let mul = values[0] * acc;
                let concat = format!("{}{}", acc, values[0]).parse::<usize>().unwrap();
                match values.len() {
                    1 => add == total || mul == total || concat == total,
                    _ => {
                        try_operator(total, &values[1..], add)
                            || try_operator(total, &values[1..], mul)
                            || try_operator(total, &values[1..], concat)
                    }
                }
            }
            try_operator(total, &values[1..], values[0]).then_some(total)
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
    const INPUT: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
    #[test]
    fn part1() {
        assert_eq!(super::part1(INPUT), 3749);
    }
    #[test]
    fn part2() {
        assert_eq!(super::part2(INPUT), 11387);
    }
}
