type Hailstone = ((i64, i64, i64), (i64, i64, i64));

fn has_intersection(h1: Hailstone, h2: Hailstone) -> Option<(i64, i64, i64)> {
    let ((px1, py1, _), (vx1, vy1, _)) = h1;
    let ((px2, py2, _), (vx2, vy2, _)) = h2;
    let (cx, cy) = (px2 - px1, py2 - py1);
    let k = vx1 * vy2 - vx2 * vy1;
    let t1 = (vy2 * cx - vx2 * cy) * if k < 0 { -1 } else { 1 };
    let t2 = (vy1 * cx - vx1 * cy) * if k < 0 { -1 } else { 1 };
    return if t1 < 0 || t2 < 0 { None } else { Some((if k < 0 { -k } else { k }, t1, t2)) };
}

fn part1(filename: &str) -> u64 {
    let file_text = std::fs::read_to_string(filename).unwrap();
    let hailstones = file_text
        .trim()
        .split('\n')
        .map(|h| h.trim().split("@").map(|x| x.trim()))
        .map(
            |mut h|
            (h.next().unwrap().split(",").map(|x| x.trim().parse::<i64>().unwrap()),
             h.next().unwrap().split(",").map(|x| x.trim().parse::<i64>().unwrap()))
        )
        .map(
            |(mut p, mut v)|
            ((p.next().unwrap(), p.next().unwrap(), p.next().unwrap()),
             (v.next().unwrap(), v.next().unwrap(), v.next().unwrap()))
        )
        .collect::<Vec<_>>();
    let lower: i64 = 200000000000000;
    let higher: i64 = 400000000000000;
    return (0..hailstones.len())
    .map(
        |i|
        (i+1..hailstones.len())
        .fold(
            0,
            |acc, j|
            if let Some((k, t1, _)) = has_intersection(hailstones[i], hailstones[j]) {
                if k != 0 {
                    let x = hailstones[i].0.0 + hailstones[i].1.0 * (t1 / k);
                    let y = hailstones[i].0.1 + hailstones[i].1.1 * (t1 / k);
                    if lower <= x && x <= higher && lower <= y && y <= higher {
                    acc + 1
                    } else {
                        acc
                    }
                } else {
                    acc
                }
            } else {
                acc
            }
        )
    )
    .fold(0, |acc, s| acc + s);
}

fn main() {
    println!("{}", part1("day24_input.txt"));
}