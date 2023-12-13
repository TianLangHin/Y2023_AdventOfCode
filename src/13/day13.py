# Rock (#) is True
def mirror_positions_col(grid: list[bool], row_length: int, col_length: int, p2: bool) -> int | list[int]:
    col_keys = [
        sum(grid[i + j * row_length] << j for j in range(col_length))
        for i in range(row_length)
    ]
    results = []
    first, last = 0, len(col_keys) - 1
    while first != last:
        if not ((last - first) & 1):
            first += 1
        else:
            f, l = first, last
            palindrome = True
            count = 0
            while f < l and palindrome:
                if p2:
                    x = col_keys[f] ^ col_keys[l]
                    if (x & (x - 1)) == 0 and col_keys[f] != col_keys[l]:
                        palindrome = count == 0
                        count += 1
                    else:
                        palindrome = col_keys[f] == col_keys[l]
                else:
                    palindrome = col_keys[f] == col_keys[l]
                f += 1
                l -= 1
            if palindrome:
                results.append(1 + first + ((last - first) >> 1))
            first += 1
    col_keys.reverse()
    first, last = 0, len(col_keys) - 1
    while first != last:
        if not ((last - first) & 1):
            first += 1
        else:
            f, l = first, last
            palindrome = True
            count = 0
            while f < l and palindrome:
                if p2:
                    x = col_keys[f] ^ col_keys[l]
                    if (x & (x - 1)) == 0 and col_keys[f] != col_keys[l]:
                        palindrome = count == 0
                        count += 1
                    else:
                        palindrome = col_keys[f] == col_keys[l]
                else:
                    palindrome = col_keys[f] == col_keys[l]
                f += 1
                l -= 1
            if palindrome:
                results.append(row_length - (1 + first + ((last - first) >> 1)))
            first += 1
    if p2:
        return results
    else:
        return 0 if not results else min(results)

def mirror_positions_row(grid: list[bool], row_length: int, col_length: int, p2: bool) -> int | list[int]:
    row_keys = [
        sum(grid[i * row_length + j] << j for j in range(row_length))
        for i in range(col_length)
    ]
    results = []
    first, last = 0, len(row_keys) - 1
    while first != last:
        if not ((last - first) & 1):
            first += 1
        else:
            f, l = first, last
            palindrome = True
            count = 0
            while f < l and palindrome:
                if p2:
                    x = row_keys[f] ^ row_keys[l]
                    if (x & (x - 1)) == 0 and row_keys[f] != row_keys[l]:
                        palindrome = count == 0
                        count += 1
                    else:
                        palindrome = row_keys[f] == row_keys[l]
                else:
                    palindrome = row_keys[f] == row_keys[l]
                f += 1
                l -= 1
            if palindrome:
                results.append(1 + first + ((last - first) >> 1))
            first += 1
    row_keys.reverse()
    first, last = 0, len(row_keys) - 1
    while first != last:
        if not ((last - first) & 1):
            first += 1
        else:
            f, l = first, last
            palindrome = True
            count = 0
            while f < l and palindrome:
                if p2:
                    x = row_keys[f] ^ row_keys[l]
                    if (x & (x - 1)) == 0 and row_keys[f] != row_keys[l]:
                        palindrome = count == 0
                        count += 1
                    else:
                        palindrome = row_keys[f] == row_keys[l]
                else:
                    palindrome = row_keys[f] == row_keys[l]
                f += 1
                l -= 1
            if palindrome:
                results.append(col_length - (1 + first + ((last - first) >> 1)))
            first += 1
    if p2:
        return [100*x for x in results]
    else:
        return 0 if not results else 100 * min(results)

def part1(filename: str) -> int:
    s = 0
    r = 0
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
                r = mirror_positions_col(grid, row, col, False)
                if r == 0:
                    r = mirror_positions_row(grid, row, col, False)
                row, col = 0, 0
                grid = []
                s += r
    r = mirror_positions_col(grid, row, col, False)
    if r == 0:
        r = mirror_positions_row(grid, row, col, False)
    row, col = 0, 0
    grid = []
    s += r
    return s

def part2(filename: str) -> int:
    s = 0
    r = 0
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
                p = mirror_positions_col(grid, row, col, False)
                if p == 0:
                    p = mirror_positions_row(grid, row, col, False)
                t = mirror_positions_col(grid, row, col, True) + mirror_positions_row(grid, row, col, True)
                row, col = 0, 0
                grid = []
                r = min(x for x in t if x != p)
                s += r
    p = mirror_positions_col(grid, row, col, False)
    if p == 0:
        p = mirror_positions_row(grid, row, col, False)
    t = mirror_positions_col(grid, row, col, True) + mirror_positions_row(grid, row, col, True)
    r = min(x for x in t if x != p)
    row, col = 0, 0
    grid = []
    s += r
    return s

if __name__ == '__main__':
    print(part1('day13_input.txt'))
    print(part2('day13_input.txt'))