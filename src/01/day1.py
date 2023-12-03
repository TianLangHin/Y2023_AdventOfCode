import re

def part_one():
    with open('day1_input.txt', 'rt') as f:
        lines = f.readlines()
    s = 0
    for line in lines:
        l = [c for c in line if c in '0123456789']
        s += int(l[0] + l[-1])
    print(s)

def part_two():
    with open('day1_input.txt', 'rt') as f:
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
    print(s)

#part_one()
part_two()