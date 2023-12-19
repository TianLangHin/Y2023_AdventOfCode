from collections import namedtuple

# x: int, m: int, a: int, s: int
Part = namedtuple('Part', ['x', 'm', 'a', 's'])

# component: int, value: int, less_than: bool, destination: str
Condition = namedtuple('Condition', ['component', 'value', 'less_than', 'destination'])

# conditions: list[Condition], end_state: str
Workflow = namedtuple('Workflow', ['conditions', 'end_state'])

# lower: int, upper: int
Span = namedtuple('Span', ['lower', 'upper'])
# NOTE: all these bounds are exclusive

# xr: list[Span], mr: list[Span], ar: list[Span], sr: list[Span]
Accept = namedtuple('Accept', ['xr', 'mr', 'ar', 'sr'])
# INVARIANT: All lists are in ascending order of `lower`.

def execute_steps(workflows: dict[str, Workflow], part: Part) -> bool:
    current_workflow = workflows['in']
    while current_workflow is not None:
        redirect = None
        for step in current_workflow.conditions:
            match step.component:
                case 0:
                    component = part.x
                case 1:
                    component = part.m
                case 2:
                    component = part.a
                case 3:
                    component = part.s
                case _:
                    raise Exception
            if component < step.value if step.less_than else component > step.value:
                redirect = step.destination
                break
        if redirect is None:
            redirect = current_workflow.end_state
        if redirect == 'A':
            return True
        elif redirect == 'R':
            return False
        current_workflow = workflows.get(redirect, None)
    raise Exception

def part1(filename: str) -> int:
    chars = ['x', 'm', 'a', 's']
    workflows = dict()
    parts = []
    blank_passed = False
    with open(filename, 'rt') as f:
        for line in f:
            line = line.strip()
            if line:
                if blank_passed:
                    components = [int(x.split('=')[1]) for x in line[1:-1].split(',')]
                    parts.append(Part(*components))
                else:
                    name, body = line.split('{')
                    body = body.strip('}').split(',')
                    final = body[-1]
                    conditions = []
                    for body_line in body[:-1]:
                        cmd, dest = body_line.split(':')
                        if '<' in cmd:
                            component, value = cmd.split('<')
                            conditions.append(Condition(chars.index(component), int(value), True, dest))
                        else:
                            component, value = cmd.split('>')
                            conditions.append(Condition(chars.index(component), int(value), False, dest))
                    workflows[name] = Workflow(conditions, final)
            else:
                blank_passed = True
    s = 0
    for part in parts:
        if execute_steps(workflows, part):
            s += part.x + part.m + part.a + part.s
    return s

def intersect(a1: Accept, a2: Accept) -> Accept:
    new_ranges = []
    old_ranges = [(a1.xr, a2.xr), (a1.mr, a2.mr), (a1.ar, a2.ar), (a1.sr, a2.sr)]
    for left, right in old_ranges:
        this_new_range = []
        i, j = 0, 0
        m, n = len(left), len(right)
        while i < m and j < n:
            if left[i].upper <= right[j].lower:
                i += 1
            elif left[i].lower >= right[j].upper:
                j += 1
            else:
                lower = max(left[i].lower, right[j].lower)
                upper = min(left[i].upper, right[j].upper)
                this_new_range.append(Span(lower, upper))
                if left[i].lower < right[j].lower:
                    i += 1
                else:
                    j += 1
        new_ranges.append(this_new_range)
    return Accept(*new_ranges)

def unroll_workflow(name: str, workflows: dict[str, Workflow], unrolled: dict[str, Accept]) -> list[Accept]:
    if (previous_result := unrolled.get(name, None)) is not None:
        return previous_result
    branches = []
    branch_else = Accept([Span(0, 4001)], [Span(0, 4001)], [Span(0, 4001)], [Span(0, 4001)])
    conditions, end_state = workflows[name]
    for c in conditions:
        this_span = Span(0, c.value) if c.less_than else Span(c.value, 4001)
        neg_this_span = Span(c.value-1, 4001) if c.less_than else Span(0, c.value+1)

        acc_list = [[Span(0, 4001)] for _ in range(4)]
        acc_list[c.component] = [this_span]
        this_accept = Accept(*acc_list)

        neg_acc_list = [[Span(0, 4001)] for _ in range(4)]
        neg_acc_list[c.component] = [neg_this_span]
        neg_this_accept = Accept(*neg_acc_list)

        this_accept = intersect(branch_else, this_accept)

        if c.destination == 'A':
            branches.append(this_accept)
        elif c.destination == 'R':
            pass
        else:
            branches.extend(
                intersect(this_accept, branch)
                for branch in unroll_workflow(c.destination, workflows, unrolled)
            )
        branch_else = intersect(branch_else, neg_this_accept)

    if end_state == 'A':
        branches.append(branch_else)
    elif end_state == 'R':
        pass
    else:
        branches.extend(
            intersect(branch_else, branch)
            for branch in unroll_workflow(end_state, workflows, unrolled)
        )
    unrolled[name] = branches
    return branches

def part2(filename: str) -> int:
    chars = ['x', 'm', 'a', 's']
    workflows = dict()
    with open(filename, 'rt') as f:
        for line in f:
            line = line.strip()
            if line:
                name, body = line.split('{')
                body = body.strip('}').split(',')
                final = body[-1]
                conditions = []
                for body_line in body[:-1]:
                    cmd, dest = body_line.split(':')
                    if '<' in cmd:
                        component, value = cmd.split('<')
                        conditions.append(Condition(chars.index(component), int(value), True, dest))
                    else:
                        component, value = cmd.split('>')
                        conditions.append(Condition(chars.index(component), int(value), False, dest))
                workflows[name] = Workflow(conditions, final)
            else:
                break
    unrolled = {}
    s = 0
    for acceptance in unroll_workflow('in', workflows, unrolled):
        x_sum = sum(span.upper - span.lower - 1 for span in acceptance.xr)
        m_sum = sum(span.upper - span.lower - 1 for span in acceptance.mr)
        a_sum = sum(span.upper - span.lower - 1 for span in acceptance.ar)
        s_sum = sum(span.upper - span.lower - 1 for span in acceptance.sr)
        s += x_sum * m_sum * a_sum * s_sum
    return s

if __name__ == '__main__':
    print(part1('day19_input.txt'))
    print(part2('day19_input.txt'))