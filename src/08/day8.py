from collections import namedtuple

# name: str, left: str, right: str
Node = namedtuple('Node', ['name', 'left', 'right'])

def execute_steps(start_node: Node, path: list[bool], *, nodes: dict[str, Node]) -> Node:
    node = start_node
    for branch in path:
        node = nodes[node.left] if branch else nodes[node.right]
    return node

def part1(filename: str) -> int:
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
    s = 0
    node = nodes['AAA']
    while node.name != 'ZZZ':
        node = execute_steps(node, steps, nodes=nodes)
        s += 1
    return s * len(steps)

def part2(filename: str) -> int:
    pass

if __name__ == '__main__':
    print(part1('day8_input.txt'))
    print(part2('day8_input.txt'))