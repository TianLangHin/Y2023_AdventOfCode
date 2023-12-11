pub struct MarkedPipe {
    pub main: bool,
    pub p1: i64,
    pub p2: i64
}

pub struct MapInfo {
    pub pipes: Vec<MarkedPipe>,
    pub max_rows: i64,
    pub row_length: i64,
    pub s_index: i64
}

fn to_pipe(character: char, row_length: i64) -> Option<MarkedPipe> {
    match character {
        '|' => Some(MarkedPipe { main: false, p1: -row_length, p2: row_length }),
        '-' => Some(MarkedPipe { main: false, p1: -1, p2: 1 }),
        'L' => Some(MarkedPipe { main: false, p1: -row_length, p2: 1 }),
        'J' => Some(MarkedPipe { main: false, p1: -row_length, p2: -1 }),
        '7' => Some(MarkedPipe { main: false, p1: -1, p2: row_length }),
        'F' => Some(MarkedPipe { main: false, p1: 1, p2: row_length }),
        '.' => Some(MarkedPipe { main: false, p1: 0, p2: 0 }),
        'S' => Some(MarkedPipe { main: false, p1: 1, p2: 1 }),
        _   => None
    }
}

fn make_map(filename: &str) -> MapInfo {
    let mut pipes: Vec<MarkedPipe> = Vec::new();
    let mut s_index: i64 = 0;
    let mut max_rows: i64 = 0;
    let mut n: i64 = 1;
    let lines = std::fs::read_to_string(filename).expect("File not found");
    for line in lines.trim().split('\n') {
        n = line.len() as i64;
        let row: Vec<MarkedPipe> = line.chars().map(|x| to_pipe(x, line.len() as i64).unwrap()).collect();
        for i in 0..row.len() {
            if row[i].p1 == 1 && row[i].p2 == 1 {
                s_index = (pipes.len() + i) as i64;
            }
        }
        pipes.extend(row.into_iter());
        max_rows += 1;
    }
    let mut directions: Vec<i64> = Vec::new();
    if s_index / n > 0 {
        if pipes[(s_index - n) as usize].p1 == n || pipes[(s_index - n) as usize].p2 == n {
            directions.push(-n);
        }
    }
    if s_index % n > 0 {
        if pipes[(s_index - 1) as usize].p1 == 1 || pipes[(s_index - 1) as usize].p2 == 1 {
            directions.push(-1);
        }
    }
    if s_index % n < n-1 {
        if pipes[(s_index + 1) as usize].p1 == -1 || pipes[(s_index + 1) as usize].p2 == -1 {
            directions.push(1);
        }
    }
    if s_index / n < max_rows-1 {
        if pipes[(s_index + n) as usize].p1 == -n || pipes[(s_index + n) as usize].p2 == -n {
            directions.push(n);
        }
    }
    pipes[s_index as usize] = MarkedPipe { main: false, p1: directions[0], p2: directions[1] };
    return MapInfo { pipes: pipes, max_rows: max_rows, row_length: n, s_index: s_index };
}

fn part1(filename: &str) -> i64 {
    let info = make_map(filename);
    let mut path_length: i64 = 0;
    let mut i = info.s_index;
    let mut choice = true;
    let mut next_pipe_offset: i64;
    let pipes = info.pipes;
    loop {
        next_pipe_offset = if choice { pipes[i as usize].p1 } else { pipes[i as usize].p2 };
        choice = pipes[(i + next_pipe_offset) as usize].p1 + next_pipe_offset != 0;
        i += next_pipe_offset;
        path_length += 1;
        if i == info.s_index { break; }
    }
    return path_length >> 1;
}

fn part2(filename: &str) -> i64 {
    let info = make_map(filename);
    let mut i = info.s_index;
    let mut choice = true;
    let mut next_pipe_offset: i64;
    let mut pipes = info.pipes;
    loop {
        pipes[i as usize].main = true;
        next_pipe_offset = if choice { pipes[i as usize].p1 } else { pipes[i as usize].p2 };
        choice = pipes[(i + next_pipe_offset) as usize].p1 + next_pipe_offset != 0;
        i += next_pipe_offset;
        if i == info.s_index { break; }
    }
    let n = info.row_length;
    let mut s: i64 = 0;
    for row_index in 0..info.max_rows {
        let mut inside_loop: bool = false;
        let mut previous_angle: Option<bool> = None;
        for i in 0..n {
            let tile = &pipes[(row_index * n + i) as usize];
            if tile.main {
                if tile.p1 == -n && tile.p2 == n {
                    inside_loop = !inside_loop;
                } else if tile.p1 != -1 || tile.p2 != 1 {
                    let current_angle = tile.p2 == n;
                    if let Some(prev) = previous_angle {
                        if current_angle != prev {
                            inside_loop = !inside_loop;
                        }
                        previous_angle = None;
                    } else {
                        previous_angle = Some(current_angle);
                    }
                }
            } else if inside_loop {
                s += 1;
            }
        }
    }
    return s;
}

fn main() {
    println!("{}", part1("day10_input.txt"));
    println!("{}", part2("day10_input.txt"));
}