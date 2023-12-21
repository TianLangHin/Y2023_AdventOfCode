use std::collections::HashMap;

#[derive(Debug)]
pub struct Module {
    pub module_type: bool,
    pub dests: Vec<String>,
    pub state: bool,
    pub inputs: HashMap<String, bool>
}

#[derive(Debug)]
pub struct Send {
    pub origin: String,
    pub dest: String,
    pub pulse: bool
}

fn press_button(modules: &mut HashMap<String, Module>) -> (u64, u64) {
    let mut tasks = modules
        .get("broadcaster")
        .unwrap()
        .dests
        .iter()
        .map(|d| Send { origin: "broadcaster".to_string(), dest: d.to_string(), pulse: false })
        .collect::<Vec<_>>();
    let mut lows = (tasks.len() + 1) as u64;
    let mut highs = 0u64;
    while tasks.len() > 0 {
        let mut new_tasks = Vec::<Send>::new();
        for task in &tasks {
            let module_name = task.dest.clone();
            if let Some(module) = modules.get(&module_name) {
                if module.module_type {
                    if !task.pulse {
                        modules.insert(
                            module_name.clone(),
                            Module {
                                module_type: module.module_type,
                                dests: module.dests.iter().cloned().collect(),
                                state: !module.state,
                                inputs: HashMap::new()
                            }
                        );
                        let new_pulse = modules.get(&module_name).unwrap().state;
                        for dest in &modules.get(&module_name).unwrap().dests {
                            if new_pulse { highs += 1; } else { lows += 1; }
                            new_tasks.push(
                                Send {
                                    origin: module_name.clone(),
                                    dest: dest.clone(),
                                    pulse: new_pulse
                                }
                            );
                        }
                    }
                } else {
                    modules
                        .get_mut(&module_name)
                        .unwrap()
                        .inputs
                        .insert(task.origin.clone(), task.pulse);
                    let new_pulse = modules
                        .get(&module_name)
                        .unwrap()
                        .inputs
                        .values()
                        .any(|s| !s);
                    for dest in &modules.get(&module_name).unwrap().dests {
                        if new_pulse { highs += 1; } else { lows += 1; }
                        new_tasks.push(
                            Send {
                                origin: module_name.clone(),
                                dest: dest.clone(),
                                pulse: new_pulse
                            }
                        );
                    }
                }
            } else {
                continue;
            }
        }
        tasks = new_tasks;
    }
    return (lows, highs);
}

fn part1(filename: &str) -> u64 {
    let mut modules = HashMap::<String, Module>::new();
    let mut conjunctions = Vec::<String>::new();
    let lines = std::fs::read_to_string(filename).expect("File not found");
    for line in lines.trim().split('\n').map(|x| x.trim()) {
        if line.starts_with("broadcaster") {
            let dests = line.split(" -> ").collect::<Vec<_>>()[1];
            modules.insert(
                "broadcaster".to_string(),
                Module {
                    module_type: true,
                    dests: dests.trim().split(", ").map(|s| s.to_string()).collect::<Vec<_>>(),
                    state: false,
                    inputs: HashMap::<String, bool>::new()
                }
            );
        } else if line.starts_with('%') {
            let mut line_iter = line[1..].trim().split(" -> ");
            let name = line_iter.next().unwrap();
            let dests = line_iter.next().unwrap();
            modules.insert(
                name.to_string(),
                Module {
                    module_type: true,
                    dests: dests.trim().split(", ").map(|s| s.to_string()).collect::<Vec<_>>(),
                    state: false,
                    inputs: HashMap::<String, bool>::new()
                }
            );
        } else if line.starts_with('&') {
            let mut line_iter = line[1..].trim().split(" -> ");
            let name = line_iter.next().unwrap();
            let dests = line_iter.next().unwrap();
            modules.insert(
                name.to_string(),
                Module {
                    module_type: false,
                    dests: dests.trim().split(", ").map(|s| s.to_string()).collect::<Vec<_>>(),
                    state: false,
                    inputs: HashMap::<String, bool>::new()
                }
            );
            conjunctions.push(name.to_string());
        } else {
            break;
        }
    }

    let keys_and_dests = modules
        .iter()
        .map(|(k, v)| (k.clone(), v.dests.iter().cloned().collect::<Vec<_>>()))
        .collect::<Vec<_>>();
    for (module_name, dests) in keys_and_dests {
        for dest in dests {
            if conjunctions.contains(&dest) {
                modules.get_mut(&dest)
                    .unwrap()
                    .inputs
                    .insert(module_name.to_string(), false);
            }
        }
    }
    let mut low_total: u64 = 0;
    let mut high_total: u64 = 0;
    for _ in 0..1000 {
        let (low, high) = press_button(&mut modules);
        low_total += low;
        high_total += high;
    }
    return low_total * high_total;
}

fn main() {
    println!("{}", part1("day20_input.txt"));
}
