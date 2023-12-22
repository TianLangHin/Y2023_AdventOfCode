use std::collections::BTreeSet;

fn make_steps(index: usize, x_bound: usize, y_bound: usize) -> BTreeSet<usize> {
    let mut steps = BTreeSet::<usize>::new();
    if index / x_bound > 0 { steps.insert(index - x_bound); }
    if index / x_bound < y_bound - 1 { steps.insert(index + x_bound); }
    if index % x_bound > 0 { steps.insert(index - 1); }
    if index % x_bound < x_bound - 1 { steps.insert(index + 1); }
    return steps;
}

fn part1(filename: &str) -> usize {
    let file_lines = std::fs::read_to_string(filename).expect("File not found");
    let lines = file_lines.trim().split('\n').map(|x| x.trim()).collect::<Vec<_>>();
    let x_bound = lines[0].len();
    let y_bound = lines.len();
    let mut gardens = BTreeSet::<usize>::new();
    let mut start: usize = 0;
    let mut i: usize = 0;
    for line in lines {
        for c in line.chars() {
            match c {
                '.' => {
                    gardens.insert(i);
                },
                'S' => {
                    start = i;
                    gardens.insert(i);
                }
                _ => {}
            }
            i += 1;
        }
    }
    let mut steps = BTreeSet::<usize>::new();
    steps.insert(start);
    for _ in 0..64 {
        let mut new_steps = BTreeSet::<usize>::new();
        for step in steps {
            for elem in make_steps(step, x_bound, y_bound) {
                new_steps.insert(elem);
            }
        }
        steps = new_steps.intersection(&gardens).cloned().collect();
    }
    return steps.len();
}

fn main() {
    println!("{}", part1("day21_input.txt"));
}
