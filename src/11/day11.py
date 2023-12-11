class Galaxy:
    __slots__ = ('x', 'y')
    def __init__(self, x, y):
        self.x = x
        self.y = y

def expanded_manhattan_sums(filename: str, factor: int) -> int:
    factor -= 1
    galaxies: list[Galaxy] = []
    occupied_columns: set[int] = set()
    number_rows = 0
    number_columns = 0
    row_expansions = 0
    with open(filename, 'rt') as f:
        for line in f:
            line = line.strip()
            number_columns = len(line)
            galaxies_in_row = False
            for i in range(number_columns):
                if line[i] == '#':
                    galaxies.append(Galaxy(i, number_rows + row_expansions * factor))
                    galaxies_in_row = True
                    occupied_columns.add(i)
            if not galaxies_in_row:
                row_expansions += 1
            number_rows += 1
    empty_columns = [i for i in range(number_columns) if i not in occupied_columns]
    for i in range(len(galaxies)):
        column_expansions = 0
        for col in empty_columns:
            if col > galaxies[i].x:
                break
            column_expansions += 1
        galaxies[i].x += column_expansions * factor
    s = 0
    for i in range(len(galaxies)):
        for j in range(i+1, len(galaxies)):
            s += abs(galaxies[j].y - galaxies[i].y) + abs(galaxies[j].x - galaxies[i].x)
    return s

def part1(filename: str) -> int:
    return expanded_manhattan_sums(filename, 2)

def part2(filename: str) -> int:
    return expanded_manhattan_sums(filename, 1000000)

if __name__ == '__main__':
    print(part1('day11_input.txt'))
    print(part2('day11_input.txt')) 