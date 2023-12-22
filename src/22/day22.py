from collections import namedtuple

Brick = namedtuple('Brick', ['x1', 'x2', 'y1', 'y2', 'z1', 'z2'])

# `bricks` and `lowest_heights` will be mutated
def pushed_height(
        index: int,
        dependencies: list[list[int]],
        bricks: list[Brick],
        lowest_heights: dict[int, int]) -> int:
    if (result := lowest_heights.get(index, None)) is not None:
        return result
    if dependencies[index]:
        lowest_z = max(
            pushed_height(i, dependencies, bricks, lowest_heights)
            for i in dependencies[index]
        ) + 1
        height = bricks[index].z2 - bricks[index].z1
        bricks[index] = Brick(
            bricks[index].x1, bricks[index].x2,
            bricks[index].y1, bricks[index].y2,
            lowest_z, lowest_z + height
        )
        lowest_heights[index] = lowest_z + height
        return lowest_z + height
    else:
        height = bricks[index].z2 - bricks[index].z1
        bricks[index] = Brick(
            bricks[index].x1, bricks[index].x2,
            bricks[index].y1, bricks[index].y2,
            0, height
        )
        lowest_heights[index] = height
        return height

def part1(filename: str) -> int:
    bricks = []
    with open(filename, 'rt') as f:
        for line in f:
            e1, e2 = line.strip().split('~')
            x1, y1, z1 = [int(x) for x in e1.split(',')]
            x2, y2, z2 = [int(x) for x in e2.split(',')]
            bricks.append(
                Brick(
                    min(x1, x2), max(x1, x2),
                    min(y1, y2), max(y1, y2),
                    min(z1, z2), max(z1, z2)
                )
            )
    dependencies = [[] for _ in range(len(bricks))]
    for i in range(len(bricks)):
        this = bricks[i]
        for j in range(len(bricks)):
            if i == j:
                continue
            that = bricks[j]
            if (that.z2 < this.z1 and
                that.x2 >= this.x1 and that.x1 <= this.x2 and
                that.y2 >= this.y1 and that.y1 <= this.y2):
                dependencies[i].append(j)
    lowest_heights = dict()
    for i in range(len(bricks)):
        pushed_height(i, dependencies, bricks, lowest_heights)
    dependencies = [
        [dep for dep in dependencies[i] if bricks[dep].z2 + 1 == bricks[i].z1]
        for i in range(len(bricks))
    ]
    return len(bricks) - len({s[0] for s in dependencies if len(s) == 1})

def part2(filename: str) -> int:
    bricks = []
    with open(filename, 'rt') as f:
        for line in f:
            e1, e2 = line.strip().split('~')
            x1, y1, z1 = [int(x) for x in e1.split(',')]
            x2, y2, z2 = [int(x) for x in e2.split(',')]
            bricks.append(
                Brick(
                    min(x1, x2), max(x1, x2),
                    min(y1, y2), max(y1, y2),
                    min(z1, z2), max(z1, z2)
                )
            )
    dependencies = [[] for _ in range(len(bricks))]
    for i in range(len(bricks)):
        this = bricks[i]
        for j in range(len(bricks)):
            if i == j:
                continue
            that = bricks[j]
            if (that.z2 < this.z1 and
                that.x2 >= this.x1 and that.x1 <= this.x2 and
                that.y2 >= this.y1 and that.y1 <= this.y2):
                dependencies[i].append(j)
    lowest_heights = dict()
    for i in range(len(bricks)):
        pushed_height(i, dependencies, bricks, lowest_heights)
    dependencies = [
        [dep for dep in dependencies[i] if bricks[dep].z2 + 1 == bricks[i].z1]
        for i in range(len(bricks))
    ]
    s = 0
    for i in range(len(bricks)):
        falling = {i}
        k = 0
        while True:
            l = len(falling)
            new_entries = [
                j for j in range(len(bricks))
                if all(x in falling for x in dependencies[j])
                and len(dependencies[j]) > 0
            ]
            for e in new_entries: falling.add(e)
            if len(falling) == l:
                break
        s += len(falling) - 1
    return s

if __name__ == '__main__':
    print(part1('day22_input.txt'))
    print(part2('day22_input.txt'))