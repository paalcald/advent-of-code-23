
use core::panic;
use std::{collections::{BTreeMap, HashMap},fmt::Display};
const INPUT_DATA: &str = include_str!("../../data/input_day_7.txt");

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Card {
    J,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    T,
    Q,
    K,
    A,
}
impl From<char> for Card {
    fn from(value: char) -> Self {
        match value {
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
            _ => panic!("unvalid"),
        }
    }
}
impl From<Card> for char {
    fn from(value: Card) -> Self {
        match value {
            Card::Two => '2',
            Card::Three => '3',
            Card::Four => '4',
            Card::Five => '5',
            Card::Six => '6',
            Card::Seven => '7',
            Card::Eight => '8',
            Card::Nine => '9',
            Card::T => 'T',
            Card::J => 'J',
            Card::Q => 'Q',
            Card::K => 'K',
            Card::A => 'A',
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Hand {
    HighCard([Card; 5]),
    OnePair([Card; 5]),
    TwoPair([Card; 5]),
    ThreeOfAKind([Card; 5]),
    FullHouse([Card; 5]),
    FourOfAKind([Card; 5]),
    FiveOfAKind([Card; 5]),
}
impl Display for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from(self))
    }
}
impl From<&str> for Hand {
    fn from(value: &str) -> Self {
        let mut values = value.chars().map(|c| Card::from(c));
        let first = values.next().unwrap();
        let second = values.next().unwrap();
        let third = values.next().unwrap();
        let fourth = values.next().unwrap();
        let fifth = values.next().unwrap();
        let value = [first, second, third, fourth, fifth];

        let mut hash_freqs: HashMap<_, _> = value
            .iter()
            .fold(
            HashMap::new(),
            |mut f, c| {
                *(f.entry(c).or_insert(0_u8)) += 1;
                f
            });
        let js = if let Some((_, js)) = hash_freqs.remove_entry(&Card::from('J')) {
            js
        } else {
            0
        };
        let mut freqs: Vec<_> = hash_freqs
            .into_iter().collect();
        freqs.sort_by(|(_, a),(_,b)| a.cmp(b));
        let freq = if let Some((_, freq)) = freqs.pop() {
            // not all J's
            freq + js
        } else {
            5
        };
        match freq {
            5 => Self::FiveOfAKind(value),
            4 => Self::FourOfAKind(value),
            3 => {
                let (_, s_freq) = freqs.pop().unwrap();
                match s_freq {
                    2 => Self::FullHouse(value),
                    1 => Self::ThreeOfAKind(value),
                    _ => panic!("unexpected outcome"),
                }
            },
            2 => {
                let (_, s_freq) = freqs.pop().unwrap();
                match s_freq {
                    2 => Self::TwoPair(value),
                    1 => Self::OnePair(value),
                    other => panic!("unexpected outcome {}", other),
                }
            },
            1 => Self::HighCard(value),
            _ => panic!("unexpected outcome"),
        }
    }
}
impl From<&Hand> for String {
    fn from(value: &Hand) -> Self {
        match value {
            Hand::FiveOfAKind(inner) => inner,
            Hand::FourOfAKind(inner) => inner,
            Hand::FullHouse(inner) => inner,
            Hand::ThreeOfAKind(inner) => inner,
            Hand::TwoPair(inner) => inner,
            Hand::OnePair(inner) => inner,
            Hand::HighCard(inner) => inner
        }.iter().map(|card| char::from(*card)).collect()
    }
} 


fn part2(input: &str) -> u64 {
    let hands: BTreeMap<_,_> = input
        .lines()
        .map(|line| {
            let (hand, bet) = line.trim().split_once(" ").unwrap();
            (Hand::from(hand.trim()), bet.trim().parse::<u64>().unwrap())
        })
        .collect();
    hands.into_iter().enumerate().fold(0, |acc, (rank, (hand, bet))| {
        println!("{}: {} with bet {}", rank + 1, hand, bet);
        acc + (rank as u64 + 1) * bet
    })
}

fn main() {
    println!("{}", part2(INPUT_DATA)); 
}