# Rock (#) is True
def mirror_positions(bits: list[bool], p2: bool) -> list[int]:
    results = []
    for i in range(2):
        first, last = 0, len(bits) - 1
        while first != last:
            if not ((last - first) & 1):
                first += 1
            else:
                f, l = first, last
                palindrome = True
                count = 0
                while f < l and palindrome:
                    x = bits[f] ^ bits[l]
                    if p2 and (x & (x - 1)) == 0 and bits[f] != bits[l]:
                        palindrome = count == 0
                        count += 1
                    else:
                        palindrome = bits[f] == bits[l]
                    f += 1
                    l -= 1
                if palindrome:
                    if i == 0:
                        results.append(1 + first + ((last - first) >> 1))
                    else:
                        results.append(len(bits) - (1 + first + ((last - first) >> 1)))
                first += 1
        bits.reverse()
    if not p2 and not results:
        results.append(0)
    return results

def part1(filename: str) -> int:
    s = 0
    grid = []
    row, col = 0, 0
    with open(filename, 'rt') as f:
        for line in f:
            line = line.strip()
            if line:
                grid.extend(c == '#' for c in line)
                row = len(line)
                col += 1
            else:
                col_bits = [
                    sum(grid[i + j * row] << j for j in range(col))
                    for i in range(row)
                ]
                r = mirror_positions(col_bits, False)[0]
                if r == 0:
                    row_bits = [
                        sum(grid[i * row + j] << j for j in range(row))
                        for i in range(col)
                    ]
                    r = 100 * mirror_positions(row_bits, False)[0]
                row, col = 0, 0
                grid = []
                s += r
    col_bits = [
        sum(grid[i + j * row] << j for j in range(col))
        for i in range(row)
    ]
    r = mirror_positions(col_bits, False)[0]
    if r == 0:
        row_bits = [
            sum(grid[i * row + j] << j for j in range(row))
            for i in range(col)
        ]
        r = 100 * mirror_positions(row_bits, False)[0]
    s += r
    return s

def part2(filename: str) -> int:
    s = 0
    grid = []
    row, col = 0, 0
    with open(filename, 'rt') as f:
        for line in f:
            line = line.strip()
            if line:
                grid.extend(c == '#' for c in line)
                row = len(line)
                col += 1
            else:
                col_bits = [
                    sum(grid[i + j * row] << j for j in range(col))
                    for i in range(row)
                ]
                row_bits = [
                    sum(grid[i * row + j] << j for j in range(row))
                    for i in range(col)
                ]
                p = mirror_positions(col_bits, False)[0]
                if p == 0:
                    p = 100 * mirror_positions(row_bits, False)[0]
                t = (mirror_positions(col_bits, True) +
                     [100*x for x in mirror_positions(row_bits, True)])
                row, col = 0, 0
                grid = []
                if not t:
                    t.append(0)
                s += min(x for x in t if x != p)
    col_bits = [
        sum(grid[i + j * row] << j for j in range(col))
        for i in range(row)
    ]
    row_bits = [
        sum(grid[i * row + j] << j for j in range(row))
        for i in range(col)
    ]
    p = mirror_positions(col_bits, False)[0]
    if p == 0:
        p = 100 * mirror_positions(row_bits, False)[0]
    t = (mirror_positions(col_bits, True) +
         [100*x for x in mirror_positions(row_bits, True)])
    if not t:
        t.append(0)
    s += min(x for x in t if x != p)
    return s

if __name__ == '__main__':
    print(part1('day13_input.txt'))
    print(part2('day13_input.txt'))