def dfs(start: int,
        end: int,
        prev_weight: int,
        path: list[int],
        legal_moves: dict[int, list[tuple[int, int]]]) -> int:
    if start == end:
        return prev_weight
    return max(
        (dfs(move[0], end, prev_weight + move[1], path + [move[0]], legal_moves)
        for move in legal_moves[start]
        if move[0] not in path),
        default=0)

def step_forward(index: int,
                 next_square: int,
                 next_steps: dict[int, set[int]]) -> (int, int):
    step_count = 1
    possibilities = next_steps[next_square].copy()
    if index in possibilities:
        possibilities.remove(index)
    idx = next_square
    while len(possibilities) == 1:
        next_square = next(iter(possibilities))
        possibilities = next_steps[next_square].copy()
        if idx in possibilities:
            possibilities.remove(idx)
        idx = next_square
        step_count += 1
    return idx, step_count

def part1(filename: str) -> int:
    with open(filename, 'rt') as f:
        lines = [x.strip() for x in f.readlines()]
    x_bound = len(lines[0])
    y_bound = len(lines)
    starting_index = lines[0].index('.')
    ending_index = lines[-1].index('.') + x_bound * (y_bound - 1)
    grid = dict()
    i = 0
    for line in lines:
        for c in line:
            match c:
                case '.':
                    grid[i] = 0
                case '^':
                    grid[i] = -x_bound
                case '<':
                    grid[i] = -1
                case 'v':
                    grid[i] = x_bound
                case '>':
                    grid[i] = 1
            i += 1
    next_steps = dict()
    for point, direction in grid.items():
        if direction == 0:
            new_set = set()
            if point % x_bound > 0 and grid.get(point - 1, 1) != 1:
                new_set.add(point - 1)
            if point % x_bound < x_bound - 1 and grid.get(point + 1, -1) != -1:
                new_set.add(point + 1)
            if point // x_bound > 0 and grid.get(point - x_bound, x_bound) != x_bound:
                new_set.add(point - x_bound)
            if point // x_bound < y_bound - 1 and grid.get(point + x_bound, -x_bound) != -x_bound:
                new_set.add(point + x_bound)
            next_steps[point] = new_set
        else:
            if point + direction in grid:
                next_steps[point] = {point + direction}
    edges = dict()
    searching = {(starting_index, nxt) for nxt in next_steps[starting_index]}
    while searching:
        new_searching = set()
        for index, next_square in searching:
            idx, step_count = step_forward(index, next_square, next_steps)
            edges[(index, idx)] = max(step_count, edges.get((index, idx), 0))
            if idx != ending_index:
                for nxt in next_steps[idx]:
                    if nxt != index:
                        new_searching.add((idx, nxt))
        searching = new_searching
    legal_moves = dict()
    for (start, end), weight in edges.items():
        if start in legal_moves:
            legal_moves[start].append((end, weight))
        else:
            legal_moves[start] = [(end, weight)]
    return dfs(starting_index, ending_index, 0, [starting_index], legal_moves)

def step_until_junction(index: int,
                        next_square: int,
                        junctions: set[int],
                        next_steps: dict[int, set[int]]) -> (int, int):
    step_count = 1
    possibilities = next_steps[next_square].copy()
    if index in possibilities:
        possibilities.remove(index)
    idx = next_square
    while next_square not in junctions:
        next_square = next(iter(possibilities))
        possibilities = next_steps[next_square].copy()
        if idx in possibilities:
            possibilities.remove(idx)
        idx = next_square
        step_count += 1
    return idx, step_count

def part2(filename: str) -> int:
    with open(filename, 'rt') as f:
        lines = [x.strip() for x in f.readlines()]
    x_bound = len(lines[0])
    y_bound = len(lines)
    starting_index = lines[0].index('.')
    ending_index = lines[-1].index('.') + x_bound * (y_bound - 1)
    grid = dict()
    i = 0
    for line in lines:
        for c in line:
            match c:
                case '.':
                    grid[i] = 0
                case '^':
                    grid[i] = -x_bound
                case '<':
                    grid[i] = -1
                case 'v':
                    grid[i] = x_bound
                case '>':
                    grid[i] = 1
            i += 1
    junctions = {ending_index}
    next_steps = dict()
    for point, direction in grid.items():
        count = 0
        new_set = set()
        if point % x_bound > 0:
            if grid.get(point - 1, 1) != 1:
                new_set.add(point - 1)
                count += 1
            elif grid.get(point - 1, None) is not None:
                count += 1
        if point % x_bound < x_bound - 1:
            if grid.get(point + 1, -1) != -1:
                new_set.add(point + 1)
                count += 1
            elif grid.get(point + 1, None) is not None:
                count += 1
        if point // x_bound > 0:
            if grid.get(point - x_bound, x_bound) != x_bound:
                new_set.add(point - x_bound)
                count += 1
            elif grid.get(point - x_bound, None) is not None:
                count += 1
        if point // x_bound < y_bound - 1:
            if grid.get(point + x_bound, -x_bound) != -x_bound:
                new_set.add(point + x_bound)
                count += 1
            elif grid.get(point + x_bound, None) is not None:
                count += 1
        if count > 2:
            junctions.add(point)
        next_steps[point] = new_set
    edges = dict()
    searching = {(starting_index, nxt) for nxt in next_steps[starting_index]}
    while searching:
        new_searching = set()
        for index, next_square in searching:
            idx, step_count = step_until_junction(index, next_square, junctions, next_steps)
            edges[(index, idx)] = max(step_count, edges.get((index, idx), 0))
            if idx != ending_index:
                for nxt in next_steps[idx]:
                    if nxt != index:
                        new_searching.add((idx, nxt))
        searching = new_searching
    legal_moves = dict()
    for (start, end), weight in edges.items():
        if start in legal_moves:
            legal_moves[start].append((end, weight))
        else:
            legal_moves[start] = [(end, weight)]
        if end in legal_moves:
            legal_moves[end].append((start, weight))
        else:
            legal_moves[end] = [(start, weight)]
    return dfs(starting_index, ending_index, 0, [starting_index], legal_moves)

if __name__ == '__main__':
    print(part1('day23_input.txt'))
    print(part2('day23_input.txt'))