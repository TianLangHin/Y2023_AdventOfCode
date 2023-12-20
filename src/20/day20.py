from collections import namedtuple
from typing import Optional

# type: bool, dests: list[str], state: bool, inputs: Optional[dict[str, bool]]
Module = namedtuple('Module', ['type', 'dests', 'state', 'inputs'])
# NOTE: broadcaster is internally represented as a flip-flop, but treated differently
# True is flip-flop, False is conjunction.
# state: True is high, False is low.
# `inputs` is only valid for conjunction (i.e. dests=False)

# origin: str, dest: str, pulse: bool
Send = namedtuple('Send', ['origin', 'dest', 'pulse'])
# pulse: True is high, False is low.

# This procedure edits `modules` as a mutable reference
# It returns the number of low and high signals sent.
def press_button(modules: dict[str, Module]) -> tuple[int, int]:
    tasks = [Send('broadcaster', d, False) for d in modules['broadcaster'].dests]
    lows, highs = len(tasks) + 1, 0
    while tasks:
        new_tasks = []
        for task in tasks:
            module_name = task.dest
            if modules.get(module_name, None) is None:
                continue
            if modules[module_name].type:
                if not task.pulse:
                    modules[module_name] = Module(
                        modules[module_name].type,
                        modules[module_name].dests,
                        not modules[module_name].state,
                        None
                    )
                    new_pulse = modules[module_name].state
                    for dest in modules[module_name].dests:
                        if new_pulse:
                            highs += 1
                        else:
                            lows += 1
                        new_tasks.append(Send(module_name, dest, new_pulse))
            else:
                modules[module_name].inputs[task.origin] = task.pulse
                new_pulse = any(not signal for signal in modules[module_name].inputs.values())
                for dest in modules[module_name].dests:
                    if new_pulse:
                        highs += 1
                    else:
                        lows += 1
                    new_tasks.append(Send(module_name, dest, new_pulse))
        tasks = new_tasks
    return lows, highs

def part1(filename: str) -> int:
    modules = dict()
    conjunctions = []
    with open(filename, 'rt') as f:
        for line in f:
            if line.startswith('broadcaster'):
                _, dests = line.split(' -> ')
                modules['broadcaster'] = Module(True, dests.strip().split(', '), False, None)
            elif line.startswith('%'):
                name, dests = line.lstrip('%').strip().split(' -> ')
                modules[name] = Module(True, dests.strip().split(', '), False, None)
            elif line.startswith('&'):
                name, dests = line.lstrip('&').strip().split(' -> ')
                modules[name] = Module(False, dests.strip().split(', '), False, dict())
                conjunctions.append(name)
            else:
                break
    for module in modules:
        for dest in modules[module].dests:
            if dest in conjunctions:
                modules[dest].inputs[module] = False

    low_total, high_total = 0, 0
    for _ in range(1000):
        low, high = press_button(modules)
        low_total += low
        high_total += high
    return low_total * high_total

if __name__ == '__main__':
    print(part1('day20_input.txt'))