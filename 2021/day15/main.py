import sys
from typing import (
    List,
)


def read_input(input_file: str) -> List[str]:
    with open(input_file) as f:
        return f.read().splitlines()


class Grid:
    def __init__(self, lines: List[str]):
        self.rows = len(lines)
        self.cols = len(lines[0])
        self.cells = [[0] * self.cols for _ in range(self.rows)]
        self.start = (0, 0)
        self.end = (self.rows - 1, self.cols - 1)
        for r, l in enumerate(lines):
            for c, char in enumerate(l):
                self.cells[r][c] = int(char)

    def find_lowest_risk(self) -> int:
        dp = self.cells.copy()
        dp[0][0] = 0

        for r in range(1, self.rows):
            dp[r][0] += dp[r - 1][0]
        for c in range(1, self.cols):
            dp[0][c] += dp[0][c - 1]

        for r in range(1, self.rows):
            for c in range(1, self.cols):
                up, left = dp[r - 1][c], dp[r][c - 1]
                dp[r][c] += min(up, left)

        return dp[-1][-1]


def main(input_file: str) -> None:
    lines = read_input(input_file)
    grid = Grid(lines)
    print(grid.find_lowest_risk())


if __name__ == "__main__":
    main(sys.argv[1])
