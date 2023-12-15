from collections import namedtuple
from functools import reduce

Lens = namedtuple('Lens', ['label', 'focal_length'])

def hash(string: str) -> int:
    return reduce(lambda acc, e: ((acc + ord(e)) * 17) & 255, string, 0)

def part1(filename: str) -> int:
    with open(filename, 'rt') as f:
        text = f.readlines()[0].strip()
    strings = text.split(',')
    return sum(hash(string) for string in strings)

def part2(filename: str) -> int:
    with open(filename, 'rt') as f:
        text = f.readlines()[0].strip()
    strings = text.split(',')
    operations = [
        (string[:-1], 0) if string.endswith('-')
        else [(a, int(b)) for a, b in [string.split('=')]][0]
        for string in strings
    ]
    boxes = [[] for _ in range(256)]
    for string, focal_length in operations:
        box_index = hash(string)
        if focal_length == 0:
            for i in range(len(boxes[box_index])):
                if boxes[box_index][i].label == string:
                    f = boxes[box_index][i].focal_length
                    boxes[box_index].remove(Lens(string, f))
                    break
        else:
            found = False
            for i in range(len(boxes[box_index])):
                if boxes[box_index][i].label == string:
                    boxes[box_index][i] = Lens(string, focal_length)
                    found = True
                    break
            if not found:
                boxes[box_index].append(Lens(string, focal_length))
    return sum(
        (i+1) * sum((j+1) * f for j, (_, f) in enumerate(boxes[i]))
        for i in range(256)
    )

if __name__ == '__main__':
    print(part1('day15_input.txt'))
    print(part2('day15_input.txt'))