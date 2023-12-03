from collections import namedtuple

# symbols: list[int], numbers: dict[int,int]
LineResult = namedtuple('LineResult', ['symbols', 'numbers'])

# Returns "gears" only if mode is True, all symbols otherwise
def parse_line(line: str, mode: bool) -> LineResult:
    symbols: list[int] = []
    numbers: dict[int,int] = {}
    current_number = 0
    current_index = None
    for i in range(len(line)):
        match line[i]:
            case '.':
                if current_number != 0:
                    numbers[current_index] = current_number
                    current_number = 0
                    current_index = None
            case c if c.isnumeric():
                if current_number == 0:
                    current_index = i
                current_number = current_number * 10 + int(c)
            case '*' if mode:
                if current_number != 0:
                    numbers[current_index] = current_number
                    current_number = 0
                    current_index = None
                symbols.append(i)
            case _:
                if current_number != 0:
                    numbers[current_index] = current_number
                    current_number = 0
                    current_index = None
                if not mode:
                    symbols.append(i)
    if current_number != 0:
        numbers[current_index] = current_number
    return LineResult(symbols, numbers)

def part1(filename: str) -> int:
    digits = lambda s: len(str(s))
    running_sum = 0
    before = LineResult([], {})
    current = LineResult([], {})
    after = LineResult([], {})
    with open(filename, 'rt') as f:
        for line in f:
            after = parse_line(line.strip(), False)
            adjacencies = set()
            for index in current.numbers.keys():
                if any(index - 1 <= x <= index + digits(current.numbers[index]) for x in after.symbols):
                    adjacencies.add(index)
                if any(index - 1 <= x <= index + digits(current.numbers[index]) for x in current.symbols):
                    adjacencies.add(index)
                if any(index - 1 <= x <= index + digits(current.numbers[index]) for x in before.symbols):
                    adjacencies.add(index)
            running_sum += sum(current.numbers[i] for i in adjacencies)
            before = current
            current = after
        adjacencies = set()
        for index in current.numbers.keys():
            if any(index - 1 <= x <= index + digits(current.numbers[index]) for x in current.symbols):
                adjacencies.add(index)
            if any(index - 1 <= x <= index + digits(current.numbers[index]) for x in before.symbols):
                adjacencies.add(index)
        running_sum += sum(current.numbers[i] for i in adjacencies)
    return running_sum

def part2(filename: str) -> int:
    digits = lambda s: len(str(s))
    running_sum = 0
    before = LineResult([], {})
    current = LineResult([], {})
    after = LineResult([], {})
    with open(filename, 'rt') as f:
        adjacencies = []
        for line in f:
            after = parse_line(line.strip(), True)
            for index in current.symbols:
                adjacencies.clear()
                adjacencies.extend(value for key, value in before.numbers.items() if key - 1 <= index <= key + digits(value))
                adjacencies.extend(value for key, value in current.numbers.items() if key - 1 <= index <= key + digits(value))
                adjacencies.extend(value for key, value in after.numbers.items() if key - 1 <= index <= key + digits(value))
                if len(adjacencies) == 2:
                    running_sum += adjacencies[0] * adjacencies[1]
            before = current
            current = after
        for index in current.symbols:
            adjacencies.clear()
            adjacencies.extend(value for key, value in before.numbers.items() if key - 1 <= index <= key + digits(value))
            adjacencies.extend(value for key, value in current.numbers.items() if key - 1 <= index <= key + digits(value))
            if len(adjacencies) == 2:
                print(adjacencies)
                running_sum += adjacencies[0] * adjacencies[1]
    return running_sum

if __name__ == '__main__':
    print(part1('day3_input.txt'))
    print(part2('day3_input.txt'))