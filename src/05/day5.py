from collections import namedtuple

############
# Part One #
############

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

############
# Part Two #
############

# start: int, end: int
# `start` and `end` are inclusive.
Range = namedtuple('Range', ['start', 'end'])
# start_range: Range, offset: int
Mapping = namedtuple('Mapping', ['start_range', 'offset'])

def transform_ranges(a: list[Range], mapper: list[Mapping]) -> list[Range]:
    top_item = None
    i, j = 0, 0
    result_ranges: list[Range] = []
    while i < len(a) and j < len(mapper):
        top_item = a[i] if top_item is None else top_item
        b, offset = mapper[j]
        if top_item.start < b.start:
            if top_item.end > b.end:
                result_ranges.append(Range(top_item.start, b.start - 1))
                result_ranges.append(Range(b.start + offset, b.end + offset))
                top_item = Range(b.end + 1, top_item.end)
                j += 1
            elif b.start <= top_item.end <= b.end:
                result_ranges.append(Range(top_item.start, b.start - 1))
                result_ranges.append(Range(b.start + offset, top_item.end + offset))
                top_item = None
                i += 1
            elif top_item.end < b.start:
                result_ranges.append(Range(top_item.start, top_item.end))
                top_item = None
                i += 1
            else:
                raise Exception("shouldn't happen 02 {} {}".format(top_item, b))
        elif top_item.start >= b.start:
            if top_item.start > b.end:
                top_item = None
                j += 1
            elif top_item.end > b.end:
                result_ranges.append(Range(top_item.start + offset, b.end + offset))
                top_item = Range(b.end + 1, top_item.end)
                j += 1
            elif top_item.end <= b.end:
                result_ranges.append(Range(top_item.start + offset, top_item.end + offset))
                top_item = None
                i += 1
            else:
                raise Exception("shouldn't happen 03 {} {}".format(top_item, b))
        else:
            raise Exception("shouldn't happen 01 {} {}".format(top_item, b))
    while i < len(a):
        result_ranges.append(a[i])
        i += 1
    result_ranges.sort(key=lambda r: r.start)
    return result_ranges

def part2(filename: str) -> int:
    seeds: list[Range] = []

    transformations: dict[str,list[Mapping]] = {
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
                    line = [int(x) for x in line[7:].split()]
                    for i in range(len(line) // 2):
                        seeds.append(Range(line[2*i], line[2*i] + line[2*i+1] - 1))
                case line if line.endswith('map:'):
                    key = line[:-5]
                case line:
                    end, start, length = [int(x) for x in line.split()]
                    transformations[key].append(
                        Mapping(
                            Range(start, start + length - 1),
                            end-start
                        )
                    )
                    transformations[key].sort(key=lambda m: m.start_range.start)

    seeds.sort(key=lambda r: r.start)
    for t in transformations:
        seeds = transform_ranges(seeds, transformations[t])
    return seeds[0].start

if __name__ == '__main__':
    print(part1('day5_input.txt'))
    print(part2('day5_input.txt'))