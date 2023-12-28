use anyhow::Result;
use core::fmt;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt::Debug;
use std::fs::File;
use std::io::{BufRead, BufReader};

// A card is a set of cards of the same value, e.g. AA or 666
#[derive(PartialEq, Eq, Clone, Copy)]
struct CardSet {
    num: u32,
    value: usize,
}

impl PartialOrd for CardSet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // println!("comparing {:?} to {:?}", self, other);
        let num_order = self.num.partial_cmp(&other.num);
        if num_order == Some(Ordering::Equal) {
            self.value.partial_cmp(&other.value)
        } else {
            num_order
        }
    }
}

impl fmt::Display for CardSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let char = value_to_char(self.value);
        let val = char.to_string().repeat(self.num as usize);
        write!(f, "{}", val)
    }
}

// A Hand is an ordered vector of CardSet's.
#[derive(PartialEq, Eq)]
struct Hand {
    hand_str: String,
    cards: Vec<CardSet>,
}

#[derive(PartialEq, Eq, Ord, PartialOrd)]
enum HandType {
    Five,
    Four,
    FullHouse,
    Three,
    TwoPair,
    Pair,
    High,
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let type_order = other.hand_type().partial_cmp(&self.hand_type());
        if type_order == Some(Ordering::Equal) {
            // self.cards.partial_cmp(&other.cards)
            for (c1, c2) in self.hand_str.chars().zip(other.hand_str.chars()) {
                let v1 = char_to_value(c1);
                let v2 = char_to_value(c2);
                let order = v1.partial_cmp(&v2);
                if order != Some(Ordering::Equal) {
                    return order;
                }
            }
            Some(Ordering::Equal)
        } else {
            type_order
        }
    }
}

impl fmt::Display for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut cards_str = String::new();
        cards_str.push_str(&self.hand_str);
        cards_str.push_str(" | ");
        for card in &self.cards {
            cards_str.push_str(&card.to_string());
            cards_str.push(' ');
        }
        write!(f, "{}", cards_str)
    }
}

impl Hand {
    pub fn new(hand_str: &str) -> Self {
        let mut hand_count: HashMap<usize, u32> =
            hand_str
                .chars()
                .map(char_to_value)
                .fold(HashMap::new(), |mut acc, x| {
                    *acc.entry(x).or_insert(0) += 1;
                    acc
                });
        let num_jokers = hand_count.remove(&1).unwrap_or(0);
        let mut cards: Vec<CardSet> = hand_count
            .iter()
            .map(|(k, v)| CardSet { num: *v, value: *k })
            .collect();
        cards.sort_by(|a, b| b.partial_cmp(a).unwrap());
        if !cards.is_empty() {
            cards[0].num += num_jokers;
        }

        Self {
            hand_str: hand_str.to_string(),
            cards,
        }
    }
    pub fn hand_type(&self) -> HandType {
        let num_sets = self.cards.len();
        let best_num = match self.cards.get(0) {
            Some(first_card) => first_card.num,
            None => 5,
        };

        // let best_num = self.cards.get(0).num;
        match (num_sets, best_num) {
            (0, 5) => HandType::Five,
            (1, 5) => HandType::Five,
            (2, 4) => HandType::Four,
            (3, 3) => HandType::Three,
            (4, 2) => HandType::Pair,
            (5, 1) => HandType::High,
            (3, 2) => HandType::TwoPair,
            (2, 3) => HandType::FullHouse,
            _ => panic!("Unknown hand type {}. num_sets={}, best_num={}", self, num_sets, best_num),
        }
    }
}

fn char_to_value(c: char) -> usize {
    // println!("char: {}", c);
    match c {
        'T' => 10,
        'J' => 1,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => c.to_digit(10).unwrap() as usize,
    }
}

fn value_to_char(v: usize) -> char {
    match v {
        10 => 'T',
        1 => 'J',
        12 => 'Q',
        13 => 'K',
        14 => 'A',
        _ => std::char::from_digit(v as u32, 10).unwrap(),
    }
}

struct Bet {
    hand: Hand,
    bet: u32,
}

impl Bet {
    pub fn from_line(line: &str) -> Self {
        let mut split = line.split_whitespace();
        let hand = Hand::new(split.next().unwrap());
        let bet = split.next().unwrap().parse::<u32>().unwrap();
        Self { hand, bet }
    }
}

fn main() -> Result<()> {
    let file = File::open("day7/src/input.txt")?;
    let reader = BufReader::new(file);

    let mut bets: Vec<Bet> = reader
        .lines()
        .map(|l| Bet::from_line(&l.unwrap()))
        .collect();
    bets.sort_by(|bet1, bet2| bet1.hand.partial_cmp(&bet2.hand).unwrap());
    let mut sum = 0;
    for (rank, bet) in bets.iter().enumerate() {
        println!("{}\t{} * {}", bet.hand, bet.bet, rank + 1);
        sum += ((rank as u32) + 1) * bet.bet;
    }

    println!("sum: {}", sum);

    Ok(())
}
