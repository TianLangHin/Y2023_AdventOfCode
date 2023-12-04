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

def part2(filename: str) -> int:
    with open(filename, 'rt') as f:
        s = 0
        bonuses = {}
        for line in f:
            card, data = line.split(':')
            _, card_no = card.split()
            card_no = int(card_no)
            wins, have = [x.strip() for x in data.split('|')]
            wins = [int(x) for x in wins.split()]
            have = sum(1 for x in have.split() if int(x) in wins)
            for i in range(1, have+1):
                bonuses[card_no + i] = bonuses.get(card_no + i, 0) + bonuses.get(card_no, 0) + 1
            s += bonuses.get(card_no, 0) + 1
            if card_no in bonuses:
                bonuses.pop(card_no)
        return s

if __name__ == '__main__':
    print(part1('day4_input.txt'))
    print(part2('day4_input.txt'))