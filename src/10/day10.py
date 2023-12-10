from collections import namedtuple

# p1: int, p2: int
# They are the offsets you need to add to its index
# to get to the neighbour designated by p1 or p2.
Pipe = namedtuple('Pipe', ['p1', 'p2'])

# x-axis: - is left, + is right
# y-axis: - is up,   + is down
def to_pipe(character: str, *, row_length: int) -> Pipe:
    match character:
        case '|':
            return Pipe(-row_length, row_length)
        case '-':
            return Pipe(-1, 1)
        case 'L':
            return Pipe(-row_length, 1)
        case 'J':
            return Pipe(-row_length, -1)
        case '7':
            return Pipe(-1, row_length)
        case 'F':
            return Pipe(1, row_length)
        case '.':
            return Pipe(0, 0)
        case 'S':
            return Pipe(1, 1) # This is sentinel for 'S'

def part1(filename: str) -> int:
    pipes: list[int] = []
    s_index = None
    with open(filename, 'rt') as f:
        max_rows = 0
        for line in f:
            n = len(line.strip())
            row = [to_pipe(x, row_length=n) for x in line.strip()]
            for i in range(len(row)):
                if row[i].p1 == 1 and row[i].p2 == 1:
                    if s_index is not None:
                        print('This should not happen')
                    s_index = len(pipes) + i
            pipes.extend(row)
            max_rows += 1
    directions = []
    if s_index // n > 0:
        if pipes[s_index - n].p1 == n or pipes[s_index - n].p2 == n:
            directions.append(-n)
    if s_index % n > 0:
        if pipes[s_index - 1].p1 == 1 or pipes[s_index - 1].p2 == 1:
            directions.append(-1)
    if s_index % n < n-1:
        if pipes[s_index + 1].p1 == -1 or pipes[s_index + 1].p2 == -1:
            directions.append(1)
    if s_index // n < max_rows - 1:
        if pipes[s_index + n].p1 == -n or pipes[s_index + n].p2 == -n:
            directions.append(n)
    pipes[s_index] = Pipe(directions[0], directions[1])
    path_length = 0
    i = s_index
    choice = True
    while True:
        next_pipe_offset = pipes[i].p1 if choice else pipes[i].p2
        if pipes[i + next_pipe_offset].p1 + next_pipe_offset == 0:
            choice = False
        else:
            choice = True
        i += next_pipe_offset
        path_length += 1
        if i == s_index:
            break
    return path_length // 2

def part2(filename: str) -> int:
    return 0

if __name__ == '__main__':
    print(part1('day10_input.txt'))
    print(part2('test_input.txt'))