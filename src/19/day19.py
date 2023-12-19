from collections import namedtuple

# x: int, m: int, a: int, s: int
Part = namedtuple('Part', ['x', 'm', 'a', 's'])

# component: str, value: int, less_than: bool, destination: str
Condition = namedtuple('Condition', ['component', 'value', 'less_than', 'destination'])

# conditions: list[Condition], end_state: str
Workflow = namedtuple('Workflow', ['conditions', 'end_state'])

def execute_steps(workflows: dict[str, Workflow], part: Part) -> bool:
    current_workflow = workflows['in']
    while current_workflow is not None:
        redirect = None
        for step in current_workflow.conditions:
            match step.component:
                case 'x':
                    component = part.x
                case 'm':
                    component = part.m
                case 'a':
                    component = part.a
                case 's':
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
                            conditions.append(Condition(component, int(value), True, dest))
                        else:
                            component, value = cmd.split('>')
                            conditions.append(Condition(component, int(value), False, dest))
                    workflows[name] = Workflow(conditions, final)
            else:
                blank_passed = True
    s = 0
    for part in parts:
        if execute_steps(workflows, part):
            s += part.x + part.m + part.a + part.s
    return s

if __name__ == '__main__':
    print(part1('day19_input.txt'))