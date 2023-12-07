use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(PartialEq, PartialOrd)]
pub enum HandType {
    HighCard = 0,
    OnePair = 1,
    TwoPair = 2,
    ThreeOfAKind = 3,
    FullHouse = 4,
    FourOfAKind = 5,
    FiveOfAKind = 6,
}

pub struct Hand {
    pub hand_type: HandType,
    pub cards: Vec<u64>,
}

impl std::cmp::PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.hand_type == other.hand_type
        && self.cards.iter().zip(other.cards.iter()).all(|(&c1, &c2)| c1 == c2)
    }
}

impl std::cmp::PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return match self.hand_type.partial_cmp(&other.hand_type) {
            Some(Ordering::Equal) => {
                let mut card_comparator = self.cards.iter().zip(other.cards.iter());
                while let Some((&card1, &card2)) = card_comparator.next() {
                    match card1.partial_cmp(&card2) {
                        Some(Ordering::Equal) => {},
                        card_cmp_result => { return card_cmp_result }
                    }
                }
                return Some(Ordering::Equal)
            },
            cmp_result => { cmp_result }
        }
    }
}

fn make_type(mut card_counter: HashMap<char, u64>, part_two: bool) -> HandType {
    let freq = if part_two {
        let joker_count = match card_counter.get(&'J') {
            Some(&c) => {card_counter.remove(&'J'); c},
            None => 0
        };
        let mut f = card_counter.values().map(|&x| x).collect::<Vec<u64>>();
        f.sort_by(|r1, r2| r2.cmp(&r1));
        if f.len() != 0 { f[0] += joker_count } else { f.push(5) };
        f
    } else {
        card_counter.values().map(|&x| x).collect::<Vec<u64>>()
    };
    let count = |itr: &Vec<u64>, val: u64| itr.iter().filter(|&&x| x == val).count();
    return if count(&freq, 5) == 1 { HandType::FiveOfAKind }
           else if count(&freq, 4) == 1 { HandType::FourOfAKind }
           else if count(&freq, 3) == 1 && count(&freq, 2) == 1 { HandType::FullHouse }
           else if count(&freq, 3) == 1 && count(&freq, 1) == 2 { HandType::ThreeOfAKind }
           else if count(&freq, 2) == 2 { HandType::TwoPair }
           else if count(&freq, 2) == 1 && count(&freq, 1) == 3 { HandType::OnePair }
           else { HandType::HighCard };
}

fn process_cards(filename: &str, part_two: bool) -> u64 {
    let mut hands: Vec<(Hand, u64)> = Vec::new();
    let lines = std::fs::read_to_string(filename).expect("File not found");
    for (cards, bid) in lines
        .trim()
        .split('\n')
        .map(|line| 
            line
            .split_whitespace()
            .collect::<Vec<_>>()
        ).map(|v|
            (v[0], v[1].parse::<u64>().unwrap())
        )
    {
        let mut card_counter: HashMap<char, u64> = HashMap::new();
        let mut card_list: Vec<u64> = Vec::new();
        for card in cards.chars() {
            card_counter.insert(
                card,
                match card_counter.get(&card) {
                    Some(count) => count + 1,
                    None => 1
                }
            );
            card_list.push(match card {
                'A' => 14, 'K' => 13, 'Q' => 12,
                'J' => if part_two {1} else {11}, 'T' => 10,
                '9' => 9, '8' => 8, '7' => 7, '6' => 6,
                '5' => 5, '4' => 4, '3' => 3, '2' => 2,
                _ => panic!()
            });
        }
        hands.push((Hand { hand_type: make_type(card_counter, part_two), cards: card_list }, bid));
    }
    hands.sort_by(|r1, r2| r1.0.partial_cmp(&r2.0).unwrap());
    return hands.iter().enumerate().fold(0, |acc, (idx, item)| acc + (idx as u64 + 1) * item.1);
}

fn part1(filename: &str) -> u64 {
    process_cards(filename, false)
}

fn part2(filename: &str) -> u64 {
    process_cards(filename, true)
}

fn main() {
    println!("{}", part1("day7_input.txt"));
    println!("{}", part2("day7_input.txt"));
}
