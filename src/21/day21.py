def make_steps(index: int, x_bound: int, y_bound: int) -> set[int]:
    steps = set()
    if index // x_bound > 0:
        steps.add(index - x_bound)
    if index // x_bound < y_bound - 1:
        steps.add(index + x_bound)
    if index % x_bound > 0:
        steps.add(index - 1)
    if index % x_bound < x_bound - 1:
        steps.add(index + 1)
    return steps

def part1(filename: str) -> int:
    with open(filename, 'rt') as f:
        text = [x.strip() for x in f.readlines()]
    x_bound = len(text[0])
    y_bound = len(text)
    gardens = set()
    exclusions = set()
    start = 0
    i = 0
    for line in text:
        for character in line:
            match character:
                case '.':
                    gardens.add(i)
                case '#':
                    exclusions.add(i)
                case 'S':
                    start = i
                    gardens.add(i)
            i += 1
    steps = {start}
    for _ in range(64):
        new_steps = set()
        for step in steps:
            new_steps |= make_steps(step, x_bound, y_bound)
        steps = new_steps & gardens
    return len(steps)

if __name__ == '__main__':
    print(part1('day21_input.txt'))