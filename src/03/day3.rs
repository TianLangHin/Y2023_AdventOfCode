use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

pub struct LineResult {
    pub symbols: Vec<i32>,
    pub numbers: HashMap<i32, i32>
}

impl LineResult {
    pub fn new() -> LineResult {
        return LineResult {
            symbols: Vec::new(),
            numbers: HashMap::new()
        }
    }
}

fn parse_line(line: &str, mode: bool) -> LineResult {
    let mut symbols: Vec<i32> = Vec::new();
    let mut numbers: HashMap<i32, i32> = HashMap::new();

    let mut current_number: i32 = 0;
    let mut current_index: Option<i32> = None;

    let mut index: i32 = 0;

    for character in line.chars() {
        match character {
            '.' => {
                match current_index {
                    Some(idx) => {
                        numbers.insert(idx, current_number);
                        current_number = 0;
                        current_index = None;
                    },
                    None => {}
                }
            },
            c if c.is_numeric() => {
                match current_index {
                    Some(_) => {},
                    None => {current_index = Some(index)}
                }
                current_number = current_number * 10 +
                                 c.to_digit(10).unwrap() as i32;
            },
            '*' if mode => {
                match current_index {
                    Some(index) => {
                        numbers.insert(index, current_number);
                        current_number = 0;
                        current_index = None;
                    },
                    None => {}
                }
                symbols.push(index);
            },
            _ => {
                match current_index {
                    Some(index) => {
                        numbers.insert(index, current_number);
                        current_number = 0;
                        current_index = None;
                    },
                    None => {}
                }
                if !mode {
                    symbols.push(index);
                }
            }
        }
        index += 1;
    }
    if current_number != 0 {
        numbers.insert(current_index.unwrap(), current_number);
    }
    return LineResult {
        symbols: symbols,
        numbers: numbers
    };
}

fn part1(filename: &str) -> i32 {
    let digits = |s| {
        let mut x = s;
        let mut i: i32 = 0;
        while x > 0 {
            x /= 10;
            i += 1;
        }
        return i;
    };

    let mut running_sum: i32 = 0;

    let mut before = LineResult::new();
    let mut current = LineResult::new();
    let mut after;

    let mut adjacencies: HashSet<i32> = HashSet::new();

    let lines = fs::read_to_string(filename).expect("File not found");

    for line in lines.split_whitespace().map(|x| x.trim()) {
 
        after = parse_line(&line, false);
        adjacencies.clear();

        for (index, number) in &current.numbers {

            let adjacent = |&x| {*index - 1 <= x && x <= *index + digits(*number)};

            if after.symbols.iter().any(adjacent) {
                adjacencies.insert(*index);
            }
            if current.symbols.iter().any(adjacent) {
                adjacencies.insert(*index);
            }
            if before.symbols.iter().any(adjacent) {
                adjacencies.insert(*index);
            }
        }
        for index in &adjacencies {
            running_sum += current.numbers.get(&index).unwrap();
        }
        before = current;
        current = after;
    }

    adjacencies.clear();
    for (index, number) in &current.numbers {

        let adjacent = |&x| {*index - 1 <= x && x <= *index + digits(*number)};

        if current.symbols.iter().any(adjacent) {
            adjacencies.insert(*index);
        }
        if before.symbols.iter().any(adjacent) {
            adjacencies.insert(*index);
        }
    }
    for index in &adjacencies {
        running_sum += current.numbers.get(&index).unwrap();
    }

    return running_sum;
}

fn part2(filename: &str) -> i32 {
    let digits = |s| {
        let mut x = s;
        let mut i: i32 = 0;
        while x > 0 {
            x /= 10;
            i += 1;
        }
        return i;
    };

    let mut running_sum: i32 = 0;

    let mut before = LineResult::new();
    let mut current = LineResult::new();
    let mut after;

    let mut adjacencies: Vec<i32> = Vec::new();

    let lines = fs::read_to_string(filename).expect("File not found");

    for line in lines.split_whitespace().map(|x| x.trim()) {

        after = parse_line(&line, true);

        for index in &current.symbols {

            let adjacent_value = |(&key, &value)| {
                if key - 1 <= *index && *index <= key + digits(value) {Some(value)} else {None}
            };

            adjacencies.clear();

            adjacencies.extend(before.numbers.iter().filter_map(adjacent_value));
            adjacencies.extend(current.numbers.iter().filter_map(adjacent_value));
            adjacencies.extend(after.numbers.iter().filter_map(adjacent_value));

            if adjacencies.len() == 2 {
                running_sum += adjacencies[0] * adjacencies[1];
            }
        }
        before = current;
        current = after;
    }
    for index in &current.symbols {

        let adjacent_value = |(&key, &value)| {
            if key - 1 <= *index && *index <= key + digits(value) {Some(value)} else {None}
        };

        adjacencies.clear();

        adjacencies.extend(before.numbers.iter().filter_map(adjacent_value));
        adjacencies.extend(current.numbers.iter().filter_map(adjacent_value));

        if adjacencies.len() == 2 {
            running_sum += adjacencies[0] * adjacencies[1];
        }
    }

    return running_sum;
}

fn main() {
    println!("{}", part1("day3_input.txt"));
    println!("{}", part2("day3_input.txt"));
}