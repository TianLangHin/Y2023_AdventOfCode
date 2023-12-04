def part1(filename: str) -> int:
    with open(filename, 'rt') as f:
        s = 0
        for line in f:
            _, data = line.split(':')
            wins, have = [x.strip() for x in data.split('|')]
            wins = [int(x) for x in wins.split()]
            have = sum(1 for x in have.split() if int(x) in wins)
            if have:
                s += 1 << (have - 1)
        return s

if __name__ == '__main__':
    print(part1('day4_input.txt'))