use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

fn part1(filename: &str) -> u32 {
    let lines = fs::read_to_string(filename).expect("File not found");
    let mut s: u32 = 0;
    for line in lines.trim().split('\n') {
        let data = line.split(':').collect::<Vec<_>>()[1];
        let split_card: Vec<_> = data.split('|').collect();
        let wins = split_card[0]
                   .split_whitespace()
                   .map(|x| x.parse::<u32>().unwrap())
                   .collect::<HashSet<u32>>();
        let have = split_card[1]
                   .split_whitespace()
                   .map(|x| x.parse::<u32>().unwrap())
                   .collect::<HashSet<u32>>();
        let winning_numbers = wins.intersection(&have).count();
        if winning_numbers > 0 {
            s += 1 << (winning_numbers - 1);
        }
    }
    return s;
}

fn part2(filename: &str) -> u32 {
    let lines = fs::read_to_string(filename).expect("File not found");
    let mut s: u32 = 0;
    let mut bonuses: HashMap<u32, u32> = HashMap::new();
    for line in lines.trim().split('\n') {
        let split_line = line.split(':').collect::<Vec<_>>();
        let card_no = split_line[0]
                      .split_whitespace()
                      .collect::<Vec<_>>()[1]
                      .parse::<u32>()
                      .unwrap();
        let split_card: Vec<_> = split_line[1].split('|').collect();
        let wins = split_card[0]
                   .split_whitespace()
                   .map(|x| x.parse::<u32>().unwrap())
                   .collect::<HashSet<u32>>();
        let have = split_card[1]
                   .split_whitespace()
                   .map(|x| x.parse::<u32>().unwrap())
                   .collect::<HashSet<u32>>();
        let winning_numbers = wins.intersection(&have).count();
        let mut current_entry: u32;
        let mut future_entry: u32;
        for i in 1..winning_numbers+1 {
            match bonuses.get(&card_no) {
                Some(&bonus) => {
                    current_entry = bonus;
                },
                None => {
                    current_entry = 0;
                }
            }
            match bonuses.get(&(card_no + i as u32)) {
                Some(&bonus) => {
                    future_entry = bonus;
                },
                None => {
                    future_entry = 0;
                }
            }
            bonuses.insert(card_no + i as u32, future_entry + current_entry + 1);
        }
        match bonuses.get(&card_no) {
            Some(&bonus) => {
                s += bonus + 1;
            },
            None => {
                s += 1;
            }
        }
    }
    return s;
}

fn main() {
    println!("{}", part1("day4_input.txt"));
    println!("{}", part2("day4_input.txt"));
}
