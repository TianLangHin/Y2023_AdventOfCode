import re

def part1(filename: str) -> int:
    with open(filename, 'rt') as f:
        lines = f.readlines()
    s = 0
    for line in lines:
        l = [c for c in line if c in '0123456789']
        s += int(l[0] + l[-1])
    return s

def part2(filename: str) -> int:
    with open(filename, 'rt') as f:
        lines = f.readlines()
    s = 0
    conv = {
        'one': 1,   '1': 1,
        'two': 2,   '2': 2,
        'three': 3, '3': 3,
        'four': 4,  '4': 4,
        'five': 5,  '5': 5,
        'six': 6,   '6': 6,
        'seven': 7, '7': 7,
        'eight': 8, '8': 8,
        'nine': 9,  '9': 9
    }
    for line in lines:
        line = line.strip()
        digits = re.findall('(?=(one|two|three|four|five|six|seven|eight|nine|\d))', line)
        x = conv[digits[0]]*10 + conv[digits[-1]]
        s += x
    return s

if __name__ == '__main__':
    print(part1('day1_input.txt'))
    print(part2('day1_input.txt'))