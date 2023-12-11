use std::collections::HashSet;

fn expanded_manhattan_sums(filename: &str, expansion_factor: u64) -> u64 {
    let factor = expansion_factor - 1;
    let mut empty_rows: HashSet<u64> = HashSet::new();
    let mut full_columns: HashSet<u64> = HashSet::new();
    let mut number_rows: u64 = 0;
    let mut number_columns: u64 = 0;
    let mut galaxies: Vec<u64> = Vec::new();
    let lines = std::fs::read_to_string(filename).expect("File not found");
    for line in lines.trim().split('\n').map(|x| x.trim()) {
        number_columns = line.len() as u64;
        let mut galaxies_in_row = false;
        let mut i: u64 = 0;
        for c in line.chars() {
            if c == '#' {
                galaxies.push(number_columns * number_rows + i);
                galaxies_in_row = true;
                full_columns.insert(i);
            }
            i += 1;
        }
        if !galaxies_in_row { empty_rows.insert(number_rows); }
        number_rows += 1;
    }
    let empty_columns = HashSet::from_iter(0..number_columns)
        .difference(&full_columns)
        .map(|&x| x)
        .collect::<HashSet<u64>>();
    let mut s: u64 = 0;
    let mut a_row: u64;
    let mut a_col: u64;
    let mut b_row: u64;
    let mut b_col: u64;
    for i in 0..galaxies.len() {
        for j in i+1..galaxies.len() {
            a_row = galaxies[i] / number_columns;
            a_col = galaxies[i] % number_columns;
            b_row = galaxies[j] / number_columns;
            b_col = galaxies[j] % number_columns;
            if a_col > b_col {
                (a_col, b_col) = (b_col, a_col);
            }
            if a_row > b_row {
                (a_row, b_row) = (b_row, a_row);
            }
            s += (b_col - a_col) +
                 factor * empty_rows
                    .intersection(&HashSet::from_iter(a_row..b_row))
                    .collect::<HashSet<_>>().len() as u64 +
                 (b_row - a_row) +
                 factor * empty_columns
                    .intersection(&HashSet::from_iter(a_col..b_col))
                    .collect::<HashSet<_>>().len() as u64
        }
    }
    return s;
}

fn part1(filename: &str) -> u64 {
    return expanded_manhattan_sums(filename, 2);
}

fn part2(filename: &str) -> u64 {
    return expanded_manhattan_sums(filename, 1000000);
}

fn main() {
    println!("{}", part1("day11_input.txt"));
    println!("{}", part2("day11_input.txt"));
}