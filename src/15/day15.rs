pub struct Lens {
    pub label: String,
    pub focal_length: usize
}

fn hash(string: &str) -> usize {
    string.chars().fold(0, |acc, e| ((acc + (e as usize)) * 17) & 255)
}

fn part1(filename: &str) -> usize {
    return std::fs::read_to_string(filename).unwrap().trim().split(',').map(|string| hash(string)).sum();
}

fn part2(filename: &str) -> usize {
    let commands = std::fs::read_to_string(filename).unwrap();
    let operations = commands.trim().split(',').map(
        |string|
        if string.ends_with('-') {
            (&string[0..string.len()-1], 0)
        } else {
            let splitted = string.split('=').collect::<Vec<_>>();
            (splitted[0], splitted[1].parse::<usize>().unwrap())
        }
    )
    .collect::<Vec<_>>();
    let mut boxes: Vec<Vec<Lens>> = Vec::new();
    for _ in 0..256 { boxes.push(Vec::<Lens>::new()); }
    for (string, focal_length) in operations {
        let box_index = hash(&string);
        if focal_length == 0 {
            if let Some((i, _)) = boxes[box_index]
                .iter()
                .enumerate()
                .find(|(_, x)| x.label == string)
            {
                boxes[box_index].remove(i);
            }
        } else {
            if let Some((i, _)) = boxes[box_index]
                .iter()
                .enumerate()
                .find(|(_, x)| x.label == string)
            {
                boxes[box_index][i] = Lens { label: string.to_string(), focal_length };
            } else {
                boxes[box_index].push(Lens { label: string.to_string(), focal_length });
            }
        }
    }
    return (0..256)
        .map(
            |i|
            (i+1)*boxes[i].iter().enumerate().map(|(j, l)| (j+1)*l.focal_length).sum::<usize>()
        )
        .sum();
}

fn main() {
    println!("{}", part1("day15_input.txt"));
    println!("{}", part2("day15_input.txt"));
}
