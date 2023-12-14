use num_bigint::BigUint;

#[derive(Clone, Copy, Debug)]
pub enum Rock {
    Empty,
    Round,
    Cube
}

fn one_cycle(grid: &mut Vec<Rock>, x_bound: usize, y_bound: usize) {
    let mut anchors: Vec<usize> = Vec::new();
    for _ in 0..x_bound { anchors.push(0); }
    for i in 0..y_bound {
        for j in 0..x_bound {
            match grid[i * x_bound + j] {
                Rock::Empty => {},
                Rock::Round => {
                    grid[i * x_bound + j] = Rock::Empty;
                    grid[anchors[j] * x_bound + j] = Rock::Round;
                    anchors[j] += 1;
                },
                Rock::Cube => anchors[j] = i+1
            }
        }
    }
    let mut anchors: Vec<usize> = Vec::new();
    for _ in 0..y_bound { anchors.push(0); }
    for i in 0..y_bound {
        for j in 0..x_bound {
            match grid[i * x_bound + j] {
                Rock::Empty => {},
                Rock::Round => {
                    grid[i * x_bound + j] = Rock::Empty;
                    grid[i * x_bound + anchors[i]] = Rock::Round;
                    anchors[i] += 1;
                },
                Rock::Cube => anchors[i] = j+1
            }
        }
    }
    let mut anchors: Vec<usize> = Vec::new();
    for _ in 0..x_bound { anchors.push(x_bound); }
    for i in (0..y_bound).rev() {
        for j in (0..x_bound).rev() {
            match grid[i * x_bound + j] {
                Rock::Empty => {},
                Rock::Round => {
                    grid[i * x_bound + j] = Rock::Empty;
                    grid[(anchors[j] - 1) * x_bound + j] = Rock::Round;
                    anchors[j] -= 1;
                },
                Rock::Cube => anchors[j] = i
            }
        }
    }
    let mut anchors: Vec<usize> = Vec::new();
    for _ in 0..y_bound { anchors.push(y_bound); }
    for i in (0..x_bound).rev() {
        for j in (0..y_bound).rev() {
            match grid[i * x_bound + j] {
                Rock::Empty => {},
                Rock::Round => {
                    grid[i * x_bound + j] = Rock::Empty;
                    grid[i * x_bound + (anchors[i] - 1)] = Rock::Round;
                    anchors[i] -= 1;
                },
                Rock::Cube => anchors[i] = j
            }
        }
    }
}

fn part1(filename: &str) -> usize {
    let file_lines = std::fs::read_to_string(filename).expect("File not found");
    let lines = file_lines.trim().split('\n').map(|x| x.trim()).collect::<Vec<_>>();
    let n = lines[0].len();
    let mut last_anchor: Vec<usize> = Vec::new();
    let mut column_weights: Vec<(usize, usize)> = Vec::new();
    for _ in 0..n {
        last_anchor.push(0);
        column_weights.push((0, 0));
    }
    let mut row_number: usize = 0;
    let mut i: usize;
    for line in lines {
        i = 0;
        for c in line.chars() {
            match c {
                '#' => last_anchor[i] = row_number + 1,
                'O' => {
                    column_weights[i] = (
                        column_weights[i].0 + 1,
                        column_weights[i].1 + last_anchor[i]
                    );
                    last_anchor[i] += 1;
                }
                _ => {}
            }
            i += 1;
        }
        row_number += 1;
    }
    return column_weights
        .iter()
        .map(|(c, s)| c * row_number - s)
        .reduce(|acc, x| acc + x)
        .unwrap();
}

fn grid_to_key(grid: &Vec<Rock>) -> BigUint {
    let mut key: BigUint = BigUint::from_bytes_le(&[0]);
    for rock in grid.iter().rev() {
        key <<= 2;
        key += match rock {
            Rock::Empty => 0,
            Rock::Round => 1,
            Rock::Cube  => 2
        } as u8;
    }
    return key;
}

fn key_to_grid(key: &BigUint, total_length: usize) -> Vec<Rock> {
    let mut key_copy: BigUint = key.clone();
    let mut grid: Vec<Rock> = Vec::new();
    let mask: BigUint = BigUint::from_bytes_le(&[3]);
    let two: BigUint = BigUint::from_bytes_le(&[2]);
    let one: BigUint = BigUint::from_bytes_le(&[1]);
    while key_copy.count_ones() != 0 {
        if(&key_copy & &mask) == two {
            grid.push(Rock::Cube);
        } else if (&key_copy & &mask) == one {
            grid.push(Rock::Round);
        } else {
            grid.push(Rock::Empty);
        }
        key_copy >>= 2;
    }
    for _ in 0..(total_length - grid.len()) {
        grid.push(Rock::Empty);
    }
    return grid;
}

fn part2(filename: &str) -> usize {
    let file_lines = std::fs::read_to_string(filename).expect("File not found");
    let lines = file_lines.trim().split('\n').map(|x| x.trim()).collect::<Vec<_>>();
    let x_bound = lines[0].len();
    let y_bound = lines.len();
    let mut grid: Vec<Rock> = Vec::new();
    for line in lines {
        for c in line.chars() {
            match c {
                '#' => { grid.push(Rock::Cube); },
                'O' => { grid.push(Rock::Round); },
                '.' => { grid.push(Rock::Empty); },
                _ => {}
            }
        }
    }
    let mut memo: Vec<BigUint> = Vec::new();
    let mut index: Option<usize>;
    let mut new_key: BigUint;
    memo.push(grid_to_key(&grid));
    loop {
        one_cycle(&mut grid, x_bound, y_bound);
        new_key = grid_to_key(&grid);
        index = None;
        for i in 0..memo.len() {
            if memo[i] == new_key {
                index = Some(i);
                break;
            }
        }
        if let Some(_) = index {
            break;
        } else {
            memo.push(new_key);
        }
    }
    let final_result = &memo[
        (1000000000 - index.unwrap()) % (memo.len() - index.unwrap()) + index.unwrap()
    ];
    let final_grid = key_to_grid(&final_result, x_bound * y_bound);
    let mut final_load: usize = 0;
    for i in 0..y_bound {
        for j in 0..x_bound {
            final_load += match final_grid[i * x_bound + j] {
                Rock::Round => y_bound - i,
                _ => 0
            };
        }
    }
    return final_load;
}

fn main() {
    println!("{}", part1("day14_input.txt"));
    println!("{}", part2("day14_input.txt"));
}