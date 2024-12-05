use regex::Regex;

fn part1(input: &str) -> usize {
    let regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    regex
        .captures_iter(input)
        .map(|c| {
            c.extract::<2>()
                .1
                .into_iter()
                .map(|s| s.parse::<usize>().unwrap())
                .fold(0, |acc, v| if acc == 0 { v } else { acc * v })
        })
        .sum()
}
fn part2(input: &str) -> usize {
    let regex = Regex::new(r"(mul\((\d+),(\d+)\)|don't\(\)|do\(\))").unwrap();
    let mut acc = 0usize;
    let mut enabled = true;
    for capture in regex.captures_iter(input) {
        let full = capture.get(0).unwrap().as_str();
        if full == "don't()" {
            enabled = false
        } else if full == "do()" {
            enabled = true
        } else if full.starts_with("mul") {
            if enabled {
                acc += capture.get(2).unwrap().as_str().parse::<usize>().unwrap()
                    * capture.get(3).unwrap().as_str().parse::<usize>().unwrap()
            }
        } else {
            unreachable!();
        }
    }
    acc
}
fn main() {
    let input = include_str!("../input.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    #[test]
    fn part1() {
        assert_eq!(super::part1(INPUT), 161);
    }
    const INPUT2: &str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
    #[test]
    fn part2() {
        assert_eq!(super::part2(INPUT2), 48);
    }
}
