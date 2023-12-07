from collections import namedtuple

# hand_type: int, cards: tuple[int,int,int,int,int]
Hand = namedtuple('Hand', ['hand_type', 'cards'])

def make_type(card_counter: dict[str,int], part_two: bool) -> int:
    if part_two:
        joker_count = card_counter.get('J', 0)
        if joker_count:
            card_counter.pop('J')
        freq = list(card_counter.values())
        freq.sort(reverse=True)
        if freq:
            freq[0] += joker_count
        else:
            freq.append(5)
    else:
        freq = list(card_counter.values())
    if freq.count(5) == 1:
        return 6
    elif freq.count(4) == 1:
        return 5
    elif freq.count(3) == 1 and freq.count(2) == 1:
        return 4
    elif freq.count(3) == 1 and freq.count(1) == 2:
        return 3
    elif freq.count(2) == 2:
        return 2
    elif freq.count(2) == 1 and freq.count(1) == 3:
        return 1
    return 0

def part1(filename: str) -> int:
    hands: list[tuple[Hand,int]] = []
    card_map = {
        'A': 14,
        'K': 13,
        'Q': 12,
        'J': 11,
        'T': 10,
        '9': 9,
        '8': 8,
        '7': 7,
        '6': 6,
        '5': 5,
        '4': 4,
        '3': 3,
        '2': 2,
    }
    with open(filename, 'rt') as f:
        for line in f:
            cards, bid = line.split()
            card_counter = {}
            card_list = []
            for card in cards:
                card_counter[card] = card_counter.get(card, 0) + 1
                card_list.append(card_map[card])
            hands.append((Hand(make_type(card_counter, False), tuple(card_list)), int(bid)))
    hands.sort(key=lambda x: x[0])
    return sum((i+1)*bid for i, (_, bid) in enumerate(hands))

def part2(filename: str) -> int:
    hands: list[tuple[Hand,int]] = []
    card_map = {
        'A': 14,
        'K': 13,
        'Q': 12,
        'T': 10,
        '9': 9,
        '8': 8,
        '7': 7,
        '6': 6,
        '5': 5,
        '4': 4,
        '3': 3,
        '2': 2,
        'J': 1,
    }
    with open(filename, 'rt') as f:
        for line in f:
            cards, bid = line.split()
            card_counter = {}
            card_list = []
            for card in cards:
                card_counter[card] = card_counter.get(card, 0) + 1
                card_list.append(card_map[card])
            hands.append((Hand(make_type(card_counter, True), tuple(card_list)), int(bid)))
    hands.sort(key=lambda x: x[0])
    return sum((i+1)*bid for i, (_, bid) in enumerate(hands))

if __name__ == '__main__':
    print(part1('day7_input.txt'))
    print(part2('day7_input.txt'))