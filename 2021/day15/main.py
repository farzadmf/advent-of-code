from heapq import (
    heappush,
    heappop,
)
import sys
from typing import (
    List,
)


def read_input(input_file: str) -> List[str]:
    with open(input_file) as f:
        return f.read().splitlines()


def repeat(row: List[int]) -> List[int]:
    res = []
    for r in row:
        res.append(1 if r == 9 else r + 1)
    return res


class Grid:
    def __init__(self, lines: List[str]):
        self.rows = len(lines)
        self.cols = len(lines[0])
        self.cells = [[0] * self.cols for _ in range(self.rows)]
        self.end = (self.rows - 1, self.cols - 1)

        for r, l in enumerate(lines):
            for c, char in enumerate(l):
                self.cells[r][c] = int(char)

    def expand(self):
        for r in self.cells:
            rep = r
            for _ in range(4):
                res = repeat(rep)
                r += res
                rep = res
        self.cols *= 5
        for i in range(4):
            for r in range(self.rows):
                row = self.cells[r + (self.rows * i)]
                self.cells.append(repeat(row))
        self.rows *= 5
        self.end = (self.rows - 1, self.cols - 1)

    def find_lowest_risk_bfs(self) -> int:
        q = [(0, (0, 0))]
        dirs = [(0, 1), (1, 0), (0, -1), (-1, 0)]
        while q:
            cost, (r, c) = heappop(q)
            if self.cells[r][c] == -1:
                continue
            self.cells[r][c] = -1
            if (r, c) == self.end:
                return cost
            for dr, dc in dirs:
                nr, nc = r + dr, c + dc
                if not (0 <= nr < self.rows and 0 <= nc < self.cols):
                    continue
                heappush(q, (cost + self.cells[nr][nc], (nr, nc)))
        return -1


def main(input_file: str) -> None:
    lines = read_input(input_file)
    grid = Grid(lines)
    grid.expand()
    print(grid.find_lowest_risk_bfs())


if __name__ == "__main__":
    main(sys.argv[1])
