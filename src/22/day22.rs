use std::collections::{BTreeSet, HashMap};

pub struct Brick {
    pub x1: u32,
    pub x2: u32,
    pub y1: u32,
    pub y2: u32,
    pub z1: u32,
    pub z2: u32
}

fn pushed_height(index: usize,
                 dependencies: &Vec<Vec<usize>>,
                 bricks: &mut Vec<Brick>,
                 lowest_heights: &mut HashMap<usize, u32>) -> u32 {
    if let Some(&result) = lowest_heights.get(&index) {
        return result;
    }

    let lowest_z = if !dependencies[index].is_empty() {
        dependencies[index]
        .iter()
        .map(|&i| pushed_height(i, dependencies, bricks, lowest_heights))
        .reduce(|acc, x| if acc > x { acc } else { x })
        .unwrap()
        + 1
    } else {
        0
    };

    let height = bricks[index].z2 - bricks[index].z1;
    bricks[index].z1 = lowest_z;
    bricks[index].z2 = lowest_z + height;
    lowest_heights.insert(index, lowest_z + height);

    return lowest_z + height;
}

fn part1(filename: &str) -> usize {
    let lines = std::fs::read_to_string(filename).unwrap();
    let mut bricks = lines
        .trim()
        .split('\n')
        .map(|x| x.trim().split('~'))
        .map(
            |mut x| (
            x.next().unwrap().split(',').map(|x| x.parse::<u32>().unwrap()).collect::<Vec<_>>(),
            x.next().unwrap().split(',').map(|x| x.parse::<u32>().unwrap()).collect::<Vec<_>>()
            )
        )
        .map(
            |(p1, p2)|
            Brick {
                x1: std::cmp::min(p1[0], p2[0]),
                x2: std::cmp::max(p1[0], p2[0]),
                y1: std::cmp::min(p1[1], p2[1]),
                y2: std::cmp::max(p1[1], p2[1]),
                z1: std::cmp::min(p1[2], p2[2]),
                z2: std::cmp::max(p1[2], p2[2])
            }
        )
        .collect::<Vec<_>>();
    let mut dependencies = (0..bricks.len())
        .map(|_| Vec::<usize>::new())
        .collect::<Vec<_>>();
    for i in 0..bricks.len() {
        for j in 0..bricks.len() {
            if i == j { continue; }
            if bricks[j].z2 < bricks[i].z1
            && bricks[j].x2 >= bricks[i].x1 && bricks[j].x1 <= bricks[i].x2
            && bricks[j].y2 >= bricks[i].y1 && bricks[j].y1 <= bricks[i].y2 {
                dependencies[i].push(j);
            }
        }
    }
    let mut lowest_heights = HashMap::<usize, u32>::new();
    for i in 0..bricks.len() {
        pushed_height(i, &dependencies, &mut bricks, &mut lowest_heights);
    }
    dependencies = (0..bricks.len())
        .map(
            |i|
            dependencies[i]
            .iter()
            .filter(|&&dep| bricks[dep].z2 + 1 == bricks[i].z1)
            .cloned()
            .collect()
        )
        .collect();
    let mut single_deps = Vec::<usize>::new();
    for s in dependencies {
        if s.len() == 1 && !single_deps.contains(&s[0]) {
            single_deps.push(s[0]);
        }
    }
    return bricks.len() - single_deps.len();
}

fn part2(filename: &str) -> usize {
    let lines = std::fs::read_to_string(filename).unwrap();
    let mut bricks = lines
        .trim()
        .split('\n')
        .map(|x| x.trim().split('~'))
        .map(
            |mut x| (
            x.next().unwrap().split(',').map(|x| x.parse::<u32>().unwrap()).collect::<Vec<_>>(),
            x.next().unwrap().split(',').map(|x| x.parse::<u32>().unwrap()).collect::<Vec<_>>()
            )
        )
        .map(
            |(p1, p2)|
            Brick {
                x1: std::cmp::min(p1[0], p2[0]),
                x2: std::cmp::max(p1[0], p2[0]),
                y1: std::cmp::min(p1[1], p2[1]),
                y2: std::cmp::max(p1[1], p2[1]),
                z1: std::cmp::min(p1[2], p2[2]),
                z2: std::cmp::max(p1[2], p2[2])
            }
        )
        .collect::<Vec<_>>();
    let mut dependencies = (0..bricks.len())
        .map(|_| Vec::<usize>::new())
        .collect::<Vec<_>>();
    for i in 0..bricks.len() {
        for j in 0..bricks.len() {
            if i == j { continue; }
            if bricks[j].z2 < bricks[i].z1
            && bricks[j].x2 >= bricks[i].x1 && bricks[j].x1 <= bricks[i].x2
            && bricks[j].y2 >= bricks[i].y1 && bricks[j].y1 <= bricks[i].y2 {
                dependencies[i].push(j);
            }
        }
    }
    let mut lowest_heights = HashMap::<usize, u32>::new();
    for i in 0..bricks.len() {
        pushed_height(i, &dependencies, &mut bricks, &mut lowest_heights);
    }
    dependencies = (0..bricks.len())
        .map(
            |i|
            dependencies[i]
            .iter()
            .filter(|&&dep| bricks[dep].z2 + 1 == bricks[i].z1)
            .cloned()
            .collect()
        )
        .collect();
    let mut s = 0;
    for i in 0..bricks.len() {
        let mut falling = BTreeSet::<usize>::new();
        falling.insert(i);
        loop {
            let l = falling.len();
            for j in 0..bricks.len() {
                if !dependencies[j].is_empty() && dependencies[j].iter().all(|x| falling.contains(&x)) {
                    falling.insert(j);
                }
            }
            if falling.len() == l { break; }
        }
        s += falling.len() - 1;
    }
    return s;
}

fn main() {
    println!("{}", part1("day22_input.txt"));
    println!("{}", part2("day22_input.txt"));
}
