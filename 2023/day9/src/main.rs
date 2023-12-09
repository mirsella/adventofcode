struct Readings {
    pub data: Vec<i32>,
}
impl Readings {
    pub fn parse(line: &str) -> Self {
        Self {
            data: line
                .split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect(),
        }
    }
    pub fn predict_after(&self) -> i32 {
        fn predict_rec(n: &[i32]) -> i32 {
            let reduced = n.windows(2).map(|w| w[1] - w[0]).collect::<Vec<_>>();
            if reduced.iter().all(|&n| n == 0) {
                return *n.last().unwrap();
            }
            let rec = predict_rec(&reduced);
            n.last().unwrap() + rec
        }
        predict_rec(&self.data)
    }
    pub fn predict_before(&self) -> i32 {
        fn predict_rec(n: &[i32]) -> i32 {
            let reduced = n.windows(2).map(|w| w[1] - w[0]).collect::<Vec<_>>();
            if reduced.iter().all(|&n| n == 0) {
                return *n.first().unwrap();
            }
            let rec = predict_rec(&reduced);
            n.first().unwrap() - rec
        }
        predict_rec(&self.data)
    }
}
fn part1(input: &str) -> i32 {
    input
        .lines()
        .map(Readings::parse)
        .map(|r| r.predict_after())
        .sum()
}
fn part2(input: &str) -> i32 {
    input
        .lines()
        .map(Readings::parse)
        .map(|r| r.predict_before())
        .sum()
}
fn main() {
    let input = include_str!("../input.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "0 3 6 9 12 15
    1 3 6 10 15 21
    10 13 16 21 30 45";
    #[test]
    fn part1() {
        assert_eq!(super::part1(INPUT), 114);
    }
    #[test]
    fn part2() {
        assert_eq!(super::part2(INPUT), 2);
    }
    #[test]
    fn part2_3() {
        assert_eq!(super::part2("10 13 16 21 30 45"), 5);
    }
}
