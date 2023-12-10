use std::{cmp::Ordering, str::FromStr};

pub fn puzzle1(input: &str) {
    let mut hands = Vec::new();
    for line in input.lines() {
        hands.push(line.parse::<Hand>().unwrap());
    }
    hands.sort();
    hands.reverse();

    let mut acum = 0;
    for (i, hand) in hands.iter().enumerate() {
        let points = (i + 1) as u32 * hand.bid;
        acum += points;
        println!("{:?} - {} - {} - {}", hand.kind, hand.text, points, acum);
    }

    let points = hands
        .iter()
        .enumerate()
        .map(|(i, hand)| (i + 1) as u64 * hand.bid as u64)
        .fold(0, |acum, v| acum + v);

    println!("{points}");
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
enum Card {
    A,
    K,
    Q,
    J,
    T,
    N9,
    N8,
    N7,
    N6,
    N5,
    N4,
    N3,
    N2,
}

#[derive(Eq)]
struct Hand {
    text: String,
    cards: [Card; 5],
    bid: u32,
    kind: Type,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
enum Type {
    FiveOf,
    FourOf,
    FullHouse,
    ThreeOf,
    TwoPair,
    Pair,
    High,
}

impl Card {
    fn from_char(input: char) -> Result<Card, ()> {
        match input {
            '2' => Ok(Card::N2),
            '3' => Ok(Card::N3),
            '4' => Ok(Card::N4),
            '5' => Ok(Card::N5),
            '6' => Ok(Card::N6),
            '7' => Ok(Card::N7),
            '8' => Ok(Card::N8),
            '9' => Ok(Card::N9),
            'T' => Ok(Card::T),
            'J' => Ok(Card::J),
            'Q' => Ok(Card::Q),
            'K' => Ok(Card::K),
            'A' => Ok(Card::A),
            _ => Err(()),
        }
    }
}

impl FromStr for Hand {
    type Err = ();
    fn from_str(input: &str) -> Result<Hand, ()> {
        let pair: Vec<_> = input.split_whitespace().collect();
        let bid = pair[1].parse().unwrap();
        let mut cards = [Card::A; 5];
        for (i, c) in pair[0].chars().enumerate() {
            cards[i] = Card::from_char(c).unwrap();
        }

        let mut amount = [0; 13];
        for card in cards {
            amount[card as usize] += 1;
        }
        amount.sort_by(|a, b| b.cmp(a));

        let kind = match amount[0] {
            5 => Type::FiveOf,
            4 => Type::FourOf,
            3 => match amount[1] {
                2 => Type::FullHouse,
                _ => Type::ThreeOf,
            },
            2 => match amount[1] {
                2 => Type::TwoPair,
                _ => Type::Pair,
            },
            _ => Type::High,
        };
        Ok(Hand {
            text: input.to_string(),
            cards,
            bid,
            kind,
        })
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let ord = self.kind.cmp(&other.kind);
        if ord != Ordering::Equal {
            return ord;
        }
        for i in 0..5 {
            let ord = self.cards[i].cmp(&other.cards[i]);
            if ord != Ordering::Equal {
                return ord;
            }
        }
        self.bid.cmp(&other.bid)
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards && self.bid == other.bid
    }
}
