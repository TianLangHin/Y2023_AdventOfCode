def part1():
    s = 0
    with open('day2_input.txt', 'rt') as f:
        for line in f:
            game_no, games = line.split(':')
            game_no = int(game_no[5:])
            cubes = {'blue': 0, 'red': 0, 'green': 0}
            for game in games.split(';'):
                for category in game.split(','):
                    num, colour = category.split()
                    num = int(num)
                    cubes[colour.strip()] = max(cubes[colour.strip()], num)
            if cubes['red'] <= 12 and cubes['green'] <= 13 and cubes['blue'] <= 14:
                s += game_no
    print(s)

def part2():
    s = 0
    with open('day2_input.txt', 'rt') as f:
        for line in f:
            game_no, games = line.split(':')
            game_no = int(game_no[5:])
            cubes = {'blue': 0, 'red': 0, 'green': 0}
            for game in games.split(';'):
                for category in game.split(','):
                    num, colour = category.split()
                    num = int(num)
                    cubes[colour.strip()] = max(cubes[colour.strip()], num)
            s += cubes['blue'] * cubes['red'] * cubes['green']
    print(s)

if __name__ == '__main__':
    part1()
    part2()