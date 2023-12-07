mod hand;
use hand::*;

fn run(input: &str) -> usize {
    let mut hands = input.lines().map(Hand::from).collect::<Vec<_>>();
    hands.sort();
    dbg!(&hands);
    let mut sum = 0;
    for (i, hand) in hands.iter().enumerate() {
        sum += hand.bet * (i + 1);
    }
    sum
}
fn main() {
    let input = include_str!("../input.txt");
    println!("{}", run(input));
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
    #[test]
    fn part1_example() {
        assert_eq!(super::run(INPUT), 6440);
    }
    #[test]
    fn part2_example() {
        assert_eq!(super::run(INPUT), 5905);
    }
}
