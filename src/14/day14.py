def one_cycle(grid: list[int], x_bound: int, y_bound: int):
    # north
    anchors = [0] * x_bound
    for i in range(y_bound):
        for j in range(x_bound):
            match grid[i * x_bound + j]:
                case 1:
                    grid[i * x_bound + j] = 0
                    grid[anchors[j] * x_bound + j] = 1
                    anchors[j] += 1
                case 2:
                    anchors[j] = i+1
    # west
    anchors = [0] * y_bound
    for i in range(y_bound):
        for j in range(x_bound):
            match grid[i * x_bound + j]:
                case 1:
                    grid[i * x_bound + j] = 0
                    grid[i * x_bound + anchors[i]] = 1
                    anchors[i] += 1
                case 2:
                    anchors[i] = j+1
    # south
    anchors = [x_bound] * x_bound
    for i in range(y_bound-1, -1, -1):
        for j in range(x_bound-1, -1, -1):
            match grid[i * x_bound + j]:
                case 1:
                    grid[i * x_bound + j] = 0
                    grid[(anchors[j] - 1) * x_bound + j] = 1
                    anchors[j] -= 1
                case 2:
                    anchors[j] = i
    # east
    anchors = [y_bound] * y_bound
    for i in range(x_bound-1, -1, -1):
        for j in range(y_bound-1, -1, -1):
            match grid[i * x_bound + j]:
                case 1:
                    grid[i * x_bound + j] = 0
                    grid[i * x_bound + (anchors[i] - 1)] = 1
                    anchors[i] -= 1
                case 2:
                    anchors[i] = j

def grid_to_key(grid: list[int]) -> int:
    return sum(grid[i] << (i << 1) for i in range(len(grid)))

def key_to_grid(key: int, total_length: int) -> list[int]:
    grid = []
    while key != 0:
        grid.append(key & 3)
        key >>= 2
    for _ in range(total_length - len(grid)):
        grid.append(0)
    return grid

def part1(filename: str) -> int:
    with open(filename, 'rt') as f:
        lines = [x.strip() for x in f.readlines()]
    n = len(lines[0])
    last_anchor = [0] * n
    column_weights = [(0, 0)] * n
    row_number = 0
    for line in lines:
        for i in range(n):
            match line[i]:
                case '#':
                    last_anchor[i] = row_number + 1
                case 'O':
                    column_weights[i] = (
                        column_weights[i][0] + 1,
                        column_weights[i][1] + last_anchor[i]
                    )
                    last_anchor[i] += 1
        row_number += 1
    return sum(
        c * row_number - s
        for c, s in column_weights
    )

def part2(filename: str) -> int:
    with open(filename, 'rt') as f:
        lines = [x.strip() for x in f.readlines()]
    x_bound = len(lines[0])
    y_bound = len(lines)
    grid = []
    for line in lines:
        for character in line:
            match character:
                case '#':
                    grid.append(2)
                case 'O':
                    grid.append(1)
                case '.':
                    grid.append(0)
    memo = [grid_to_key(grid)]
    index = None
    while True:
        one_cycle(grid, x_bound, y_bound)
        new_key = grid_to_key(grid)
        index = None
        for i in range(len(memo)):
            if memo[i] == new_key:
                index = i
                break
        if index is None:
            memo.append(new_key)
        else:
            break
    final_result = memo[(1_000_000_000 - index) % (len(memo) - index) + index]
    final_grid = key_to_grid(final_result, x_bound * y_bound)
    final_load = 0
    for i in range(y_bound):
        for j in range(x_bound):
            if final_grid[i * x_bound + j] == 1:
                final_load += y_bound - i
    return final_load

if __name__ == '__main__':
    print(part1('day14_input.txt'))
    print(part2('day14_input.txt'))