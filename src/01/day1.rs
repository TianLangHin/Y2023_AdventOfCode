use std::fs;
use std::collections::HashMap;
use regex::Regex;

fn part1(filename: &str) -> u32 {
    let mut s: u32 = 0;
    let mut start: u32;
    let mut end: u32;
    let lines = fs::read_to_string(filename).expect("File not found");
    for line in lines.split('\n') {
        start = 10;
        end = 10;
        for character in line.chars() {
            match character.to_digit(10) {
                Some(digit) => {
                    if start == 10 {
                        start = digit;
                    }
                    end = digit;
                },
                None => {}
            }
        }
        if start == 10 || end == 10 {
            panic!("A line of puzzle input violates its premise");
        }
        s += start * 10 + end;
    }

    return s;
}

fn part2(filename: &str) -> u32 {
    let pattern = Regex::new(r"one|two|three|four|five|six|seven|eight|nine|\d").unwrap();
    let mut s: u32 = 0;
    let mut start: u32;
    let mut end: u32;
    let conv: HashMap<&str, u32> = HashMap::from([
        ("one", 1), ("1", 1),
        ("two", 2), ("2", 2),
        ("three", 3), ("3", 3),
        ("four", 4), ("4", 4),
        ("five", 5), ("5", 5),
        ("six", 6), ("6", 6),
        ("seven", 7), ("7", 7),
        ("eight", 8), ("8", 8),
        ("nine", 9), ("9", 9),
    ]);
    let lines = fs::read_to_string(filename).expect("File not found");
    for line in lines.split('\n').map(|l| l.trim()) {
        start = 10;
        end = 10;
        for i in 0..line.len() {
            let pattern_match = pattern.find_at(line, i);
            match pattern_match {
                Some(word) => {
                    match conv.get(word.as_str()) {
                        Some(&number) => {
                            if start == 10 {
                                start = number;
                            }
                            end = number;
                        },
                        None => {}
                    }
                },
                None => {}
            }
        }
        if start == 10 || end == 10 {
            panic!("A line of puzzle input violates its premise");
        }
        s += start * 10 + end;
    }
    return s;
}

fn main() {
    println!("{}", part1("day1_input.txt"));
    println!("{}", part2("day1_input.txt"));
}