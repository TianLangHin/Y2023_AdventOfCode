from functools import reduce

def get_input(filename: str) -> list[list[int]]:
    values: list[list[int]] = []
    with open(filename, 'rt') as f:
        for line in f:
            values.append([int(x) for x in line.strip().split()])
    return values

def get_layers(numbers: list[int]) -> list[list[int]]:
    layers = []
    while any(x != 0 for x in numbers):
        layers.append(numbers)
        numbers = [numbers[i+1] - numbers[i] for i in range(len(numbers)-1)]
    return layers

def part1(filename: str) -> int:
    return sum(sum(x[-1] for x in get_layers(numbers)) for numbers in get_input(filename))

def part2(filename: str) -> int:
    diff = lambda itr: reduce(lambda acc, x: x - acc, itr, 0)
    return sum(diff(x[0] for x in reversed(get_layers(numbers))) for numbers in get_input(filename))

if __name__ == '__main__':
    print(part1('day9_input.txt'))
    print(part2('day9_input.txt'))