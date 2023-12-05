from functools import reduce

def transform(source: list[int], *, mapping: list[tuple[int,int,int]]) -> None:
    converted = [False] * len(source)
    for end, start, length in mapping:
        for i in range(len(source)):
            if not converted[i] and start <= source[i] < start + length:
                source[i] += end - start
                converted[i] = True
    # implicitly leaves unchanged cells intact.
    return

def part1(filename: str) -> int:
    seeds: list[int] = []

    transformations: dict[str,list[tuple[int,int,int]]] = {
        'seed-to-soil': [],
        'soil-to-fertilizer': [],
        'fertilizer-to-water': [],
        'water-to-light': [],
        'light-to-temperature': [],
        'temperature-to-humidity': [],
        'humidity-to-location': [],
    }

    key = None
    with open(filename, 'rt') as f:
        for file_line in f:
            match file_line.strip():
                case '':
                    pass
                case line if line.startswith('seeds:'):
                    seeds.extend(int(x) for x in line[7:].split())
                case line if line.endswith('map:'):
                    key = line[:-5]
                case line:
                    transformations[key].append(tuple(int(x) for x in line.split()))

    for transformation in transformations:
        transform(seeds, mapping=transformations[transformation])

    return min(seeds)


def part2(filename: str) -> int:
    return 0

if __name__ == '__main__':
    print(part1('day5_input.txt'))
    print(part2('day5_input.txt'))