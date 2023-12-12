use std::collections::HashMap;

#[derive(Clone, Copy)]
pub enum Spring {
    Operational,
    Unknown,
    Damaged,
}

pub struct Memo<'a> {
    pub row_length: usize,
    pub cache: &'a mut HashMap<usize, usize>,
    pub springs: &'a Vec<Spring>,
    pub populations: &'a Vec<usize>,
}

fn suitable_jump(i: usize, j: usize, memo: &Memo) -> bool {
    i + memo.populations[j] <= memo.springs.len() &&
    !memo.springs[i..i + memo.populations[j]].iter()
        .any(|x| match x {Spring::Operational => true, _ => false}) &&
    !(i + memo.populations[j] < memo.springs.len() &&
        match memo.springs[i + memo.populations[j]] {Spring::Damaged => true, _ => false})
}

fn solve(i: usize, j: usize, memo: &mut Memo) -> usize {
    if j == memo.populations.len() {
        if i < memo.springs.len() &&
            memo.springs[i..].iter().any(|x| match x {Spring::Damaged => true, _ => false}) {
            return 0;
        } else {
            memo.cache.insert(i * memo.row_length + j, 1);
            return 1;
        }
    } else if i >= memo.springs.len() {
        return 0;
    }
    if let Some(&cached_result) = memo.cache.get(&(i * memo.row_length + j)) {
        return cached_result;
    }

    let this_skip: usize = match memo.springs[i] {
        Spring::Damaged => 0,
        _ => solve(i+1, j, memo)
    };
    let this_match: usize = match memo.springs[i] {
        Spring::Operational => 0,
        _ if suitable_jump(i, j, memo) => solve(i + memo.populations[j] + 1, j + 1, memo),
        _ => 0,
    };
    memo.cache.insert(i * memo.row_length + j, this_skip + this_match);
    
    return this_skip + this_match;
}

fn part1(filename: &str) -> usize {
    let mut s: usize = 0;
    let lines = std::fs::read_to_string(filename).expect("File not found");
    for (length, springs, populations) in lines
        .trim()
        .split('\n')
        .map(|x|
            (x.trim().len(), 
             x.trim().split_whitespace().collect::<Vec<_>>()))
        .map(|(l, x)|
             (l,
              x[0].chars().map(|x|
                  match x {
                      '.' => Spring::Operational,
                      '?' => Spring::Unknown,
                      '#' => Spring::Damaged,
                      _ => panic!()
                  }
              ).collect::<Vec<_>>(),
              x[1]
              .split(',')
              .map(|x| x.parse::<usize>().unwrap())
              .collect::<Vec<_>>()))
    {
        let mut cache: HashMap<usize, usize> = HashMap::new();
        let mut memo = Memo {
            row_length: length,
            cache: &mut cache,
            springs: &springs,
            populations: &populations
        };
        s += solve(0, 0, &mut memo);
    }
    return s;
}

fn part2(filename: &str) -> usize {
    let mut s: usize = 0;
    let lines = std::fs::read_to_string(filename).expect("File not found");
    for (length, springs, populations) in lines
        .trim()
        .split('\n')
        .map(|x|
            (x.trim().len(), 
             x.trim().split_whitespace().collect::<Vec<_>>()))
        .map(|(l, x)|
             (l,
              x[0].chars().map(|x|
                  match x {
                      '.' => Spring::Operational,
                      '?' => Spring::Unknown,
                      '#' => Spring::Damaged,
                      _ => panic!()
                  }
              ).collect::<Vec<_>>(),
              x[1]
              .split(',')
              .map(|x| x.parse::<usize>().unwrap())
              .collect::<Vec<_>>()))
    {
        let mut cache: HashMap<usize, usize> = HashMap::new();
        let mut unfolded_springs: Vec<Spring> = Vec::new();
        unfolded_springs.extend(springs.iter().map(|&x| x));
        for _ in 0..4 {
            unfolded_springs.push(Spring::Unknown);
        unfolded_springs.extend(springs.iter().map(|&x| x));
        }
        let mut unfolded_populations: Vec<usize> = Vec::new();
        for _ in 0..5 {
            unfolded_populations.extend(populations.iter().map(|&x| x));
        }
        let mut memo = Memo {
            row_length: length,
            cache: &mut cache,
            springs: &unfolded_springs,
            populations: &unfolded_populations
        };
        s += solve(0, 0, &mut memo);
    }
    return s;
}

fn main() {
    println!("{}", part1("day12_input.txt"));
    println!("{}", part2("day12_input.txt"));
}
