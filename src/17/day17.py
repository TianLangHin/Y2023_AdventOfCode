RIGHT, DOWN, UP, LEFT = 0, 1, 2, 3

def in_bound(direction: int, pos: int, x_bound: int, y_bound: int) -> bool:
    if direction == RIGHT:
        return pos % x_bound < x_bound - 1
    elif direction == LEFT:
        return pos % x_bound > 0
    elif direction == DOWN:
        return pos // x_bound < y_bound - 1
    elif direction == UP:
        return pos // x_bound > 0

def increment(index: int, direction: int, x_bound: int) -> int:
    if direction == RIGHT:
        return index + 1
    elif direction == LEFT:
        return index - 1
    elif direction == DOWN:
        return index + x_bound
    elif direction == UP:
        return index - x_bound

# `count` is [0-3] << 2 for Part 1
# two bits for `count`.
def part1(filename: str) -> int:
    with open(filename, 'rt') as f:
        text_lines = [x.strip() for x in f.readlines()]
    x_bound = len(text_lines[0])
    y_bound = len(text_lines)
    grid = [int(x) for line in text_lines for x in line]
    max_cost = 9 * (x_bound + y_bound)
    costs = [max_cost] * (len(grid) * 4 * 4)
    stopping_point = len(grid) - 1
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
                        i = (increment(0, search_direction, x_bound) << 4) | 4 | search_direction
                        new_cost = costs[index] + grid[i >> 4]
                        if new_cost >= costs[i]:
                            continue
                        costs[i] = new_cost
                        if (i >> 4) != stopping_point:
                            new_paths.add(i)
                    elif search_direction == (index & 3): # same direction
                        if (index & 12) != 12: # count is not 3 (represented as 12)
                            # count + 1
                            i = (increment(index >> 4, search_direction, x_bound) << 4) + (index & 15) + 4
                            new_cost = costs[index] + grid[i >> 4]
                            if new_cost >= costs[i]:
                                continue
                            costs[i] = new_cost
                            if (i >> 4) != stopping_point:
                                new_paths.add(i)
                    # switching direction, set count to 0, cannot reverse
                    elif search_direction + (index & 3) != 3:
                        # ignore count, store new direction
                        i = (increment(index >> 4, search_direction, x_bound) << 4) | 4 | search_direction
                        new_cost = costs[index] + grid[i >> 4]
                        if new_cost >= costs[i]:
                            continue
                        costs[i] = new_cost
                        if (i >> 4) != stopping_point:
                            new_paths.add(i)
        paths = new_paths
    return min(costs[stopping_point << 4 : (stopping_point << 4) + 16])

# `count` is [0-10] << 2 for Part 1
# four bits for `count`.
def part2(filename: str) -> int:
    with open(filename, 'rt') as f:
        text_lines = [x.strip() for x in f.readlines()]
    x_bound = len(text_lines[0])
    y_bound = len(text_lines)
    grid = [int(x) for line in text_lines for x in line]
    max_cost = 9 * (x_bound + y_bound)
    costs = [max_cost] * (len(grid) * 4 * 16)
    stopping_point = len(grid) - 1
    # index is represented as: [index][count][direction]
    costs[0] = 0
    paths = {0}
    while paths:
        new_paths = set()
        for index in paths:
            for search_direction in range(4):
                if in_bound(search_direction, index >> 6, x_bound, y_bound):
                    if index == 0:
                        # ignore count, store new direction
                        i = (increment(0, search_direction, x_bound) << 6) | 4 | search_direction
                        new_cost = costs[index] + grid[i >> 6]
                        if new_cost >= costs[i]:
                            continue
                        costs[i] = new_cost
                        if (i >> 6) != stopping_point:
                            new_paths.add(i)
                    elif search_direction == (index & 3): # same direction
                        if (index & 60) < 40: # count is <= 10 (represented as 40)
                            # count + 1
                            i = (increment(index >> 6, search_direction, x_bound) << 6) + (index & 63) + 4
                            new_cost = costs[index] + grid[i >> 6]
                            if new_cost >= costs[i]:
                                continue
                            if (i >> 6) == stopping_point:
                                if (i & 60) >= 16: # count is >= 4 (represented as 16)
                                    costs[i] = new_cost
                            else:
                                costs[i] = new_cost
                                new_paths.add(i)
                    # switching direction, set count to 0, cannot reverse
                    elif search_direction + (index & 3) != 3:
                        if (index & 60) >= 16: # count is >= 4 (represented as 16)
                            # ignore count, store new direction
                            i = (increment(index >> 6, search_direction, x_bound) << 6) | 4 | search_direction
                            new_cost = costs[index] + grid[i >> 6]
                            if new_cost >= costs[i]:
                                continue
                            if (i >> 6) != stopping_point:
                                costs[i] = new_cost
                                new_paths.add(i)
        paths = new_paths
    return min(costs[stopping_point << 6 : (stopping_point << 6) + 64])

if __name__ == '__main__':
    print(part1('day17_input.txt'))
    print(part2('day17_input.txt'))