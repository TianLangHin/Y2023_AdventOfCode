use std::collections::BTreeSet;

pub struct Galaxy {
    pub x: i64,
    pub y: i64,
}

fn expanded_manhattan_sums(filename: &str, expansion_factor: i64) -> i64 {
    let factor = expansion_factor - 1;
    let mut galaxies: Vec<Galaxy> = Vec::new();
    let mut occupied_columns: BTreeSet<i64> = BTreeSet::new();
    let mut number_rows: i64 = 0;
    let mut number_columns: i64 = 0;
    let mut row_expansions: i64 = 0;
    let lines = std::fs::read_to_string(filename).expect("File not found");
    for line in lines.trim().split('\n').map(|x| x.trim()) {
        number_columns = line.len() as i64;
        let mut galaxies_in_row = false;
        let mut i: i64 = 0;
        for c in line.chars() {
            if c == '#' {
                galaxies.push(Galaxy { x: i, y: number_rows + row_expansions * factor });
                galaxies_in_row = true;
                occupied_columns.insert(i);
            }
            i += 1;
        }
        if !galaxies_in_row {
            row_expansions += 1;
        }
        number_rows += 1;
    }
    let empty_columns = (0..number_columns)
        .filter(|x| !occupied_columns.contains(x))
        .collect::<Vec<_>>();
    for i in 0..galaxies.len() {
        let mut column_expansions: i64 = 0;
        for col in &empty_columns {
            if col > &galaxies[i].x { break; }
            column_expansions += 1;
        }
        galaxies[i].x += column_expansions * factor;
    }
    let mut s: i64 = 0;
    for i in 0..galaxies.len() {
        for j in i+1..galaxies.len() {
            s += (&galaxies[j].y - &galaxies[i].y).abs() +
                 (&galaxies[j].x - &galaxies[i].x).abs();
        }
    }
    return s;
}

fn part1(filename: &str) -> i64 {
    return expanded_manhattan_sums(filename, 2);
}

fn part2(filename: &str) -> i64 {
    return expanded_manhattan_sums(filename, 1000000);
}

fn main() {
    println!("{}", part1("day11_input.txt"));
    println!("{}", part2("day11_input.txt"));
}