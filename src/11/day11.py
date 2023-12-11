def part1(filename: str) -> int:
    empty_rows: set[int] = set()
    full_columns: set[int] = set()
    number_rows = 0
    number_columns = 0
    galaxies: list[int] = []
    with open(filename, 'rt') as f:
        for line in f:
            line = line.strip()
            number_columns = len(line)
            galaxies_in_row = False
            for i in range(number_columns):
                if line[i] == '#':
                    galaxies.append(number_columns * number_rows + i)
                    galaxies_in_row = True
                    full_columns.add(i)
            if not galaxies_in_row:
                empty_rows.add(number_rows)
            number_rows += 1
    empty_columns = set(range(number_columns)).difference(full_columns)
    s = 0
    for i in range(len(galaxies)):
        for j in range(i+1, len(galaxies)):
            a_row, a_col = divmod(galaxies[i], number_columns)
            b_row, b_col = divmod(galaxies[j], number_columns)
            if a_col > b_col:
                a_col, b_col = b_col, a_col
            if a_row > b_row:
                a_row, b_row = b_row, a_row
            expanded_rows = len(empty_rows.intersection(set(range(a_row, b_row))))
            expanded_columns = len(empty_columns.intersection(set(range(a_col, b_col))))
            row_path_length = b_col - a_col
            col_path_length = b_row - a_row
            s += (expanded_rows + row_path_length) + (expanded_columns + col_path_length)
    return s

def part2(filename: str) -> int:
    empty_rows: set[int] = set()
    full_columns: set[int] = set()
    number_rows = 0
    number_columns = 0
    galaxies: list[int] = []
    with open(filename, 'rt') as f:
        for line in f:
            line = line.strip()
            number_columns = len(line)
            galaxies_in_row = False
            for i in range(number_columns):
                if line[i] == '#':
                    galaxies.append(number_columns * number_rows + i)
                    galaxies_in_row = True
                    full_columns.add(i)
            if not galaxies_in_row:
                empty_rows.add(number_rows)
            number_rows += 1
    empty_columns = set(range(number_columns)).difference(full_columns)
    s = 0
    for i in range(len(galaxies)):
        for j in range(i+1, len(galaxies)):
            a_row, a_col = divmod(galaxies[i], number_columns)
            b_row, b_col = divmod(galaxies[j], number_columns)
            if a_col > b_col:
                a_col, b_col = b_col, a_col
            if a_row > b_row:
                a_row, b_row = b_row, a_row
            expanded_rows = len(empty_rows.intersection(set(range(a_row, b_row))))
            expanded_columns = len(empty_columns.intersection(set(range(a_col, b_col))))
            row_path_length = b_col - a_col
            col_path_length = b_row - a_row
            s += (999999*expanded_rows + row_path_length) + (999999*expanded_columns + col_path_length)
    return s

if __name__ == '__main__':
    print(part1('day11_input.txt'))
    print(part2('day11_input.txt')) 