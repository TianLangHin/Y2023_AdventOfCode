fn mirror_positions(grid_bits: &Vec<u32>, part_two: bool) -> Vec<u32> {
    let mut bits: Vec<u32> = grid_bits.clone();
    let mut results: Vec<u32> = Vec::new();
    for i in 0..2 {
        let mut first: usize = 0;
        let last: usize = bits.len() - 1;
        while first != last {
            if ((last - first) & 1) == 0 {
                first += 1;
            } else {
                let (mut f, mut l) = (first, last);
                let mut palindrome = true;
                let mut count = 0;
                while f < l && palindrome {
                    let bit_diff = bits[f] ^ bits[l];
                    if part_two && bits[f] != bits[l] && (bit_diff & (bit_diff - 1)) == 0 {
                        palindrome = count == 0;
                        count += 1;
                    } else {
                        palindrome = bits[f] == bits[l];
                    }
                    f += 1;
                    l -= 1;
                }
                if palindrome {
                    if i == 0 {
                        results.push((1 + first + ((last - first) >> 1)) as u32);
                    } else {
                        results.push((bits.len() - (1 + first + ((last - first) >> 1))) as u32);
                    }
                }
                first += 1;
            }
        }
        bits.reverse();
    }
    if !part_two && results.len() == 0 {
        results.push(0);
    }
    return results;
}

fn part1(filename: &str) -> u32 {
    let mut s: u32 = 0;
    let mut grid: Vec<bool> = Vec::new();
    let mut row = 0;
    let mut col = 0;
    let lines = std::fs::read_to_string(filename).expect("File not found");
    for line in lines.trim().split('\n').map(|x| x.trim()) {
        if line.len() > 0 {
            grid.extend(line.chars().map(|c| c == '#'));
            row = line.len();
            col += 1;
        } else {
            let col_bits: Vec<u32> = (0..row).map(
                |i|
                (0..col)
                .fold(
                    0,
                    |acc, j|
                    if grid[i + j * row] { acc | (1 << j) } else { acc }
                )
            )
            .collect::<Vec<_>>();
            let mut result = mirror_positions(&col_bits, false)[0];
            if result == 0 {
                let row_bits: Vec<u32> = (0..col).map(
                    |i|
                    (0..row)
                    .fold(
                        0,
                        |acc, j|
                        if grid[i * row + j] { acc | (1 << j) } else { acc }
                    )
                )
                .collect::<Vec<_>>();
                result = 100 * mirror_positions(&row_bits, false)[0];
            }
            row = 0;
            col = 0;
            grid.clear();
            s += result;
        }
    }
    let col_bits: Vec<u32> = (0..row).map(
        |i|
        (0..col)
        .fold(
            0,
            |acc, j|
            if grid[i + j * row] { acc | (1 << j) } else { acc }
        )
    )
    .collect::<Vec<_>>();
    let mut result = mirror_positions(&col_bits, false)[0];
    if result == 0 {
        let row_bits: Vec<u32> = (0..col).map(
            |i|
            (0..row)
            .fold(
                0,
                |acc, j|
                if grid[i * row + j] { acc | (1 << j) } else { acc }
            )
        )
        .collect::<Vec<_>>();
        result = 100 * mirror_positions(&row_bits, false)[0];
    }
    grid.clear();
    s += result;
    return s;
}

fn part2(filename: &str) -> u32 {
    let mut s: u32 = 0;
    let mut grid: Vec<bool> = Vec::new();
    let mut row = 0;
    let mut col = 0;
    let lines = std::fs::read_to_string(filename).expect("File not found");
    for line in lines.trim().split('\n').map(|x| x.trim()) {
        if line.len() > 0 {
            grid.extend(line.chars().map(|c| c == '#'));
            row = line.len();
            col += 1;
        } else {
            let col_bits: Vec<u32> = (0..row).map(
                |i|
                (0..col)
                .fold(
                    0,
                    |acc, j|
                    if grid[i + j * row] { acc | (1 << j) } else { acc }
                )
            )
            .collect::<Vec<_>>();
            let row_bits: Vec<u32> = (0..col).map(
                |i|
                (0..row)
                .fold(
                    0,
                    |acc, j|
                    if grid[i * row + j] { acc | (1 << j) } else { acc }
                )
            )
            .collect::<Vec<_>>();
            let mut part_one_result = mirror_positions(&col_bits, false)[0];
            if part_one_result == 0 {
                part_one_result = 100 * mirror_positions(&row_bits, false)[0];
            }
            let mut results = mirror_positions(&col_bits, true);
            results.extend(mirror_positions(&row_bits, true).into_iter().map(|x| 100*x));
            if results.len() == 0 {
                results.push(0);
            }
            row = 0;
            col = 0;
            grid.clear();
            s += results
                .iter()
                .filter(|&&x| x != part_one_result)
                .reduce(|x1, x2| if x1 < x2 { x1 } else { x2 })
                .unwrap();
        }
    }
    let col_bits: Vec<u32> = (0..row).map(
        |i|
        (0..col)
        .fold(
            0,
            |acc, j|
            if grid[i + j * row] { acc | (1 << j) } else { acc }
        )
    )
    .collect::<Vec<_>>();
    let row_bits: Vec<u32> = (0..col).map(
        |i|
        (0..row)
        .fold(
            0,
            |acc, j|
            if grid[i * row + j] { acc | (1 << j) } else { acc }
        )
    )
    .collect::<Vec<_>>();
    let mut part_one_result = mirror_positions(&col_bits, false)[0];
    if part_one_result == 0 {
        part_one_result = 100 * mirror_positions(&row_bits, false)[0];
    }
    let mut results = mirror_positions(&col_bits, true);
    results.extend(mirror_positions(&row_bits, true).into_iter().map(|x| 100*x));
    if results.len() == 0 {
        results.push(0);
    }
    grid.clear();
    s += results
        .iter()
        .filter(|&&x| x != part_one_result)
        .reduce(|x1, x2| if x1 < x2 { x1 } else { x2 })
        .unwrap();
    return s;
}

fn main() {
    println!("{}", part1("day13_input.txt"));
    println!("{}", part2("day13_input.txt"));
}