from functools import reduce

def part1(filename: str) -> int:
    with open(filename, 'rt') as f:
        text = f.readlines()[0].strip()
    strings = text.split(',')
    return sum(
        reduce(lambda acc, e: ((acc + ord(e)) * 17) % 256, string, 0)
        for string in strings
    )

if __name__ == '__main__':
    print(part1('day15_input.txt'))