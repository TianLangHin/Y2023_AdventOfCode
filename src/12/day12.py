ROW_LENGTH: int = 0
MEMO: dict[int, int] = {}
SPRINGS: str = ''
POPULATIONS: list[int] = []

def func(i: int, j: int) -> int:
    if j == len(POPULATIONS):
        if '#' not in SPRINGS[i:]:
            MEMO[i * ROW_LENGTH + j] = 1
            return 1
        else:
            return 0
    elif i >= len(SPRINGS):
        return 0
    cached_result = MEMO.get(i * ROW_LENGTH + j, None)
    if cached_result is not None:
        return cached_result
    this_skip = 0
    if SPRINGS[i] != '#':
        this_skip = func(i+1, j)
    this_match = 0
    if SPRINGS[i] != '.':
        if ((not any(k == '.' for k in SPRINGS[i:i+POPULATIONS[j]])) and
            i + POPULATIONS[j] <= len(SPRINGS) and
            not (i + POPULATIONS[j] < len(SPRINGS) and
            SPRINGS[i + POPULATIONS[j]] == '#')):
            this_match = func(i + POPULATIONS[j] + 1, j + 1)
    MEMO[i * ROW_LENGTH + j] = this_skip + this_match
    return this_skip + this_match

def part1(filename: str) -> int:
    global SPRINGS, POPULATIONS, ROW_LENGTH
    s = 0
    with open(filename, 'rt') as f:
        for line in f:
            springs, pops = line.strip().split()
            populations = [int(x) for x in pops.split(',')]
            SPRINGS = springs
            POPULATIONS = populations
            MEMO.clear()
            ROW_LENGTH = len(line)
            result = func(0, 0)
            s += result
    return s

def part2(filename: str) -> int:
    global SPRINGS, POPULATIONS, ROW_LENGTH
    s = 0
    with open(filename, 'rt') as f:
        for line in f:
            springs, pops = line.strip().split()
            populations = [int(x) for x in pops.split(',')]
            SPRINGS = '?'.join([springs]*5)
            POPULATIONS = populations * 5
            MEMO.clear()
            ROW_LENGTH = len(line)
            result = func(0, 0)
            s += result
    return s

if __name__ == '__main__':
    print(part1('day12_input.txt'))
    print(part2('day12_input.txt'))