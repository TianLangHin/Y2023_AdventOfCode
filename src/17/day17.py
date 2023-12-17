RIGHT, DOWN, UP, LEFT = 0, 1, 2, 3
# count is 0, 4, 8, 12

def in_bound(direction: int, pos: int, x_bound: int, y_bound: int) -> bool:
    if direction == RIGHT:
        return pos % x_bound < x_bound - 1
    elif direction == LEFT:
        return pos % x_bound > 0
    elif direction == DOWN:
        return pos // x_bound < y_bound - 1
    elif direction == UP:
        return pos // x_bound > 0

def compute_min_path(grid: list[int], x_bound: int, y_bound: int) -> int:
    max_cost = 9 * (x_bound + y_bound)
    costs = [max_cost] * (len(grid) * 4 * 4)
    stopping_point = len(grid) - 1
    increment = lambda idx, d: idx + ((
        1 if d == RIGHT else -1 if d == LEFT else -x_bound if d == UP else x_bound) << 4)
    # index is represented as: [index][count][direction]
    costs[0] = 0
    paths = {0}
    while paths:
        new_paths = set()
        for index in paths:
            for search_direction in range(4):
                if in_bound(search_direction, index >> 4, x_bound, y_bound):
                    if index == 0:
                        # ignore count, store new direction
                        i = increment(0, search_direction) | 4 | search_direction
                        new_cost = costs[index] + grid[i >> 4]
                        if new_cost >= costs[i]:
                            continue
                        costs[i] = new_cost
                        if (i >> 4) != stopping_point:
                            new_paths.add(i)
                    elif search_direction == (index & 3): # same direction
                        if (index & 12) != 12: # count is not 3 (represented as 12)
                            # count + 1
                            i = increment(index, search_direction) + 4
                            new_cost = costs[index] + grid[i >> 4]
                            if new_cost >= costs[i]:
                                continue
                            costs[i] = new_cost
                            if (i >> 4) != stopping_point:
                                new_paths.add(i)
                    # switching direction, set count to 0, cannot reverse
                    elif search_direction + (index & 3) != 3:
                        # ignore count, store new direction
                        i = ((increment(index, search_direction) >> 4) << 4) | 4 | search_direction
                        new_cost = costs[index] + grid[i >> 4]
                        if new_cost >= costs[i]:
                            continue
                        costs[i] = new_cost
                        if (i >> 4) != stopping_point:
                            new_paths.add(i)
        paths = new_paths
    return min(costs[stopping_point << 4 : (stopping_point << 4) + 16])

def part1(filename: str) -> int:
    with open(filename, 'rt') as f:
        text_lines = [x.strip() for x in f.readlines()]
    x_bound = len(text_lines[0])
    y_bound = len(text_lines)
    grid = [int(x) for line in text_lines for x in line]
    return compute_min_path(grid, x_bound, y_bound)

if __name__ == '__main__':
    print(part1('day17_input.txt'))