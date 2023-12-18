def part1(filename: str) -> int:
    instructions = []
    with open(filename, 'rt') as f:
        for line in f:
            direction, magnitude, _ = line.strip().split()
            instructions.append((direction, int(magnitude)))
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

if __name__ == '__main__':
    print(part1('day18_input.txt'))