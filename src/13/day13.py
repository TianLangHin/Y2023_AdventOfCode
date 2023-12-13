# Rock (#) is True
def mirror_positions_col(grid: list[bool], row_length: int, col_length: int) -> int:
    col_keys = [
        sum(grid[i + j * row_length] << j for j in range(col_length))
        for i in range(row_length)
    ]
    first, last = 0, len(col_keys) - 1
    while first < last:
        if col_keys[first] != col_keys[last] or not ((last - first) & 1):
            first += 1
        else:
            f, l = first, last
            palindrome = True
            while f < l and palindrome:
                palindrome = col_keys[f] == col_keys[l]
                f += 1
                l -= 1
            if palindrome:
                return 1 + first + ((last - first) >> 1)
            first += 1
    col_keys.reverse()
    first, last = 0, len(col_keys) - 1
    while first != last:
        if col_keys[first] != col_keys[last] or not ((last - first) & 1):
            first += 1
        else:
            f, l = first, last
            palindrome = True
            iterated = False
            while f < l and palindrome:
                palindrome = col_keys[f] == col_keys[l]
                f += 1
                l -= 1
                iterated = True
            if palindrome:
                return row_length - (1 + first + ((last - first) >> 1))
            first += 1
    return 0

def mirror_positions_row(grid: list[bool], row_length: int, col_length: int) -> int:
    row_keys = [
        sum(grid[i * row_length + j] << j for j in range(row_length))
        for i in range(col_length)
    ]
    first, last = 0, len(row_keys) - 1
    while first < last:
        if row_keys[first] != row_keys[last] or not ((last - first) & 1):
            first += 1
        else:
            f, l = first, last
            palindrome = True
            iterated = False
            while f < l and palindrome:
                palindrome = row_keys[f] == row_keys[l]
                f += 1
                l -= 1
                iterated = True
            if palindrome and iterated:
                return 1 + first + ((last - first) >> 1)
            first += 1
    row_keys.reverse()
    first, last = 0, len(row_keys) - 1
    while first != last:
        if row_keys[first] != row_keys[last] or not ((last - first) & 1):
            first += 1
        else:
            f, l = first, last
            palindrome = True
            iterated = False
            while f < l and palindrome:
                palindrome = row_keys[f] == row_keys[l]
                f += 1
                l -= 1
                iterated = True
            if palindrome:
                return col_length - (1 + first + ((last - first) >> 1))
            first += 1
    return 0

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
                r = mirror_positions_col(grid, row, col)
                if r == 0:
                    r = 100 * mirror_positions_row(grid, row, col)
                row, col = 0, 0
                grid = []
                s += r
    r = mirror_positions_col(grid, row, col)
    if r == 0:
        r = 100 * mirror_positions_row(grid, row, col)
    row, col = 0, 0
    grid = []
    s += r
    #print('col' if odd else 'row', r)
    return s

def part2(filename: str) -> int:
    return 0

if __name__ == '__main__':
    print(part1('day13_input.txt'))
    print(part2('test_input.txt'))