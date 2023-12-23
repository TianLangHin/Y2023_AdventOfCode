use std::collections::{BTreeSet, HashMap};

#[derive(Clone, Copy)]
pub enum Tile {
    Empty,
    Up,
    Left,
    Down,
    Right
}

fn tile_eq(t1: &Tile, t2: &Tile) -> bool {
    match (&t1, &t2) {
        (Tile::Empty,   Tile::Empty)
        | (Tile::Up,    Tile::Up)
        | (Tile::Left,  Tile::Left)
        | (Tile::Down,  Tile::Down)
        | (Tile::Right, Tile::Right) => true,
        _ => false
    }
}

fn dfs(start: usize,
       end: usize,
       prev_weight: u64,
       traversed: &mut Vec<usize>,
       legal_moves: &HashMap<usize, Vec<(usize, u64)>>) -> u64 {
    if start == end { return prev_weight; }
    return legal_moves
        .get(&start)
        .unwrap()
        .iter()
        .fold(
            0,
            |acc, &legal_move| {
                if !traversed.contains(&legal_move.0) {
                    traversed.push(legal_move.0);
                    let result = dfs(
                        legal_move.0,
                        end,
                        prev_weight + legal_move.1,
                        traversed,
                        legal_moves
                    );
                    traversed.pop();
                    std::cmp::max(acc, result)
                } else {
                    acc
                }
            }
        );
}

fn step_forward(index: usize,
                next_square: usize,
                next_steps: &HashMap<usize, BTreeSet<usize>>) -> (usize, u64) {
    let mut next_sq = next_square;
    let mut step_count = 1u64;
    let mut possibilities: BTreeSet<usize> = next_steps
        .get(&next_square)
        .unwrap()
        .iter()
        .cloned()
        .collect();
    possibilities.remove(&index);
    let mut idx = next_sq;
    while possibilities.len() == 1 {
        next_sq = possibilities.pop_first().unwrap();
        possibilities = next_steps
            .get(&next_sq)
            .unwrap()
            .iter()
            .cloned()
            .collect();
        possibilities.remove(&idx);
        idx = next_sq;
        step_count += 1;
    }
    return (idx, step_count);
}

fn part1(filename: &str) -> u64 {
    let file_text = std::fs::read_to_string(filename).unwrap();
    let lines = file_text.trim().split('\n').map(|x| x.trim()).collect::<Vec<_>>();
    let x_bound = lines[0].len();
    let y_bound = lines.len();
    let starting_index = lines[0]
        .chars()
        .enumerate()
        .find_map(|(i, c)| if c == '.' { Some(i) } else { None })
        .unwrap();
    let ending_index = lines[lines.len() - 1]
        .chars()
        .enumerate()
        .find_map(|(i, c)| if c == '.' { Some(i) } else { None })
        .unwrap()
        + x_bound * (y_bound - 1);
    let mut grid = HashMap::<usize, Tile>::new();
    let mut i: usize = 0;
    for line in lines {
        for c in line.chars() {
            match c {
                '.' => grid.insert(i, Tile::Empty),
                '^' => grid.insert(i, Tile::Up),
                '<' => grid.insert(i, Tile::Left),
                'v' => grid.insert(i, Tile::Down),
                '>' => grid.insert(i, Tile::Right),
                _   => None
            };
            i += 1;
        }
    }
    let mut next_steps = HashMap::<usize, BTreeSet<usize>>::new();
    for (&point, &direction) in &grid {
        match direction {
            Tile::Empty => {
                let mut new_set = BTreeSet::<usize>::new();
                if point % x_bound > 0
                && !tile_eq(grid.get(&(point - 1)).unwrap_or(&Tile::Right), &Tile::Right) {
                    new_set.insert(point - 1);
                }
                if point % x_bound < x_bound - 1
                && !tile_eq(grid.get(&(point + 1)).unwrap_or(&Tile::Left), &Tile::Left) {
                    new_set.insert(point + 1);
                }
                if point / x_bound > 0
                && !tile_eq(grid.get(&(point - x_bound)).unwrap_or(&Tile::Down), &Tile::Down) {
                    new_set.insert(point - x_bound);
                }
                if point / x_bound < y_bound - 1
                && !tile_eq(grid.get(&(point + x_bound)).unwrap_or(&Tile::Up), &Tile::Up) {
                    new_set.insert(point + x_bound);
                }
                next_steps.insert(point, new_set);
            },
            Tile::Up => if grid.contains_key(&(point - x_bound)) {
                next_steps.insert(point, [point - x_bound].into_iter().collect());
            },
            Tile::Left => if grid.contains_key(&(point - 1)) {
                next_steps.insert(point, [point - 1].into_iter().collect());
            },
            Tile::Down => if grid.contains_key(&(point + x_bound)) {
                next_steps.insert(point, [point + x_bound].into_iter().collect());
            },
            Tile::Right => if grid.contains_key(&(point + 1)) {
                next_steps.insert(point, [point + 1].into_iter().collect());
            }
        }
    }
    let mut edges = HashMap::<(usize, usize), u64>::new();
    let mut searching = next_steps
        .get(&starting_index)
        .unwrap()
        .iter()
        .map(|&nxt| (starting_index, nxt))
        .collect::<BTreeSet<(usize, usize)>>();
    while searching.len() > 0 {
        let mut new_searching = BTreeSet::<(usize, usize)>::new();
        for &(index, next_square) in &searching {
            let (idx, step_count) = step_forward(index, next_square, &next_steps);
            edges.insert(
                (index, idx),
                std::cmp::max(step_count, *edges.get(&(index, idx)).unwrap_or(&0))
            );
            if idx != ending_index {
                for &nxt in next_steps.get(&idx).unwrap() {
                    if nxt != index { new_searching.insert((idx, nxt)); }
                }
            }
        }
        searching = new_searching;
    }
    let mut legal_moves = HashMap::<usize, Vec<(usize, u64)>>::new();
    for (&(start, end), &weight) in &edges {
        if legal_moves.contains_key(&start) {
            legal_moves.get_mut(&start).unwrap().push((end, weight));
        } else {
            legal_moves.insert(start, vec![(end, weight)]);
        }
    }
    let mut traversed = vec![starting_index];
    return dfs(starting_index, ending_index, 0, &mut traversed, &legal_moves);
}

fn step_until_junction(index: usize,
                       next_square: usize,
                       junctions: &BTreeSet<usize>,
                       next_steps: &HashMap<usize, BTreeSet<usize>>) -> (usize, u64) {
    let mut next_sq = next_square;
    let mut step_count = 1u64;
    let mut possibilities: BTreeSet<usize> = next_steps
        .get(&next_square)
        .unwrap()
        .iter()
        .cloned()
        .collect();
    possibilities.remove(&index);
    let mut idx = next_sq;
    while !junctions.contains(&next_sq) {
        next_sq = possibilities.pop_first().unwrap();
        possibilities = next_steps
            .get(&next_sq)
            .unwrap()
            .iter()
            .cloned()
            .collect();
        possibilities.remove(&idx);
        idx = next_sq;
        step_count += 1;
    }
    return (idx, step_count);
}

fn part2(filename: &str) -> u64 {
    let file_text = std::fs::read_to_string(filename).unwrap();
    let lines = file_text.trim().split('\n').map(|x| x.trim()).collect::<Vec<_>>();
    let x_bound = lines[0].len();
    let y_bound = lines.len();
    let starting_index = lines[0]
        .chars()
        .enumerate()
        .find_map(|(i, c)| if c == '.' { Some(i) } else { None })
        .unwrap();
    let ending_index = lines[lines.len() - 1]
        .chars()
        .enumerate()
        .find_map(|(i, c)| if c == '.' { Some(i) } else { None })
        .unwrap()
        + x_bound * (y_bound - 1);
    let mut grid = HashMap::<usize, Tile>::new();
    let mut i: usize = 0;
    for line in lines {
        for c in line.chars() {
            match c {
                '.' => grid.insert(i, Tile::Empty),
                '^' => grid.insert(i, Tile::Up),
                '<' => grid.insert(i, Tile::Left),
                'v' => grid.insert(i, Tile::Down),
                '>' => grid.insert(i, Tile::Right),
                _   => None
            };
            i += 1;
        }
    }
    let mut junctions: BTreeSet<usize> = [ending_index].into_iter().collect();
    let mut next_steps = HashMap::<usize, BTreeSet<usize>>::new();
    for (&point, &_) in &grid {
        let mut count = 0;
        let mut new_set = BTreeSet::<usize>::new();
        if point % x_bound > 0 {
            if let Some(tile) = grid.get(&(point - 1)) {
                count += 1;
                if !tile_eq(tile, &Tile::Right) { new_set.insert(point - 1); }
            }
        }
        if point % x_bound < x_bound - 1 {
            if let Some(tile) = grid.get(&(point + 1)) {
                count += 1;
                if !tile_eq(tile, &Tile::Left) { new_set.insert(point + 1); }
            }
        }
        if point / x_bound > 0 {
            if let Some(tile) = grid.get(&(point - x_bound)) {
                count += 1;
                if !tile_eq(tile, &Tile::Down) { new_set.insert(point - x_bound); }
            }
        }
        if point / x_bound < y_bound - 1 {
            if let Some(tile) = grid.get(&(point + x_bound)) {
                count += 1;
                if !tile_eq(tile, &Tile::Up) { new_set.insert(point + x_bound); }
            }
        }
        if count > 2 { junctions.insert(point); }
        next_steps.insert(point, new_set);
    }
    let mut edges = HashMap::<(usize, usize), u64>::new();
    let mut searching = next_steps
        .get(&starting_index)
        .unwrap()
        .iter()
        .map(|&nxt| (starting_index, nxt))
        .collect::<BTreeSet<(usize, usize)>>();
    while searching.len() > 0 {
        let mut new_searching = BTreeSet::<(usize, usize)>::new();
        for &(index, next_square) in &searching {
            let (idx, step_count) = step_until_junction(index, next_square, &junctions, &next_steps);
            edges.insert(
                (index, idx),
                std::cmp::max(step_count, *edges.get(&(index, idx)).unwrap_or(&0))
            );
            if idx != ending_index {
                for &nxt in next_steps.get(&idx).unwrap() {
                    if nxt != index { new_searching.insert((idx, nxt)); }
                }
            }
        }
        searching = new_searching;
    }
    let mut legal_moves = HashMap::<usize, Vec<(usize, u64)>>::new();
    for (&(start, end), &weight) in &edges {
        if legal_moves.contains_key(&start) {
            legal_moves.get_mut(&start).unwrap().push((end, weight));
        } else {
            legal_moves.insert(start, vec![(end, weight)]);
        }
        if legal_moves.contains_key(&end) {
            legal_moves.get_mut(&end).unwrap().push((start, weight));
        } else {
            legal_moves.insert(end, vec![(start, weight)]);
        }
    }
    let mut traversed = vec![starting_index];
    return dfs(starting_index, ending_index, 0, &mut traversed, &legal_moves);
}

fn main() {
    println!("{}", part1("day23_input.txt"));
    println!("{}", part2("day23_input.txt"));
}
