from functools import reduce

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
    boxes = [dict() for _ in range(256)]
    for string, focal_length in operations:
        index = hash(string)
        if focal_length == 0:
            existing_lens = boxes[index].get(string, None)
            if existing_lens is not None:
                boxes[index].pop(string)
        else:
            boxes[index][string] = focal_length
    return sum(
        (i+1) * sum((j+1) * f for j, (_, f) in enumerate(boxes[i].items()))
        for i in range(256)
    )

if __name__ == '__main__':
    print(part1('day15_input.txt'))
    print(part2('day15_input.txt'))