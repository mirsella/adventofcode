mod hand;
mod handjoker;

fn part1(input: &str) -> usize {
    let mut hands = input.lines().map(hand::Hand::from).collect::<Vec<_>>();
    hands.sort();
    // dbg!(&hands);
    let mut sum = 0;
    for (i, hand) in hands.iter().enumerate() {
        sum += hand.bet * (i + 1);
    }
    sum
}
fn part2(input: &str) -> usize {
    let mut hands = input.lines().map(handjoker::Hand::from).collect::<Vec<_>>();
    hands.sort();
    // dbg!(&hands);
    let mut sum = 0;
    for (i, hand) in hands.iter().enumerate() {
        sum += hand.bet * (i + 1);
    }
    sum
}
fn main() {
    let input = include_str!("../input.txt");
    println!("part1: {}", part1(input));
    println!("part2: {}", part2(input));
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    // https://www.reddit.com/r/adventofcode/comments/18cr4xr/2023_day_7_better_example_input_not_a_spoiler/
    const INPUT2: &str = "2345A 1
Q2KJJ 13
Q2Q2Q 19
T3T3J 17
T3Q33 11
2345J 3
J345A 2
32T3K 5
T55J5 29
KK677 7
KTJJT 34
QQQJA 31
JJJJJ 37
JAAAA 43
AAAAJ 59
AAAAA 61
2AAAA 23
2JJJJ 53
JJJJ2 41";
    #[test]
    fn part1_example() {
        assert_eq!(super::part1(INPUT), 6440);
    }
    #[test]
    fn part2_example() {
        assert_eq!(super::part2(INPUT), 5905);
    }
    #[test]
    fn part2_reddit_example() {
        assert_eq!(super::part2(INPUT2), 6839);
    }
}
