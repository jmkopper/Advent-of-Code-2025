import math
import re

from scipy.spatial import KDTree


def dist2(a, b):
    return sum((x - y) ** 2 for x, y in zip(a, b))


def part1_kdtree(pts, num_iter=1000):
    kdtree = KDTree(pts)  # cannot be bothered to implement this on my own
    connections = {p: set() for p in pts}

    for _ in range(num_iter):
        best = None
        best_d = float("inf")

        for p in pts:
            # increase k until we find someone not directly connected
            k = 1
            while True:
                _, idx = kdtree.query(p, k=k)
                cand = pts[idx[-1]] if k > 1 else pts[idx]

                if cand != p and cand not in connections[p]:
                    d2 = dist2(p, cand)
                    if d2 < best_d:
                        best = (p, cand)
                        best_d = d2
                    break

                k += 1

        assert isinstance(best, tuple)  # shutup, linter
        x, y = best

        connections[x].add(y)
        connections[y].add(x)

    return connections


def part2_kdtree(pts):
    kdtree = KDTree(pts)  # cannot be bothered to implement this on my own
    connections = {p: set() for p in pts}
    circuits = [set([x]) for x in pts]

    # for _ in range(10):
    while len(circuits) > 1:
        print(len(circuits))
        best = None
        best_d = float("inf")

        for p in pts:
            # increase k until we find someone not directly connected
            k = 1
            while True:
                _, idx = kdtree.query(p, k=k)
                cand = pts[idx[-1]] if k > 1 else pts[idx]

                if cand != p and cand not in connections[p]:
                    d2 = dist2(p, cand)
                    if d2 < best_d:
                        best = (p, cand)
                        best_d = d2
                    break

                k += 1

        assert isinstance(best, tuple)  # shutup, linter
        x, y = best

        # add direct connections
        connections[x].add(y)
        connections[y].add(x)

        # merge circuits
        idx = next(i for i in range(len(circuits)) if x in circuits[i])
        idy = next(i for i in range(len(circuits)) if y in circuits[i])
        if idx != idy:
            if idx > idy:
                idx, idy = idy, idx
            circuits[idx] = circuits[idx].union(circuits[idy])
            del circuits[idy]

    return circuits, connections, x, y


def part1(lines):
    circuits, connections, x, y = part2_kdtree(lines)
    print(circuits)
    print(x, y)
    lens = sorted([len(x) for x in circuits], reverse=True)
    print(x[0] * y[0])
    return math.prod(lens[:3])
    # connections = part1_kdtree(lines)
    # # # connections : dict {point -> set(point)}

    # visited = set()
    # circuits = []

    # def dfs(start):
    #     stack = [start]
    #     comp = set()
    #     while stack:
    #         x = stack.pop()
    #         if x in visited:
    #             continue
    #         visited.add(x)
    #         comp.add(x)
    #         for y in connections[x]:
    #             if y not in visited:
    #                 stack.append(y)
    #     return comp

    # # Build connected components
    # for p in connections:
    #     if p not in visited:
    #         circuits.append(dfs(p))

    # # Sizes sorted
    # lens = sorted([len(c) for c in circuits], reverse=True)
    # print(lens)
    # print(circuits)

    # return math.prod(lens[:3])


def main():
    with open("puzzles/day8.txt", "r") as f:
        lines = [tuple(int(x) for x in re.findall(r"\d+", s)) for s in f.readlines()]
        # circuits = {x: i for i, x in enumerate(lines)}
        # print(part1(circuits))
        print(part1(lines))


if __name__ == "__main__":
    main()
