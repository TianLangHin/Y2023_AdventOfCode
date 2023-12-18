use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug)]
pub enum Mirror {
    Empty,
    Vertical,
    Horizontal,
    UpRight,
    UpLeft
}

#[derive(Clone, Copy, Debug, PartialOrd, Ord, PartialEq, Eq)]
pub enum Direction {
    Right,
    Left,
    Down,
    Up
}

type StepResult = (usize, Option<Direction>, bool);

fn point_range(start: i64, stop: i64, step: i64) -> Vec<usize> {
    let mut points: Vec<usize> = Vec::new();
    let mut index = start;
    if step < 0 {
        while index > stop {
            points.push(index as usize);
            index += step;
        }
    } else {
        while index < stop {
            points.push(index as usize);
            index += step;
        }
    }
    return points;
}

fn direction_offset(direction: Direction, x_bound: usize) -> i64 {
    match direction {
        Direction::Left  => -1,
        Direction::Up    => -(x_bound as i64),
        Direction::Right => 1,
        Direction::Down  => x_bound as i64
    }
}

fn reverse(direction: Direction) -> Direction {
    match direction {
        Direction::Left  => Direction::Right,
        Direction::Up    => Direction::Down,
        Direction::Right => Direction::Left,
        Direction::Down  => Direction::Up
    }
}

fn against_up_right(direction: Direction) -> Direction {
    match direction {
        Direction::Left  => Direction::Down,
        Direction::Up    => Direction::Right,
        Direction::Right => Direction::Up,
        Direction::Down  => Direction::Left
    }
}

fn against_up_left(direction: Direction) -> Direction {
    match direction {
        Direction::Left  => Direction::Up,
        Direction::Up    => Direction::Left,
        Direction::Right => Direction::Down,
        Direction::Down  => Direction::Right
    }
}

fn step(index: usize, grid: &Vec<Mirror>, x_bound: usize, direction: Direction) -> StepResult {
    let travelling_x = match direction {
        Direction::Right | Direction::Left => true,
        _ => false
    };
    let grid_bound = match direction {
        Direction::Left  => ((index / x_bound) * x_bound) as i64 - 1,
        Direction::Up    => -1,
        Direction::Right => ((index / x_bound + 1) * x_bound) as i64,
        Direction::Down  => grid.len() as i64
    };
    let increment = direction_offset(direction, x_bound);
    let mut j = index as i64;
    for i in point_range(j + increment, grid_bound, increment) {
        j = i as i64;
        match grid[i as usize] {
            Mirror::Vertical if travelling_x => return (i, Some(Direction::Down), true),
            Mirror::Horizontal if !travelling_x => return (i, Some(Direction::Right), true),
            Mirror::UpRight => return (i, Some(against_up_right(direction)), false),
            Mirror::UpLeft => return (i, Some(against_up_left(direction)), false),
            _ => {}
        }
    }
    return (j as usize, None, false);
}

fn energy(start_index: usize, starting_direction: Direction, grid: &Vec<Mirror>, x_bound: usize) -> usize {
    let mut traversed_points = BTreeSet::<usize>::new();
    let mut traversed_directions = BTreeSet::<(usize, Direction)>::new();
    let mut paths = BTreeSet::<(usize, Direction)>::new();
    paths.insert((start_index, starting_direction));
    while paths.len() > 0 {
        let mut new_paths = BTreeSet::<(usize, Direction)>::new();
        for &(index, direction) in &paths {
            let step_result = step(index, &grid, x_bound, direction);
            match step_result.1 {
                Some(d) => {
                    if step_result.2 {
                        if !traversed_directions.contains(&(step_result.0, d)) {
                            new_paths.insert((step_result.0, d));
                            traversed_directions.insert((step_result.0, d));
                        }
                        if !traversed_directions.contains(&(step_result.0, reverse(d))) {
                            new_paths.insert((step_result.0, reverse(d)));
                            traversed_directions.insert((step_result.0, reverse(d)));
                        }
                    } else {
                        if !traversed_directions.contains(&(step_result.0, d)) {
                            new_paths.insert((step_result.0, d));
                        }
                    }
                },
                None => {}
            }
            for i in point_range(
                index as i64,
                step_result.0 as i64 + direction_offset(direction, x_bound),
                direction_offset(direction, x_bound))
            {
                traversed_points.insert(i);
            }
        }
        paths = new_paths;
    }
    return traversed_points.len();
}

fn part1(filename: &str) -> usize {
    let file_lines = std::fs::read_to_string(filename).expect("File not found");
    let lines = file_lines.trim().split('\n').map(|x| x.trim()).collect::<Vec<_>>();
    let x_bound: usize = lines[0].len();
    let mut grid: Vec<Mirror> = Vec::new();
    for line in lines {
        grid.extend(line.chars().map(
            |tile|
            match tile {
                '|' => Mirror::Vertical,
                '-' => Mirror::Horizontal,
                '/' => Mirror::UpRight,
                '\\' => Mirror::UpLeft,
                _ => Mirror::Empty
            }
        ));
    }
    let starting_direction = match grid[0] {
        Mirror::Empty | Mirror::Horizontal => Direction::Right,
        Mirror::Vertical | Mirror::UpLeft => Direction::Down,
        Mirror::UpRight => Direction::Up
    };
    return energy(0, starting_direction, &grid, x_bound);
}

fn all_directions(index: usize, x_bound: usize, y_bound: usize, mirror: Mirror) -> Vec<Direction> {
    let mut directions: Vec<Direction> = Vec::new();
    if index % x_bound == 0 {
        match mirror {
            Mirror::Empty | Mirror::Horizontal => directions.push(Direction::Right),
            Mirror::Vertical => {
                directions.push(Direction::Up);
                directions.push(Direction::Down);
            },
            Mirror::UpRight => directions.push(Direction::Up),
            Mirror::UpLeft => directions.push(Direction::Down)
        }
    } else if index % x_bound == x_bound - 1 {
        match mirror {
            Mirror::Empty | Mirror::Horizontal => directions.push(Direction::Left),
            Mirror::Vertical => {
                directions.push(Direction::Up);
                directions.push(Direction::Down);
            },
            Mirror::UpRight => directions.push(Direction::Down),
            Mirror::UpLeft => directions.push(Direction::Up)
        }
    }
    if index / x_bound == 0 {
        match mirror {
            Mirror::Empty | Mirror::Horizontal => directions.push(Direction::Down),
            Mirror::Vertical => {
                directions.push(Direction::Left);
                directions.push(Direction::Right);
            },
            Mirror::UpRight => directions.push(Direction::Left),
            Mirror::UpLeft => directions.push(Direction::Right)
        }
    } else if index / x_bound == y_bound - 1 {
        match mirror {
            Mirror::Empty | Mirror::Horizontal => directions.push(Direction::Up),
            Mirror::Vertical => {
                directions.push(Direction::Left);
                directions.push(Direction::Right)
            },
            Mirror::UpRight => directions.push(Direction::Right),
            Mirror::UpLeft => directions.push(Direction::Left)
        }
    }
    return directions;
}

fn part2(filename: &str) -> usize {
    let file_lines = std::fs::read_to_string(filename).expect("File not found");
    let lines = file_lines.trim().split('\n').map(|x| x.trim()).collect::<Vec<_>>();
    let x_bound: usize = lines[0].len();
    let y_bound: usize = lines.len();
    let mut grid: Vec<Mirror> = Vec::new();
    for line in lines {
        grid.extend(line.chars().map(
            |tile|
            match tile {
                '|' => Mirror::Vertical,
                '-' => Mirror::Horizontal,
                '/' => Mirror::UpRight,
                '\\' => Mirror::UpLeft,
                _ => Mirror::Empty
            }
        ));
    }
    let edges: BTreeSet<usize> = point_range(0, x_bound as i64, 1).into_iter()
        .chain(point_range(0, grid.len() as i64, x_bound as i64).into_iter())
        .chain(point_range(x_bound as i64 - 1, grid.len() as i64, x_bound as i64).into_iter())
        .chain(point_range((grid.len() - x_bound) as i64, grid.len() as i64, 1).into_iter())
        .collect();
    return edges
        .iter()
        .map(
            |&edge|
            all_directions(edge, x_bound, y_bound, grid[edge])
            .iter()
            .map(
                |&direction|
                energy(edge, direction, &grid, x_bound)
            )
            .fold(0, |acc, x| if acc > x { acc } else { x })
        )
        .fold(0, |acc, x| if acc > x { acc } else { x });
}

fn main() {
    println!("{}", part1("day16_input.txt"));
    println!("{}", part2("day16_input.txt"));
}
