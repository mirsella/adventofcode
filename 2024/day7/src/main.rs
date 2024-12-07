use itertools::Itertools;

fn part1(input: &str) -> usize {
    let lines = input.lines().map(|l| {
        let mut s = l
            .split_whitespace()
            .map(|s| s.trim_end_matches(':').parse::<usize>().unwrap());
        (s.next().unwrap(), s.collect_vec())
    });
    fn try_operator(total: usize, values: &[usize], acc: usize) -> bool {
        if (acc + values[0] == total || acc * values[0] == total) && values.len() == 1 {
            return true;
        }
        if values.len() > 1 {
            if try_operator(total, &values[1..], acc + values[0]) {
                return true;
            }
            if try_operator(total, &values[1..], acc * values[0]) {
                return true;
            }
        }
        false
    }
    lines
        .filter_map(|(total, values)| try_operator(total, &values[1..], values[0]).then_some(total))
        .sum()
}

fn part2(input: &str) -> usize {
    let lines = input.lines().map(|l| {
        let mut s = l
            .split_whitespace()
            .map(|s| s.trim_end_matches(':').parse::<usize>().unwrap());
        (s.next().unwrap(), s.collect_vec())
    });
    fn try_operator(total: usize, values: &[usize], acc: usize) -> bool {
        let add = values[0] + acc;
        let mul = values[0] * acc;
        let concat = format!("{}{}", acc, values[0]).parse::<usize>().unwrap();
        if (add == total || mul == total || concat == total) && values.len() == 1 {
            return true;
        }
        if values.len() > 1 {
            if try_operator(total, &values[1..], add) {
                return true;
            }
            if try_operator(total, &values[1..], mul) {
                return true;
            }
            if try_operator(total, &values[1..], concat) {
                return true;
            }
        }
        false
    }
    lines
        .filter_map(|(total, values)| try_operator(total, &values[1..], values[0]).then_some(total))
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
