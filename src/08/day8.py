from collections import namedtuple
from functools import reduce

##########
# Part 1 #
##########

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
                cur, nxt = line.split(' = ')
                l, r = nxt.strip('()').split(', ')
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

##########
# Part 2 #
##########

# offset: int, period: int
Cycle = namedtuple('Cycle', ['offset', 'period'])

# Returns a 2-element tuple.
# 1st element is the first index where the pointers overlap,
# 2nd element is the period of the cycles after first overlap.
def overlap_cycle(arg1: Cycle, arg2: Cycle) -> Cycle:
    # We aim to find the first value X = p + as = q + bt
    p, a = arg1
    q, b = arg2
    if a > b:
        a, b = b, a
        p, q = q, p
    # Use extended Euclidean algorithm.
    old_r, r = a, b
    old_s, s = 1, 0
    old_t, t = 0, 1
    while r:
        quotient, remainder = divmod(old_r, r)
        old_r, r = r, remainder
        old_s, s = s, old_s - quotient * s
        old_t, t = t, old_t - quotient * t
    lcm = a * b // old_r
    if q < p:
        q += lcm
    factor, mismatch = divmod(q - p, old_r)
    # If the cycle periods share a factor, not all permutations will be possible
    # due to synchronisation at offsetted positions.
    if mismatch != 0:
        raise Exception('no possibility of cycle match')
    old_r, old_s, old_t = old_r * factor, old_s * factor, old_t * factor
    old_s = old_s % b
    if old_s <= 0:
        old_s += b
    return Cycle(p + a * old_s, lcm)

# Assumes that all starting nodes can reach a cycle that contains an endpoint,
# and that the end state is never reached in the middle of a stride.
# Also assumes that one endpoint does not contain another endpoint in its stride-cycle.
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
                guarantees[e[i]] = (i, len(e))

    results = []
    for node in nodes.keys():
        if node.endswith('A'):
            s = 0
            while (x := guarantees.get(node, None)) is None:
                node = transforms[node]
                s += 1
            results.append(Cycle((s + x[1] - x[0]) % x[1], x[1]))

    return len(steps) * reduce(lambda acc, x: overlap_cycle(acc, x), results)[0]

if __name__ == '__main__':
    print(part1('day8_input.txt'))
    print(part2('day8_input.txt'))