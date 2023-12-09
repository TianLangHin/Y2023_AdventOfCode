def part1(filename: str) -> int:
    values: list[list[int]] = []
    with open(filename, 'rt') as f:
        for line in f:
            values.append([int(x) for x in line.strip().split()])
    s = 0
    for numbers in values:
        n = len(numbers)
        layers = []
        while any(x != 0 for x in numbers):
            layers.append(numbers)
            numbers = [numbers[i+1] - numbers[i] for i in range(len(numbers)-1)]
        s += sum(x[-1] for x in layers) + numbers[-1]
    return s

def part2(filename: str) -> int:
    pass

if __name__ == '__main__':
    print(part1('day9_input.txt'))
    print(part2('test_input.txt'))
    #print(unique_polynomial([10, 13, 16, 21, 30, 45]))