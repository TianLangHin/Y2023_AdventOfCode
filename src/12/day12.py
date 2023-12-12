ROW_LENGTH = 32
MEMO: dict[int, int] = {}
SPRINGS: str = ''
POPULATIONS: list[int] = []

def func(i: int, j: int, sol: list[int]) -> int:
    global layers
    layers += 1
    if j == len(POPULATIONS):
        if '#' not in SPRINGS[i:]:
            MEMO[(i << ROW_LENGTH) | j] = 1
            return 1
        else:
            return 0
    elif i >= len(SPRINGS):
        return 0
    cached_result = MEMO.get((i << ROW_LENGTH) | j, None)
    if cached_result is not None:
        return cached_result
    match SPRINGS[i]:
        case '.':
            result = func(i+1, j, sol)
            MEMO[(i << ROW_LENGTH) | j] = result
            return result
        case '?':
            possibilities = func(i+1, j, sol)
            substring = SPRINGS[i : i + POPULATIONS[j]]
            if (not any(k == '.' for k in substring)) and len(substring) == POPULATIONS[j]:
                if not (i + POPULATIONS[j] < len(SPRINGS) and SPRINGS[i + POPULATIONS[j]] == '#'):
                    possibilities += func(i + POPULATIONS[j] + 1, j + 1, sol + [i])
            MEMO[(i << ROW_LENGTH) | j] = possibilities
            return possibilities
        case '#':
            substring = SPRINGS[i : i + POPULATIONS[j]]
            if (not any(k == '.' for k in substring)) and len(substring) == POPULATIONS[j]:
                possibilities = 0
                if not (i + POPULATIONS[j] < len(SPRINGS) and SPRINGS[i + POPULATIONS[j]] == '#'):
                    possibilities += func(i + POPULATIONS[j] + 1, j + 1, sol + [i])
                MEMO[(i << ROW_LENGTH) | j] = possibilities
                return possibilities
            else:
                MEMO[(i << ROW_LENGTH) | j] = 0
                return 0

def part1(filename: str) -> int:
    global SPRINGS, POPULATIONS, layers
    s = 0
    with open(filename, 'rt') as f:
        for line in f:
            line = line.strip()
            springs, pops = line.split()
            populations = [int(x) for x in pops.split(',')]
            SPRINGS = springs
            POPULATIONS = populations
            MEMO.clear()
            layers = 0
            result = func(0, 0, [])
            s += result
    return s

def part2(filename: str) -> int:
    global SPRINGS, POPULATIONS, layers
    s = 0
    with open(filename, 'rt') as f:
        for line in f:
            line = line.strip()
            springs, pops = line.split()
            populations = [int(x) for x in pops.split(',')]
            SPRINGS = springs + ('?' + springs) * 4
            POPULATIONS = populations * 5
            MEMO.clear()
            layers = 0
            result = func(0, 0, [])
            s += result
    return s

if __name__ == '__main__':
    print(part1('day12_input.txt'))
    print(part2('day12_input.txt'))