use std::collections::HashMap;

struct Part(u64, u64, u64, u64);

struct Condition(usize, u64, bool, String);

struct Workflow(Vec<Condition>, String);

#[derive(Clone, Copy)]
struct Span(u64, u64);

struct Accept(Vec<Span>, Vec<Span>, Vec<Span>, Vec<Span>);

fn execute_steps(workflows: &HashMap<String, Workflow>, part: &Part) -> bool {
    let mut current_workflow = workflows.get("in");
    while let Some(current) = current_workflow {
        let mut redirect: Option<String> = None;
        let mut component: u64;
        for step in &current.0 {
            component = match step.0 {
                0 => part.0,
                1 => part.1,
                2 => part.2,
                3 => part.3,
                _ => panic!()
            };
            if if step.2 { component < step.1 } else { component > step.1 } {
                redirect = Some(step.3.clone());
                break;
            }
        }
        if Option::is_none(&redirect) {
            redirect = Some(current.1.clone());
        }
        if let Some(path) = redirect {
            if path == "A" { return true; }
            if path == "R" { return false; }
            current_workflow = workflows.get(&path);
        }
    }
    panic!();
}

fn part1(filename: &str) -> u64 {
    let mut workflows = HashMap::<String, Workflow>::new();
    let mut parts = Vec::<Part>::new();
    let mut blank_passed = false;
    let file_text = std::fs::read_to_string(filename).unwrap();
    for line in file_text.trim().split('\n').map(|x| x.trim()) {
        if line.len() > 0 {
            if blank_passed {
                let components = line[1..line.len()-1]
                    .split(',')
                    .map(
                        |x|
                        x
                        .split('=')
                        .collect::<Vec<_>>()[1]
                        .parse::<u64>()
                        .unwrap()
                    )
                    .collect::<Vec<_>>();
                parts.push(Part(components[0], components[1], components[2], components[3]));
            } else {
                let splitted = line.split('{').collect::<Vec<_>>();
                let name = splitted[0];
                let mut body = splitted[1][0..splitted[1].len()-1].split(',').collect::<Vec<_>>();
                let final_branch = body.pop().unwrap();
                let mut conditions = Vec::<Condition>::new();
                for body_line in body {
                    let mut clause = body_line.split(':');
                    let cmd = clause.next().unwrap();
                    let dest = clause.next().unwrap();
                    let mut operands = cmd.split(if cmd.contains("<") { '<' } else { '>' });
                    let component = match operands.next().unwrap() {
                        "x" => 0,
                        "m" => 1,
                        "a" => 2,
                        "s" => 3,
                        _ => panic!()
                    };
                    let value = operands.next().unwrap().parse::<u64>().unwrap();
                    conditions.push(Condition(component, value, cmd.contains("<"), dest.to_string()));
                }
                workflows.insert(name.to_string(), Workflow(conditions, final_branch.to_string()));
            }
        } else {
            blank_passed = true;
        }
    }
    let mut s: u64 = 0;
    for part in parts {
        if execute_steps(&workflows, &part) {
            s += part.0 + part.1 + part.2 + part.3;
        }
    }
    return s;
}

fn intersect(a1: &Accept, a2: &Accept) -> Accept {
    let mut new_ranges = Vec::<Vec<Span>>::new();
    let old_ranges = vec![(&a1.0, &a2.0), (&a1.1, &a2.1), (&a1.2, &a2.2), (&a1.3, &a2.3)];
    for (&ref left, &ref right) in old_ranges {
        let mut this_new_range = Vec::<Span>::new();
        let (mut i, mut j) = (0, 0);
        let (m, n) = (left.len(), right.len());
        while i < m && j < n {
            if left[i].1 <= right[j].0 {
                i += 1;
            } else if left[i].0 >= right[j].1 {
                j += 1;
            } else {
                let lower = std::cmp::max(left[i].0, right[j].0);
                let upper = std::cmp::min(left[i].1, right[j].1);
                this_new_range.push(Span(lower, upper));
                if left[i].0 < right[j].0 {
                    i += 1;
                } else {
                    j += 1;
                }
            }
        }
        new_ranges.push(this_new_range);
    }
    return Accept(
        new_ranges[0].clone(),
        new_ranges[1].clone(),
        new_ranges[2].clone(),
        new_ranges[3].clone()
    );
}

fn unroll_workflow(name: &str, workflows: &HashMap<String, Workflow>) -> Vec<Accept> {
    let mut branches = Vec::<Accept>::new();
    let mut branch_else = Accept(
        vec![Span(0, 4001)],
        vec![Span(0, 4001)],
        vec![Span(0, 4001)],
        vec![Span(0, 4001)]
    );
    let workflow = workflows.get(name).unwrap();
    for c in &workflow.0 {
        let this_span = if c.2 { Span(0, c.1) } else { Span(c.1, 4001) };
        let neg_this_span = if c.2 { Span(c.1-1, 4001) } else { Span(0, c.1+1) };

        let this_accept = intersect(&branch_else, &Accept(
            vec![if c.0 == 0 { this_span } else { Span(0, 4001) }],
            vec![if c.0 == 1 { this_span } else { Span(0, 4001) }],
            vec![if c.0 == 2 { this_span } else { Span(0, 4001) }],
            vec![if c.0 == 3 { this_span } else { Span(0, 4001) }],
        ));
        let neg_this_accept = Accept(
            vec![if c.0 == 0 { neg_this_span } else { Span(0, 4001) }],
            vec![if c.0 == 1 { neg_this_span } else { Span(0, 4001) }],
            vec![if c.0 == 2 { neg_this_span } else { Span(0, 4001) }],
            vec![if c.0 == 3 { neg_this_span } else { Span(0, 4001) }],
        );

        if c.3 == "A" {
            branches.push(this_accept);
        } else if c.3 != "R" {
            branches.extend(
                unroll_workflow(&c.3, &workflows)
                .iter()
                .map(|branch| intersect(&this_accept, &branch))
            );
        }
        branch_else = intersect(&branch_else, &neg_this_accept);
    }
    if workflow.1 == "A" {
        branches.push(branch_else);
    } else if workflow.1 != "R" {
        branches.extend(
            unroll_workflow(&workflow.1, &workflows)
            .iter()
            .map(|branch| intersect(&branch_else, &branch))
        );
    }
    return branches;
}

fn part2(filename: &str) -> u64 {
    let mut workflows = HashMap::<String, Workflow>::new();
    let file_text = std::fs::read_to_string(filename).unwrap();
    for line in file_text.trim().split('\n').map(|x| x.trim()) {
        if line.len() > 0 {
            let splitted = line.split('{').collect::<Vec<_>>();
            let name = splitted[0];
            let mut body = splitted[1][0..splitted[1].len()-1].split(',').collect::<Vec<_>>();
            let final_branch = body.pop().unwrap();
            let mut conditions = Vec::<Condition>::new();
            for body_line in body {
                let mut clause = body_line.split(':');
                let cmd = clause.next().unwrap();
                let dest = clause.next().unwrap();
                let mut operands = cmd.split(if cmd.contains("<") { '<' } else { '>' });
                let component = match operands.next().unwrap() {
                    "x" => 0,
                    "m" => 1,
                    "a" => 2,
                    "s" => 3,
                    _ => panic!()
                };
                let value = operands.next().unwrap().parse::<u64>().unwrap();
                conditions.push(Condition(component, value, cmd.contains("<"), dest.to_string()));
            }
            workflows.insert(name.to_string(), Workflow(conditions, final_branch.to_string()));
        } else {
            break;
        }
    }
    let mut s: u64 = 0;
    for acceptance in unroll_workflow("in", &workflows) {
        s += acceptance.0.iter().map(|span| span.1 - span.0 - 1).sum::<u64>()
            * acceptance.1.iter().map(|span| span.1 - span.0 - 1).sum::<u64>()
            * acceptance.2.iter().map(|span| span.1 - span.0 - 1).sum::<u64>()
            * acceptance.3.iter().map(|span| span.1 - span.0 - 1).sum::<u64>();
    }
    return s;
}

fn main() {
    println!("{}", part1("day19_input.txt"));
    println!("{}", part2("day19_input.txt"));
}
