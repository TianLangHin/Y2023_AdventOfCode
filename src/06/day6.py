from math import ceil, floor, sqrt

def part1(filename: str) -> int:
    with open(filename, 'rt') as f:
        time, distance = [[int(y) for y in x.split(':')[1].strip().split()] for x in f.readlines()]
    s = 1
    for t, d in zip(time, distance):
        h_shift = t/2
        v_shift = t*t/4 - d
        if v_shift > 0:
            lower_bound = h_shift - sqrt(v_shift)
            upper_bound = h_shift + sqrt(v_shift)
            lower_bound = ceil(lower_bound) + (lower_bound == int(lower_bound))
            upper_bound = floor(upper_bound) - (upper_bound == int(upper_bound))
            if upper_bound >= lower_bound:
                s *= upper_bound - lower_bound + 1
    return s

if __name__ == '__main__':
    print(part1('day6_input.txt'))