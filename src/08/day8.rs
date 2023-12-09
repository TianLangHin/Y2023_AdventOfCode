use std::collections::HashMap;

pub struct Node {
    pub name: String,
    pub left: String,
    pub right: String,
}

fn execute_steps<'a>(
start_node: &'a Node, path: &'a Vec<bool>, nodes: &'a HashMap<String, Node>) -> &'a Node {
    let mut node = start_node;
    for branch in path {
        node = (if *branch { nodes.get(&node.left) } else { nodes.get(&node.right) }).unwrap();
    }
    return node;
}

fn parse_input(filename: &str) -> (Vec<bool>, HashMap<String, Node>) {
    let mut nodes: HashMap<String, Node> = HashMap::new();

    let lines = std::fs::read_to_string(filename).expect("File not found");
    let mut line_iterator = lines.split('\n').map(|line| line.trim());
    let steps = line_iterator.next().unwrap().chars().map(|c| c == 'L').collect::<Vec<_>>();
    let _ = line_iterator.next();
    for (cur, left, right) in line_iterator
        .filter_map(|l| if l.len() > 0 { Some(l.split(" = ").collect::<Vec<_>>()) } else { None })
        .map(|x| (x[0], x[1][1..x[1].len()-1].split(", ").collect::<Vec<_>>()))
        .map(|(x, y)| (x, y[0], y[1])) {
        nodes.insert(
            cur.to_owned(),
            Node {
                name: cur.to_owned(),
                left: left.to_owned(),
                right: right.to_owned()
            }
        );
    }
    return (steps, nodes);
}

fn part1(filename: &str) -> i64 {
    let (steps, nodes) = parse_input(filename);
    let mut s: i64 = 0;
    let mut node = nodes.get(&"AAA".to_string()).unwrap();
    while node.name != "ZZZ" {
        node = execute_steps(&node, &steps, &nodes);
        s += 1;
    }
    return s * (steps.len() as i64);
}

fn overlap_cycle(arg1: (i64, i64), arg2: (i64, i64)) -> Option<(i64, i64)> {
    let (mut p, mut a) = arg1;
    let (mut q, mut b) = arg2;
    if a > b {
        (a, b, p, q) = (b, a, q, p);
    }
    let (mut old_r, mut r) = (a, b);
    let (mut old_s, mut s) = (1, 0);
    while r > 0 {
        let (quotient, remainder) = (old_r / r, old_r % r);
        (old_r, r) = (r, remainder);
        (old_s, s) = (s, old_s - quotient * s);
    }
    let lcm = a * b / old_r;
    if q < p {
        q += lcm;
    }
    let (factor, mismatch) = ((q - p) / old_r, (q - p) % old_r);
    if mismatch != 0 {
        return None;
    }
    old_s = (old_s * factor) % b;
    if old_s <= 0 {
        old_s += b;
    }
    return Some((p + a * old_s, lcm));
}

fn part2(filename: &str) -> i64 {
    let (steps, nodes) = parse_input(filename);
    let mut transforms: HashMap<&str, &str> = HashMap::new();
    for node in nodes.keys() {
        transforms.insert(node, &execute_steps(&nodes.get(node).unwrap(), &steps, &nodes).name);
    }
    let mut end_cycles: HashMap<&str, Vec<&str>> = HashMap::new();
    for node in nodes.keys() {
        if node.ends_with("Z") {
            let init_node: &str = node;
            let mut this_node: &str = node;
            let mut cycle: Vec<&str> = Vec::new();
            cycle.push(this_node);
            loop {
                this_node = transforms.get(this_node).unwrap();
                if this_node == init_node {
                    break;
                }
                cycle.push(this_node);
            }
            end_cycles.insert(this_node, cycle);
        }
    }

    let mut guarantees: HashMap<&str, (i64, i64)> = HashMap::new();
    for end_node in end_cycles.keys() {
        let e = end_cycles.get(end_node).unwrap();
        for i in 0..e.len() {
            match guarantees.get(e[i]) {
                None => guarantees.insert(e[i], (i as i64, e.len() as i64)),
                _ => None
            };
        }
    }

    let mut results: Vec<(i64, i64)> = Vec::new();
    for node in nodes.keys() {
        if node.ends_with("A") {
            let mut s: i64 = 0;
            let mut this_node: &str = node;
            let mut x = guarantees.get(this_node);
            while Option::is_none(&x) {
                this_node = transforms.get(this_node).unwrap();
                s += 1;
                x = guarantees.get(this_node);
            }
            results.push(((s + x.unwrap().1 - x.unwrap().0) % x.unwrap().1, x.unwrap().1));
        }
    }
    return steps.len() as i64 * results.iter().fold((0, 1), |acc, &x| overlap_cycle(acc, x).unwrap()).0;
}

fn main() {
    println!("{}", part1("day8_input.txt"));
    println!("{}", part2("day8_input.txt"));
}