use std::{array::from_fn, cmp::Ordering, collections::HashMap};

#[derive(Debug, PartialEq, PartialOrd, Hash, Eq, Copy, Clone)]
pub enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    T,
    J,
    Q,
    K,
    A,
}
impl From<char> for Card {
    /// should use TryFrom for a failable conversion
    fn from(c: char) -> Self {
        match c {
            '2' => Self::Two,
            '3' => Self::Three,
            '4' => Self::Four,
            '5' => Self::Five,
            '6' => Self::Six,
            '7' => Self::Seven,
            '8' => Self::Eight,
            '9' => Self::Nine,
            'T' => Self::T,
            'J' => Self::J,
            'Q' => Self::Q,
            'K' => Self::K,
            'A' => Self::A,
            _ => panic!("Invalid card"),
        }
    }
}
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Suit {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}
#[derive(Debug)]
pub struct Hand {
    pub cards: [Card; 5],
    pub suit: Suit,
    pub bet: usize,
}
impl From<&str> for Hand {
    fn from(s: &str) -> Self {
        let mut s = s.split_whitespace();
        let mut c = s.next().unwrap().chars();
        let cards: [Card; 5] = from_fn(|_| Card::from(c.next().unwrap()));
        let bet = s.next().unwrap().parse().unwrap();
        let suit = find_suit(&cards);
        Self { cards, bet, suit }
    }
}

impl Eq for Hand {}
impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards && self.bet == other.bet
    }
}
impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.suit.cmp(&other.suit) {
            Ordering::Equal => {
                for (a, b) in self.cards.iter().zip(other.cards.iter()) {
                    return match a.partial_cmp(b).unwrap() {
                        Ordering::Equal => continue,
                        o => o,
                    };
                }
                Ordering::Equal
            }
            o => o,
        }
    }
}

/// From strongest to weakest, they are:
/// Five of a kind, where all five cards have the same label: AAAAA
/// Four of a kind, where four cards have the same label and one card has a different label: AA8AA
/// Full house, where three cards have the same label, and the remaining two cards share a different label: 23332
/// Three of a kind, where three cards have the same label, and the remaining two cards are each different from any other card in the hand: TTT98
/// Two pair, where two cards share one label, two other cards share a second label, and the remaining card has a third label: 23432
/// One pair, where two cards share one label, and the other three cards have a different label from the pair and each other: A23A4
/// High card, where all cards' labels are distinct: 23456
fn find_suit(cards: &[Card; 5]) -> Suit {
    let mut h = HashMap::new();
    for card in cards.iter() {
        *h.entry(card).or_insert(0) += 1;
    }
    if h.values().any(|&v| v == 5) {
        Suit::FiveOfAKind
    } else if h.values().any(|&v| v == 4) {
        Suit::FourOfAKind
    } else if h.values().any(|&v| v == 3) && h.values().any(|&v| v == 2) {
        Suit::FullHouse
    } else if h.values().any(|&v| v == 3) {
        Suit::ThreeOfAKind
    } else if h.values().filter(|&&v| v == 2).count() == 2 {
        Suit::TwoPair
    } else if h.values().any(|&v| v == 2) {
        Suit::OnePair
    } else {
        Suit::HighCard
    }
}
