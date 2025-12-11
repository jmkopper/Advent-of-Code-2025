import re
from collections import deque

import numpy as np
from scipy.optimize import linprog


def parse_line(line):
    raw = re.search(r"\[(.*?)\]", line).group(1)
    lights = [1 if c == "#" else 0 for c in raw]
    buttons = [
        tuple(map(int, g.split(","))) if g else ()
        for g in re.findall(r"\((.*?)\)", line)
    ]
    joltages = list(map(int, re.search(r"\{(.*?)\}", line).group(1).split(",")))

    # e.g. [0, 0, 1], [(1, 2), (0)], [1,2,3]
    return lights, buttons, joltages


def neighbors(lights, buttons):
    res = []
    for button in buttons:
        n = lights.copy()
        for b in button:
            n[b] = 1 - n[b]
        res.append(n)
    return res


def shortest_path(lights, buttons):
    n_lights = len(lights)
    target = tuple(lights)

    root = [0 for i in range(n_lights)]
    visited = {tuple(root)}
    q = deque([root])
    level = 0

    while q:
        level_size = len(q)
        while level_size > 0:
            u = q.popleft()
            if tuple(u) == target:
                return level
            for n in neighbors(u, buttons):
                s = tuple(n)
                if s not in visited:
                    q.append(n)
                    visited.add(s)
            level_size -= 1
        level += 1


def solve_ilp(buttons, joltages):
    num_vars = len(buttons)
    num_eqs = len(joltages)

    # linear constraint: button presses must give the
    # target joltages, i.e. Ax = b
    # x_i = number of times button i is pressed
    A = np.zeros((num_eqs, num_vars))
    for c, btn in enumerate(buttons):
        for r in btn:
            A[r, c] = 1
    b = np.array(joltages)

    c_vec = np.ones(num_vars)  # minimize sum(x_i)
    res = linprog(
        c_vec,
        A_eq=A,
        b_eq=b,
        bounds=(0, None),  # x_i >= 0
        integrality=1,
    )
    presses = np.round(res.x).astype(int)
    return int(np.sum(presses))


def part1(parsed_lines):
    return sum(shortest_path(l, b) for l, b, _ in parsed_lines)


def part2(parsed_lines):
    return sum(solve_ilp(b, j) for _, b, j in parsed_lines)


def main():
    with open("puzzles/day10.txt", "r") as f:
        lines = f.readlines()

    t = [parse_line(x) for x in lines]
    print("Part 1: ", part1(t))
    print("Part 2: ", part2(t))

    # print(b)
    # print(shortest_path2(j, b))


if __name__ == "__main__":
    main()
