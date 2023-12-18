#[derive(Clone, Copy)]
pub enum Direction {
    Right,
    Down,
    Left,
    Up
}

fn calculate_fill(instructions: &Vec<(Direction, i64)>) -> i64 {
    let mut x: i64 = 0;
    let mut y: i64 = 0;
    let mut min_x: i64 = 0;
    let mut min_y: i64 = 0;
    for &(direction, length) in instructions {
        match direction {
            Direction::Right => x += length,
            Direction::Down  => y += length,
            Direction::Left  => {
                x -= length;
                if x < min_x { min_x = x; }
            },
            Direction::Up    => {
                y -= length;
                if y < min_y { min_y = y; }
            }
        }
    }
    x = -min_x;
    y = -min_y;
    let mut total_area: i64 = 0;
    let mut perimeter: i64 = 0;
    for &(direction, length) in instructions {
        match direction {
            Direction::Right => {
                x += length;
                perimeter += length;
            },
            Direction::Left  => {
                x -= length;
            },
            Direction::Down  => {
                y += length;
                perimeter += length;
                total_area += x * length;
            },
            Direction::Up    => {
                y -= length;
                total_area -= x * length;
            }
        }
    }
    return perimeter + total_area + 1;
}

fn part1(filename: &str) -> i64 {
    let lines = std::fs::read_to_string(filename).unwrap();
    let instructions = lines
        .trim()
        .split('\n')
        .map(
            |line|
            line
            .trim()
            .split_whitespace()
            .collect::<Vec<_>>()
        )
        .map(
            |data|
            (
                match data[0] {
                    "R" => Direction::Right,
                    "D" => Direction::Down,
                    "L" => Direction::Left,
                    "U" => Direction::Up,
                    _   => panic!()
                },
                data[1].parse::<i64>().unwrap()
            )
        )
        .collect::<Vec<_>>();
    return calculate_fill(&instructions);
}

fn hex_string(string: &str) -> Option<i64> {
    let mut number: i64 = 0;
    for character in string.chars() {
        number = number << 4 |
            match character {
                '0' => 0,  '1' => 1,  '2' => 2,  '3' => 3,
                '4' => 4,  '5' => 5,  '6' => 6,  '7' => 7,
                '8' => 8,  '9' => 9,  'a' => 10, 'b' => 11,
                'c' => 12, 'd' => 13, 'e' => 14, 'f' => 15,
                _ => return None
            };
    }
    return Some(number);
}

fn part2(filename: &str) -> i64 {
    let lines = std::fs::read_to_string(filename).unwrap();
    let instructions = lines
        .trim()
        .split('\n')
        .map(
            |line|
            line
            .trim()
            .split_whitespace()
            .collect::<Vec<_>>()[2]
        )
        .map(
            |data|
            (
                match &data[7..8] {
                    "0" => Direction::Right,
                    "1" => Direction::Down,
                    "2" => Direction::Left,
                    "3" => Direction::Up,
                    _   => panic!()
                },
                hex_string(&data[2..7]).unwrap()
            )
        )
        .collect::<Vec<_>>();
    return calculate_fill(&instructions);
}

fn main() {
    println!("{}", part1("day18_input.txt"));
    println!("{}", part2("day18_input.txt"));
}