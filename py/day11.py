from functools import lru_cache


def parse_grid(lines):
    grid = {}
    for line in lines:
        node, children = line.strip().split(":")
        grid[node] = children.split()
    return grid


def part1(start, end, grid):
    @lru_cache
    def dfs(node, end):
        if node == end:
            return 1
        dist = sum(dfs(c, end) for c in grid[node])
        return dist

    return dfs(start, end)


def count_paths_both(start, end, grid):
    @lru_cache(None)
    def dfs(node, seen_fft, seen_dac):
        if node == "fft":
            seen_fft = True
        if node == "dac":
            seen_dac = True
        if node == end:
            return 1 if (seen_fft and seen_dac) else 0

        return sum(dfs(child, seen_fft, seen_dac) for child in grid[node])

    return dfs(start, False, False)


def part2(grid):
    return count_paths_both("svr", "out", grid)


def main():
    with open("puzzles/day11.txt", "r") as f:
        lines = f.readlines()

    g = parse_grid(lines)
    g["out"] = []

    print("Part 1: ", part1("you", "out", g))
    print("Part 2: ", part2(g))


if __name__ == "__main__":
    main()
