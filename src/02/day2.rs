use std::collections::HashMap;
use std::fs;

fn part1(filename: &str) -> u32 {
    let mut s: u32 = 0;
    let lines = fs::read_to_string(filename).expect("File not found");
    for line in lines.split('\n').filter(|x| x.len() != 0).map(|x| x.trim()) {
        let head_and_tail: Vec<_> = line.split(':').collect();
        let game_no = head_and_tail[0][5..].parse::<u32>().unwrap();
        let games = head_and_tail[1];
        let mut cubes = HashMap::from([("blue", 0), ("red", 0), ("green", 0)]);
        for game in games.split(';') {
            for category in game.split(',') {
                let game_entry: Vec<_> = category.split_whitespace().collect();
                let num = game_entry[0].parse::<u32>().unwrap();
                let colour = game_entry[1].trim();
                cubes.insert(
                    colour,
                    if *cubes.get(colour).unwrap() > num {*cubes.get(colour).unwrap()}
                    else {num}
                );
            }
        }
        if *cubes.get("red").unwrap() <= 12 &&
           *cubes.get("green").unwrap() <= 13 &&
           *cubes.get("blue").unwrap() <= 14 {
            s += game_no;
        }
    }
    return s;
}

fn part2(filename: &str) -> u32 {
    let mut s: u32 = 0;
    let lines = fs::read_to_string(filename).expect("File not found");
    for line in lines.split('\n').filter(|x| x.len() != 0).map(|x| x.trim()) {
        let head_and_tail: Vec<_> = line.split(':').collect();
        let games = head_and_tail[1];
        let mut cubes = HashMap::from([("blue", 0), ("red", 0), ("green", 0)]);
        for game in games.split(';') {
            for category in game.split(',') {
                let game_entry: Vec<_> = category.split_whitespace().collect();
                let num = game_entry[0].parse::<u32>().unwrap();
                let colour = game_entry[1].trim();
                cubes.insert(
                    colour,
                    if *cubes.get(colour).unwrap() > num {*cubes.get(colour).unwrap()}
                    else {num}
                );
            }
        }
        s += *cubes.get("blue").unwrap() * *cubes.get("red").unwrap() * *cubes.get("green").unwrap();
    }
    return s;
}

fn main() {
    println!("{}", part1("day2_input.txt"));
    println!("{}", part2("day2_input.txt"));
}