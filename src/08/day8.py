from collections import namedtuple
from functools import reduce

# name: str, left: str, right: str
Node = namedtuple('Node', ['name', 'left', 'right'])

def execute_steps(start_node: Node, path: list[bool], *, nodes: dict[str, Node]) -> Node:
    node = start_node
    for branch in path:
        node = nodes[node.left] if branch else nodes[node.right]
    return node

def parse_input(filename: str) -> (list[bool], dict[str, Node]):
    steps = []
    nodes: dict[str, Node] = {}
    with open(filename, 'rt') as f:
        for line in f:
            line = line.strip()
            if not steps:
                # True is Left, False is Right
                steps = [c == 'L' for c in line]
            elif line == '':
                pass
            else:
                cur, l, r = (lambda x: (x[0], x[1][0], x[1][1]))(
                    (lambda x: (x[0], x[1][1:-1].split(', ')))(
                    line.split(' = ')))
                nodes[cur] = Node(cur, l, r)
    return steps, nodes

def part1(filename: str) -> int:
    steps, nodes = parse_input(filename)
    s = 0
    node = nodes['AAA']
    while node.name != 'ZZZ':
        node = execute_steps(node, steps, nodes=nodes)
        s += 1
    return s * len(steps)

def gcd(a, b):
    if a < b:
        a, b = b, a
    if a % b == 0:
        return b
    return gcd(b, a % b)

# Assumes that all starting nodes are in a cycle that contains an endpoint,
# and that the end state is reached after an integer multiple of strides given.
def part2(filename: str) -> int:
    steps, nodes = parse_input(filename)

    transforms: dict[str, str] = {}
    for node in nodes.keys():
        transforms[node] = execute_steps(nodes[node], steps, nodes=nodes).name

    end_cycles: dict[str, list[str]] = {}
    for node in nodes.keys():
        if node.endswith('Z'):
            init_node = node
            cycle = [node]
            while True:
                node = transforms[node]
                if node == init_node:
                    break
                cycle.append(node)
            end_cycles[node] = cycle

    guarantees: dict[str, tuple[int, int]] = {}
    for end_node in end_cycles.keys():
        e = end_cycles[end_node]
        for i in range(len(e)):
            if guarantees.get(e[i], None) is None:
                guarantees[e[i]] = len(e)

    results = []
    for node in nodes.keys():
        if node.endswith('A'):
            s = 0
            while (x := guarantees.get(node, -1)) == -1:
                node = transforms[node]
                s += 1
            results.append(x)

    return len(steps) * reduce(lambda acc, x: acc * x // gcd(acc, x), results)

if __name__ == '__main__':
    print(part1('day8_input.txt'))
    print(part2('day8_input.txt'))