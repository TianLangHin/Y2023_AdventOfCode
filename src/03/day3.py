from collections import namedtuple

# symbols: list[int], numbers: dict[int,int]
LineResult = namedtuple('LineResult', ['symbols', 'numbers'])

def parse_line1(line: str) -> list[LineResult]:
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
            case _:
                if current_number != 0:
                    numbers[current_index] = current_number
                    current_number = 0
                    current_index = None
                symbols.append(i)
    if current_number != 0:
        numbers[current_index] = current_number
    return LineResult(symbols, numbers)

def parse_line2(line: str) -> list[LineResult]:
    gears: list[int] = []
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
            case '*':
                if current_number != 0:
                    numbers[current_index] = current_number
                    current_number = 0
                    current_index = None
                gears.append(i)
            case _:
                if current_number != 0:
                    numbers[current_index] = current_number
                    current_number = 0
                    current_index = None
    if current_number != 0:
        numbers[current_index] = current_number
    return LineResult(gears, numbers)

def part1(filename: str):
    digits = lambda s: len(str(s))
    running_sum = 0
    before = LineResult([], {})
    current = LineResult([], {})
    after = LineResult([], {})
    with open(filename, 'rt') as f:
        for line in f:
            after = parse_line1(line.strip())
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
    print(running_sum)

def part2(filename: str):
    digits = lambda s: len(str(s))
    running_sum = 0
    before = LineResult([], {})
    current = LineResult([], {})
    after = LineResult([], {})
    with open(filename, 'rt') as f:
        for line in f:
            after = parse_line2(line.strip())
            for index in current.symbols:
                adjacencies = []
                adjacencies.extend(value for key, value in before.numbers.items() if key - 1 <= index <= key + digits(value))
                adjacencies.extend(value for key, value in current.numbers.items() if key - 1 <= index <= key + digits(value))
                adjacencies.extend(value for key, value in after.numbers.items() if key - 1 <= index <= key + digits(value))
                if len(adjacencies) == 2:
                    running_sum += adjacencies[0] * adjacencies[1]
            before = current
            current = after
        for index in current.symbols:
            adjacencies = []
            adjacencies.extend(value for key, value in before.numbers.items() if key - 1 <= index <= key + digits(value))
            adjacencies.extend(value for key, value in current.numbers.items() if key - 1 <= index <= key + digits(value))
            if len(adjacencies) == 2:
                print(adjacencies)
                running_sum += adjacencies[0] * adjacencies[1]
    print(running_sum)

if __name__ == '__main__':
    part1('day3_input.txt')
    part2('day3_input.txt')