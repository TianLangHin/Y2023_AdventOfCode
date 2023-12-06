use std::fs;

fn part1(filename: &str) -> i64 {
    let file_lines = fs::read_to_string(filename).expect("File not found");
    let mut lines = file_lines.split('\n').map(
        |line|
        line
        .split(':')
        .collect::<Vec<&str>>()[1]
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<f64>().unwrap())
    );
    let time = lines.next().unwrap();
    let distance = lines.next().unwrap();
    let mut s: i64 = 1;
    for (t, d) in time.zip(distance) {
        let h: f64 = (t as f64) / 2.0;
        let v: f64 = ((t*t - 4.0*d) as f64) / 4.0;
        if v > 0.0 {
            let l = h - v.sqrt();
            let u = h + v.sqrt();
            let l: i64 = if l == l.round() {l.ceil() as i64 + 1} else {l.ceil() as i64};
            let u: i64 = if u == u.round() {u.floor() as i64 - 1} else {u.floor() as i64};
            if u >= l {
                s *= u - l + 1;
            }
        }
    }
    return s;
}

fn part2(filename: &str) -> i64 {
    let file_lines = fs::read_to_string(filename).expect("File not found");
    let mut lines = file_lines.split('\n').map(
        |line|
        line
        .split(':')
        .collect::<Vec<&str>>()[1]
        .trim()
        .replace(" ", "")
        .parse::<f64>()
        .unwrap()
    );
    let t = lines.next().unwrap();
    let d = lines.next().unwrap();
    let h: f64 = (t as f64) / 2.0;
    let v: f64 = ((t*t - 4.0*d) as f64) / 4.0;
    if v > 0.0 {
        let l = h - v.sqrt();
        let u = h + v.sqrt();
        let l: i64 = if l == l.round() {l.ceil() as i64 + 1} else {l.ceil() as i64};
        let u: i64 = if u == u.round() {u.floor() as i64 - 1} else {u.floor() as i64};
        if u >= l {
            return u - l + 1;
        }
    }
    return 0;
}

fn main() {
    println!("{}", part1("day6_input.txt"));
    println!("{}", part2("day6_input.txt"));
}
