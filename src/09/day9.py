from functools import reduce

def get_input(filename: str) -> list[int]:
    with open(filename, 'rt') as f:
        for line in f:
            yield [int(x) for x in line.strip().split()]

def get_layers(numbers: list[int], index: int) -> list[int]:
    layers = []
    while any(x != 0 for x in numbers):
        layers.append(numbers[index])
        numbers = [numbers[i+1] - numbers[i] for i in range(len(numbers)-1)]
    return layers

def part1(filename: str) -> int:
    return sum(sum(get_layers(numbers, -1)) for numbers in get_input(filename))

def part2(filename: str) -> int:
    diff = lambda itr: reduce(lambda acc, x: x - acc, itr, 0)
    return sum(diff(reversed(get_layers(numbers, 0))) for numbers in get_input(filename))

if __name__ == '__main__':
    print(part1('day9_input.txt'))
    print(part2('day9_input.txt'))