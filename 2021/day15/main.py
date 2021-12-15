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
        self.dp = []

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

    def find_lowest_risk(self) -> int:
        self.dp = [row[:] for row in self.cells]
        self.dp[0][0] = 0

        for r in range(1, self.rows):
            self.dp[r][0] += self.dp[r - 1][0]
        for c in range(1, self.cols):
            self.dp[0][c] += self.dp[0][c - 1]

        for r in range(1, self.rows):
            for c in range(1, self.cols):
                up, left = self.dp[r - 1][c], self.dp[r][c - 1]
                self.dp[r][c] += min(up, left)

        return self.dp[-1][-1]


def main(input_file: str) -> None:
    lines = read_input(input_file)
    grid = Grid(lines)
    grid.expand()
    print(grid.find_lowest_risk())


if __name__ == "__main__":
    main(sys.argv[1])
