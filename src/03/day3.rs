use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

pub struct LineResult {
    pub symbols: Vec<u32>,
    pub numbers: HashMap<u32, u32>
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
    let mut symbols: Vec<u32> = Vec::new();
    let mut numbers: HashMap<u32, u32> = HashMap::new();

    let mut current_number: u32 = 0;
    let mut current_index: Option<u32> = None;

    let mut index: u32 = 0;
    for character in line.chars() {
        match character {
            '.' => {
                if current_number != 0 {
                    numbers.insert(
                        current_index.expect("problem in dot"),
                        current_number
                    );
                    current_number = 0;
                    current_index = None;
                }
            },
            c if c.is_numeric() => {
                if current_number == 0 {
                    current_index = Some(index);
                }
                current_number = current_number * 10 +
                                 c.to_digit(10).expect("shouldn't happen");
            },
            '*' if mode => {
                if current_number != 0 {
                    numbers.insert(
                        current_index.expect("problem in asterisk"),
                        current_number
                    );
                    current_number = 0;
                    current_index = None;
                }
                symbols.push(index);
            },
            _ => {
                if current_number != 0 {
                    numbers.insert(
                        current_index.expect("problem in wildcard"),
                        current_number
                    );
                    current_number = 0;
                    current_index = None;
                }
                if !mode {
                    symbols.push(index);
                }
            }
        }
        index += 1;
    }
    if current_number != 0 {
        numbers.insert(current_index.expect("Problem at the end"), current_number);
    }
    return LineResult {
        symbols: symbols,
        numbers: numbers
    };
}

fn part1(filename: &str) -> u32 {
    let digits = |s: u32| {
        let mut x = s;
        let mut i: u32 = 0;
        while x > 0 {
            x /= 10;
            i += 1;
        }
        return i;
    };

    let mut running_sum: u32 = 0;

    let mut before = LineResult::new();
    let mut current = LineResult::new();
    let mut after;

    let mut adjacencies: HashSet<u32> = HashSet::new();

    let lines = fs::read_to_string(filename).expect("File not found");

    for line in lines.split_whitespace().map(|x| x.trim()) {
 
        after = parse_line(&line, false);
        adjacencies.clear();

        for (index, number) in &current.numbers {
            if after.symbols.iter().any(
                |&x| {
                    if *index == 0 {
                        return x <= *index + digits(*number)
                    }
                    return *index - 1 <= x && x <= *index + digits(*number)
                }
            ) {
                adjacencies.insert(*index);
            }

            if current.symbols.iter().any(
                |&x| {
                    if *index == 0 {
                        return x <= *index + digits(*number)
                    }
                    return *index - 1 <= x && x <= *index + digits(*number)
                }
            ) {
                adjacencies.insert(*index);
            }

            if before.symbols.iter().any(
                |&x| {
                    if *index == 0 {
                        return x <= *index + digits(*number)
                    }
                    return *index - 1 <= x && x <= *index + digits(*number)
                }
            ) {
                adjacencies.insert(*index);
            }
        }
        for index in &adjacencies {
            running_sum += current.numbers.get(&index).expect("what the hell");
        }
        before = current;
        current = after;
    }

    adjacencies.clear();
    for (index, number) in &current.numbers {
        if current.symbols.iter().any(
            |&x| {
                if *index == 0 {
                    return x <= *index + digits(*number)
                }
                return *index - 1 <= x && x <= *index + digits(*number)
            }
        ) {
            adjacencies.insert(*index);
        }

        if before.symbols.iter().any(
            |&x| {
                if *index == 0 {
                    return x <= *index + digits(*number)
                }
                return *index - 1 <= x && x <= *index + digits(*number)
            }
        ) {
            adjacencies.insert(*index);
        }
    }
    for index in &adjacencies {
        running_sum += current.numbers.get(&index).expect("what the hell");
    }

    return running_sum;
}

fn part2(filename: &str) -> u32 {
    let digits = |s: u32| {
        let mut x = s;
        let mut i: u32 = 0;
        while x > 0 {
            x /= 10;
            i += 1;
        }
        return i;
    };

    let mut running_sum: u32 = 0;

    let mut before = LineResult::new();
    let mut current = LineResult::new();
    let mut after;

    let mut adjacencies: Vec<u32> = Vec::new();

    let lines = fs::read_to_string(filename).expect("File not found");

    for line in lines.split_whitespace().map(|x| x.trim()) {
        after = parse_line(&line, true);
        for index in &current.symbols {

            adjacencies.clear();

            adjacencies.extend(
                before.numbers.iter().filter_map(
                    |(&key, &value)| {
                        if key == 0 {
                            if *index <= key + digits(value) {
                                return Some(value)
                            }
                            return None
                        }
                        if key - 1 <= *index && *index <= key + digits(value) {
                            return Some(value)
                        }
                        return None
                    }
                )
            );

            adjacencies.extend(
                current.numbers.iter().filter_map(
                    |(&key, &value)| {
                        if key == 0 {
                            if *index <= key + digits(value) {
                                return Some(value)
                            }
                            return None
                        }
                        if key - 1 <= *index && *index <= key + digits(value) {
                            return Some(value)
                        }
                        return None
                    }
                )
            );

            adjacencies.extend(
                after.numbers.iter().filter_map(
                    |(&key, &value)| {
                        if key == 0 {
                            if *index <= key + digits(value) {
                                return Some(value)
                            }
                            return None
                        }
                        if key - 1 <= *index && *index <= key + digits(value) {
                            return Some(value)
                        }
                        return None
                    }
                )
            );

            if adjacencies.len() == 2 {
                running_sum += adjacencies[0] * adjacencies[1];
            }
        }
        before = current;
        current = after;
    }
    for index in &current.symbols {

        adjacencies.clear();

        adjacencies.extend(
            before.numbers.iter().filter_map(
                |(&key, &value)| {
                    if key == 0 {
                        if *index <= key + digits(value) {
                            return Some(value)
                        }
                        return None
                    }
                    if key - 1 <= *index && *index <= key + digits(value) {
                        return Some(value)
                    }
                    return None
                }
            )
        );

        adjacencies.extend(
            current.numbers.iter().filter_map(
                |(&key, &value)| {
                    if key == 0 {
                        if *index <= key + digits(value) {
                            return Some(value)
                        }
                        return None
                    }
                    if key - 1 <= *index && *index <= key + digits(value) {
                        return Some(value)
                    }
                    return None
                }
            )
        );

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