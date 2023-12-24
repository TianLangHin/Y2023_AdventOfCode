from collections import namedtuple
from typing import Optional

# px: int, py: int, pz: int, vx: int, vy: int, vz: int
Hailstone = namedtuple('Hailstone', ['px', 'py', 'pz', 'vx', 'vy', 'vz'])

def has_intersection(h1: Hailstone, h2: Hailstone) -> Optional[tuple[int, int, int]]:
    px1, py1, pz1, vx1, vy1, vz1 = h1
    px2, py2, pz2, vx2, vy2, vz2 = h2
    cx, cy, cz = px2 - px1, py2 - py1, pz2 - pz1
    k = vx1 * vy2 - vx2 * vy1
    t1 = vy2 * cx - vx2 * cy
    t2 = vy1 * cx - vx1 * cy
    if k < 0:
        k, t1, t2 = -k, -t1, -t2
    if t1 < 0 or t2 < 0:
        return None
    return k, t1, t2

def part1(filename: str) -> int:
    hailstones = []
    with open(filename, 'rt') as f:
        for line in f:
            p, v = line.strip().split(' @ ')
            hailstones.append(
                Hailstone(
                    *[int(x) for x in p.split(', ')],
                    *[int(x) for x in v.split(', ')]
                )
            )
    lower = 200000000000000
    higher = 400000000000000
    s = 0
    for i in range(len(hailstones)):
        for j in range(i+1, len(hailstones)):
            result = has_intersection(hailstones[i], hailstones[j])
            if result is not None:
                k, t1, t2 = result
                scaled_low, scaled_high = k * lower, k * higher
                if (scaled_low <= k * hailstones[i].px + hailstones[i].vx * t1 <= scaled_high and
                    scaled_low <= k * hailstones[i].py + hailstones[i].vy * t1 <= scaled_high):
                    s += 1
    return s

if __name__ == '__main__':
    print(part1('day24_input.txt'))