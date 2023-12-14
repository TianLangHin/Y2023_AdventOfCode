from collections import namedtuple

RockData = namedtuple('RockData', ['count', 'sum'])

def part1(filename: str) -> int:
    with open(filename, 'rt') as f:
        lines = [x.strip() for x in f.readlines()]
    n = len(lines[0])
    last_anchor = [-1] * n
    column_weights = [RockData(0, 0)] * n
    row_number = 0
    for line in lines:
        for i in range(n):
            match line[i]:
                case '#':
                    last_anchor[i] = row_number
                case 'O':
                    column_weights[i] = RockData(
                        column_weights[i].count + 1,
                        column_weights[i].sum + last_anchor[i] + 1
                    )
                    last_anchor[i] += 1
        row_number += 1
    return sum(
        c * row_number - s
        for c, s in column_weights
    )

def part2(filename: str) -> int:
    return 0

if __name__ == '__main__':
    print(part1('day14_input.txt'))
    print(part2('test_input.txt'))