from collections import namedtuple
from math import inf

# index: int, direction: int, split: bool
StepResult = namedtuple('StepResult', ['index', 'direction', 'split'])

# direction is one of: 1, -1, x_bound, -x_bound
def step(index: int, grid: list[str], x_bound: int, direction: int) -> StepResult:
    travelling_x = abs(direction) == 1
    grid_bound = 0 if direction < 0 else index + x_bound if travelling_x else len(grid)

    if direction < 0:
        if travelling_x:
            grid_bound = (index // x_bound) * x_bound - 1
        else:
            grid_bound = -1
    else:
        if travelling_x:
            grid_bound = (index // x_bound + 1) * x_bound
        else:
            grid_bound = len(grid)

    i = index
    for i in range(index + direction, grid_bound, direction):
        match grid[i]:
            case '.':
                pass
            case '|' if travelling_x:
                return StepResult(i, x_bound, True)
            case '-' if not travelling_x:
                return StepResult(i, 1, True)
            case '/':
                m = x_bound if travelling_x else 1
                s = -1 if direction > 0 else 1
                return StepResult(i, s * m, False)
            case '\\':
                m = x_bound if travelling_x else 1
                s = -1 if direction < 0 else 1
                return StepResult(i, s * m, False)
    return StepResult(i, 0, False)

def energy(index: int, starting_direction: int, grid: list[str], x_bound: int) -> int:
    traversed_points = set()
    traversed_directions = {(0, starting_direction)}
    paths = {(0, starting_direction)}
    while paths:
        new_paths = set()
        for index, direction in paths:
            step_result = step(index, grid, x_bound, direction)
            if step_result.direction == 0:
                pass
            elif step_result.split:
                if (step_result.index, step_result.direction) not in traversed_directions:
                    new_paths.add((step_result.index, step_result.direction))
                    traversed_directions.add((step_result.index, step_result.direction))
                if (step_result.index, -step_result.direction) not in traversed_directions:
                    new_paths.add((step_result.index, -step_result.direction))
                    traversed_directions.add((step_result.index, -step_result.direction))
            else:
                if (step_result.index, step_result.direction) not in traversed_directions:
                    new_paths.add((step_result.index, step_result.direction))
            traversed_points |= set(range(index, step_result.index + direction, direction))
        paths = new_paths
    return len(traversed_points)

def part1(filename: str) -> int:
    with open(filename, 'rt') as f:
        lines = [x.strip() for x in f.readlines()]
    x_bound = len(lines[0])
    grid = sum([list(x) for x in lines], [])
    match grid[0]:
        case '.' | '-':
            starting_direction = 1
        case '|' | '\\':
            starting_direction = x_bound
        case '/':
            starting_direction = -x_bound
    return energy(0, starting_direction, grid, x_bound)

if __name__ == '__main__':
    print(part1('day16_input.txt'))