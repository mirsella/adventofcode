struct Card {
    pub id: usize,
    pub winnings: Vec<i32>,
    pub nums: Vec<i32>,
}
impl Card {
    pub fn parse(line: &str) -> Self {
        let mut s = line.split(&[':', '|']);
        let id = s
            .next()
            .unwrap()
            .split_whitespace()
            .nth(1)
            .unwrap()
            .parse()
            .unwrap();
        let winnings = s
            .next()
            .unwrap()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        let nums = s
            .next()
            .unwrap()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        Card { id, winnings, nums }
    }
    pub fn winnings_nums(&self) -> Vec<i32> {
        self.nums
            .iter()
            .filter(|n| self.winnings.contains(n))
            .copied()
            .collect()
    }
}

fn main() {
    let input = include_str!("../input.txt");
    println!("part1: {}", part1(input));
    println!("part2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    let cards: Vec<_> = input.lines().map(Card::parse).collect();
    let sum = cards
        .iter()
        .filter_map(|c| match c.winnings_nums().len() {
            0 => None,
            v => Some(v),
        })
        .map(|v| 2usize.pow(v as u32 - 1));
    sum.sum()
}

fn part2(input: &str) -> usize {
    let cards: Vec<_> = input.lines().map(Card::parse).collect();
    let mut copied_cards = vec![1; cards.len()];
    for card in cards {
        let ccard = copied_cards[card.id - 1];
        let count = card.winnings_nums().len();
        for i in 0..count {
            copied_cards[card.id + i] += ccard;
        }
    }
    copied_cards.iter().sum()
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn part1_example() {
        assert_eq!(super::part1(INPUT), 13);
    }
    #[test]
    fn part2_example() {
        assert_eq!(super::part2(INPUT), 30);
    }
}
