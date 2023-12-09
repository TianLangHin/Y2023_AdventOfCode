fn get_input(filename: &str) -> Vec<Vec<i64>> {
    let lines = std::fs::read_to_string(filename).expect("File not found");
    let mut values: Vec<Vec<i64>> = Vec::new();
    for line in lines
        .trim()
        .split('\n')
        .map(|x|
            x.split_whitespace()
            .map(|v| v.parse::<i64>().unwrap())
            .collect::<Vec<_>>()
        ) {
        values.push(line);
    }
    return values;
}

fn get_layers(numbers: &Vec<i64>, part_two: bool) -> Vec<i64> {
    let mut layers: Vec<i64> = Vec::new();
    let mut differences: Vec<i64> = numbers.to_vec();
    while differences.iter().any(|&x| x != 0) {
        layers.push(if part_two { differences[0] } else { differences[differences.len()-1] });
        differences = (0..(differences.len()-1))
            .map(|i| differences[i+1] - differences[i])
            .collect::<Vec<_>>();
    }
    return layers;
}

fn part1(filename: &str) -> i64 {
    return get_input(filename)
        .iter()
        .map(|n| get_layers(n, false).iter().sum::<i64>())
        .sum::<i64>();
}

fn part2(filename: &str) -> i64 {
    return get_input(filename)
        .iter()
        .map(|n| get_layers(n, true).into_iter().rev().reduce(|acc, x| x - acc).unwrap())
        .reduce(|acc, x| acc + x).unwrap();
}

fn main() {
    println!("{}", part1("day9_input.txt"));
    println!("{}", part2("day9_input.txt"));
}