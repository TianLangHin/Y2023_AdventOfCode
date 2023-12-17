use std::collections::BTreeSet;

const RIGHT: usize = 0;
const DOWN:  usize = 1;
const UP:    usize = 2;
const LEFT:  usize = 3;

fn in_bound(direction: usize, pos: usize, x_bound: usize, y_bound: usize) -> bool {
    match direction {
        RIGHT => pos % x_bound < x_bound - 1,
        LEFT  => pos % x_bound > 0,
        DOWN  => pos / x_bound < y_bound - 1,
        UP    => pos / x_bound > 0,
        _ => false
    }
}

fn increment(index: usize, direction: usize, x_bound: usize) -> usize {
    match direction {
        RIGHT => index + 1,
        LEFT  => index - 1,
        DOWN  => index + x_bound,
        UP    => index - x_bound,
        _ => 0
    }
}

fn part1(filename: &str) -> usize {
    let text_lines = std::fs::read_to_string(filename).unwrap();
    let lines = text_lines.trim().split('\n').map(|x| x.trim()).collect::<Vec<_>>();
    let x_bound = lines[0].len();
    let y_bound = lines.len();
    let mut grid: Vec<usize> = Vec::new();
    for line in lines {
        grid.extend(line.chars().map(|x| (x.to_digit(10).unwrap()) as usize));
    }
    let max_cost = 9 * (x_bound + y_bound);
    let mut costs = vec![max_cost; grid.len() * 4 * 4];
    let stopping_point = grid.len() - 1;
    costs[0] = 0;
    let mut paths: BTreeSet<usize> = BTreeSet::new();
    paths.insert(0);
    while paths.len() > 0 {
        let mut new_paths: BTreeSet<usize> = BTreeSet::new();
        for &index in paths.iter() {
            for search_direction in 0..4 {
                if in_bound(search_direction, index >> 4, x_bound, y_bound) {
                    if index == 0 {
                        let i = (increment(0, search_direction, x_bound) << 4)
                            | 4
                            | search_direction;
                        let new_cost = costs[index] + grid[i >> 4];
                        if new_cost >= costs[i] { continue; }
                        costs[i] = new_cost;
                        if (i >> 4) != stopping_point {
                            new_paths.insert(i);
                        }
                    } else if search_direction == (index & 3) {
                        if (index & 12) != 12 {
                            let i = (increment(index >> 4, search_direction, x_bound) << 4)
                                + (index & 15)
                                + 4;
                            let new_cost = costs[index] + grid[i >> 4];
                            if new_cost >= costs[i] { continue; }
                            costs[i] = new_cost;
                            if (i >> 4) != stopping_point {
                                new_paths.insert(i);
                            }
                        }
                    } else if search_direction + (index & 3) != 3 {
                        let i = (increment(index >> 4, search_direction, x_bound) << 4)
                            | 4
                            | search_direction;
                        let new_cost = costs[index] + grid[i >> 4];
                        if new_cost >= costs[i] { continue; }
                        costs[i] = new_cost;
                        if (i >> 4) != stopping_point {
                            new_paths.insert(i);
                        }
                    }
                }
            }
        }
        paths = new_paths;
    }
    return costs[stopping_point << 4 .. (stopping_point << 4) + 16]
        .iter()
        .fold(max_cost, |acc, &x| if acc < x { acc } else { x });
}

fn part2(filename: &str) -> usize {
    let text_lines = std::fs::read_to_string(filename).unwrap();
    let lines = text_lines.trim().split('\n').map(|x| x.trim()).collect::<Vec<_>>();
    let x_bound = lines[0].len();
    let y_bound = lines.len();
    let mut grid: Vec<usize> = Vec::new();
    for line in lines {
        grid.extend(line.chars().map(|x| (x.to_digit(10).unwrap()) as usize));
    }
    let max_cost = 9 * (x_bound + y_bound);
    let mut costs = vec![max_cost; grid.len() * 4 * 16];
    let stopping_point = grid.len() - 1;
    costs[0] = 0;
    let mut paths: BTreeSet<usize> = BTreeSet::new();
    paths.insert(0);
    while paths.len() > 0 {
        let mut new_paths: BTreeSet<usize> = BTreeSet::new();
        for &index in paths.iter() {
            for search_direction in 0..4 {
                if in_bound(search_direction, index >> 6, x_bound, y_bound) {
                    if index == 0 {
                        let i = (increment(0, search_direction, x_bound) << 6)
                            | 4
                            | search_direction;
                        let new_cost = costs[index] + grid[i >> 6];
                        if new_cost >= costs[i] { continue; }
                        costs[i] = new_cost;
                        if (i >> 6) != stopping_point {
                            new_paths.insert(i);
                        }
                    } else if search_direction == (index & 3) {
                        if (index & 60) < 40 {
                            let i = (increment(index >> 6, search_direction, x_bound) << 6)
                                + (index & 63)
                                + 4;
                            let new_cost = costs[index] + grid[i >> 6];
                            if new_cost >= costs[i] { continue; }
                            if (i >> 6) == stopping_point {
                                if (i & 60) >= 16 {
                                    costs[i] = new_cost;
                                }
                            } else {
                                costs[i] = new_cost;
                                new_paths.insert(i);
                            }
                        }
                    } else if search_direction + (index & 3) != 3 {
                        if (index & 60) >= 16 {
                            let i = (increment(index >> 6, search_direction, x_bound) << 6)
                                | 4
                                | search_direction;
                            let new_cost = costs[index] + grid[i >> 6];
                            if new_cost >= costs[i] { continue; }
                            if (i >> 6) != stopping_point {
                                costs[i] = new_cost;
                                new_paths.insert(i);
                            }
                        }
                    }
                }
            }
        }
        paths = new_paths;
    }
    return costs[stopping_point << 6 .. (stopping_point << 6) + 64]
        .iter()
        .fold(max_cost, |acc, &x| if acc < x { acc } else { x });
}

fn main() {
    println!("{}", part1("day17_input.txt"));
    println!("{}", part2("day17_input.txt"));
}