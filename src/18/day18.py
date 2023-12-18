def calculate_fill(instructions: list[str, int]) -> int:
    x, y = 0, 0
    min_x, min_y = 0, 0
    for direction, length in instructions:
        match direction:
            case 'R':
                x += length
            case 'D':
                y += length
            case 'L':
                x -= length
                if x < min_x:
                    min_x = x
            case 'U':
                y -= length
                if y < min_y:
                    min_y = y
    x, y = -min_x, -min_y
    total_area = 0
    perimeter = 0
    for direction, length in instructions:
        match direction:
            case 'R':
                x += length
                perimeter += length
            case 'L':
                x -= length
            case 'D':
                y += length
                perimeter += length
                total_area += x * length
            case 'U':
                y -= length
                total_area -= x * length
    return perimeter + total_area + 1

def part1(filename: str) -> int:
    instructions = []
    with open(filename, 'rt') as f:
        for line in f:
            direction, magnitude, _ = line.strip().split()
            instructions.append((direction, int(magnitude)))
    return calculate_fill(instructions)

def part2(filename: str) -> int:
    instructions = []
    with open(filename, 'rt') as f:
        for line in f:
            _, _, hexadecimal = line.strip().split()
            magnitude = int(hexadecimal[2:7], 16)
            match hexadecimal[7]:
                case '0':
                    direction = 'R'
                case '1':
                    direction = 'D'
                case '2':
                    direction = 'L'
                case '3':
                    direction = 'U'
            instructions.append((direction, magnitude))
    return calculate_fill(instructions)

if __name__ == '__main__':
    print(part1('day18_input.txt'))
    print(part2('day18_input.txt'))