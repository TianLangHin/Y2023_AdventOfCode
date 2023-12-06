use std::fs;

pub struct Range {
    pub start: i64,
    pub end: i64,
}

pub struct Mapping {
    pub start_range: Range,
    pub offset: i64,
}

pub struct Row {
    pub end: i64,
    pub start: i64,
    pub length: i64,
}

fn transform(mut source: Vec<i64>, mapping: &Vec<Row>) -> Vec<i64> {
    let mut converted: Vec<bool> = Vec::with_capacity(source.len());
    for _ in 0..source.len() {
        converted.push(false);
    }
    for map in mapping {
        for i in 0..source.len() {
            if !converted[i]
                && map.start <= source[i]
                && source[i] < map.start + map.length
            {
                source[i] += map.end - map.start;
                converted[i] = true;
            }
        }
    }
    return source
}

fn part1(filename: &str) -> i64 {
    let mut seeds: Vec<i64> = Vec::new();
    let mut transformations = vec![
        Vec::<Row>::new(), // sentinel
        Vec::<Row>::new(), // seed-to-soil
        Vec::<Row>::new(), // soil-to-fertilizer
        Vec::<Row>::new(), // fertilizer-to-water
        Vec::<Row>::new(), // water-to-light
        Vec::<Row>::new(), // light-to-temperature
        Vec::<Row>::new(), // temperature-to-humidity
        Vec::<Row>::new(), // humidity-to-location
    ];
    let mut i: usize = 0;
    let lines = fs::read_to_string(filename).expect("File not found");
    for file_line in lines.split('\n').map(|x| x.trim()) {
        match file_line {
            "" => {},
            line if line.starts_with("seeds:") => {
                seeds.extend(
                    line[7..]
                    .split_whitespace()
                    .map(|x| x.parse::<i64>().unwrap())
                );
            },
            line if line.ends_with("map:") => {
                i += 1;
            },
            line => {
                let splitted_line: Vec<_> = line
                    .split_whitespace()
                    .map(|x| x.parse::<i64>().unwrap())
                    .collect();
                transformations[i].push(
                    Row {
                        end: splitted_line[0],
                        start: splitted_line[1], 
                        length: splitted_line[2]
                    }
                );
            }
        }
    }
    for t in 1..8 {
        seeds = transform(seeds, &transformations[t]);
    }
    return *seeds.iter().min().unwrap();
}

fn transform_ranges(seeds: &Vec<Range>, mappers: &Vec<Mapping>) -> Vec<Range> {
    let mut top_item: Option<Range> = None;

    let mut result_ranges: Vec<Range> = Vec::new();
    let mut seeds_iter = seeds.iter();
    let mut mappers_iter = mappers.iter();

    let mut current_seed = seeds_iter.next();
    let mut current_map = mappers_iter.next();

    while let Some((seed, map)) = current_seed.zip(current_map) {
        match top_item {
            None => {top_item = Some(Range { start: seed.start, end: seed.end })}
            _ => {}
        }
        let b = Range { start: map.start_range.start, end: map.start_range.end };
        let offset = map.offset;
        match top_item {
            Some(ref t) => {
                if t.start < b.start {
                    if b.end < t.end {
                        result_ranges.push(Range { start: t.start, end: b.start - 1 });
                        result_ranges.push(Range { start: b.start + offset, end: b.end + offset });
                        top_item = Some(Range { start: b.end + 1, end: t.end });
                        current_map = mappers_iter.next();
                    } else if b.start <= t.end && t.end <= b.end {
                        result_ranges.push(Range { start: t.start, end: b.start - 1 });
                        result_ranges.push(Range { start: b.start + offset, end: t.end + offset });
                        top_item = None;
                        current_seed = seeds_iter.next();
                    } else if t.end < b.start {
                        result_ranges.push(Range { start: t.start, end: t.end });
                        top_item = None;
                        current_seed = seeds_iter.next();
                    }
                } else if b.start <= t.start {
                    if b.end < t.start {
                        top_item = None;
                        current_map = mappers_iter.next();
                    } else if b.end < t.end {
                        result_ranges.push(Range { start: t.start + offset, end: b.end + offset });
                        top_item = Some(Range { start: b.end + 1, end: t.end });
                        current_map = mappers_iter.next();
                    } else if t.end <= b.end {
                        result_ranges.push(Range { start: t.start + offset, end: t.end + offset });
                        top_item = None;
                        current_seed = seeds_iter.next();
                    }
                }
            },
            None => {}
        }
    }
    while let Some(seed) = current_seed {
        result_ranges.push(Range { start: seed.start, end: seed.end });
        current_seed = seeds_iter.next();
    }
    result_ranges.sort_by(|r1, r2| r1.start.partial_cmp(&r2.start).unwrap());
    return result_ranges;
}

fn part2(filename: &str) -> i64 {
    let mut seeds: Vec<Range> = Vec::new();
    let mut transformations = vec![
        Vec::<Mapping>::new(), // sentinel
        Vec::<Mapping>::new(), // seed-to-soil
        Vec::<Mapping>::new(), // soil-to-fertilizer
        Vec::<Mapping>::new(), // fertilizer-to-water
        Vec::<Mapping>::new(), // water-to-light
        Vec::<Mapping>::new(), // light-to-temperature
        Vec::<Mapping>::new(), // temperature-to-humidity
        Vec::<Mapping>::new(), // humidity-to-location
    ];
    let mut i: usize = 0;
    let lines = fs::read_to_string(filename).expect("File not found");
    for file_line in lines.split('\n').map(|x| x.trim()) {
        match file_line {
            "" => {},
            line if line.starts_with("seeds:") => {
                let split: Vec<_> = line[7..]
                    .split_whitespace()
                    .map(|x| x.parse::<i64>().unwrap())
                    .collect();
                for i in 0..split.len() >> 1 {
                    seeds.push(
                        Range {
                            start: split[i << 1],
                            end: split[i << 1] + split[(i << 1) + 1] - 1
                        }
                    );
                }
            },
            line if line.ends_with("map:") => {
                i += 1;
            },
            line => {
                let integers: Vec<_> = line
                    .split_whitespace()
                    .map(|x| x.parse::<i64>().unwrap())
                    .collect();
                let end = integers[0];
                let start = integers[1];
                let length = integers[2];
                transformations[i].push(
                    Mapping {
                        start_range: Range {
                            start: start,
                            end: start + length - 1
                        },
                        offset: end-start
                    }
                );
                transformations[i].sort_by(|r1, r2|
                    r1
                    .start_range
                    .start
                    .partial_cmp(&r2.start_range.start)
                    .unwrap()
                );
            }
        }
    }
    seeds.sort_by(|r1, r2| r1.start.partial_cmp(&r2.start).unwrap());
    for t in 1..8 {
        seeds = transform_ranges(&seeds, &transformations[t]);
    }
    return seeds[0].start;
}

fn main() {
    println!("{}", part1("day5_input.txt"));
    println!("{}", part2("day5_input.txt"));
}