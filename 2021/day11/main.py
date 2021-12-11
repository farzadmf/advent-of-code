from collections import deque
import sys
from typing import (
    Deque,
    List,
    Set,
    Tuple,
)

Cell = Tuple[int, int]


class Oct:
    def __init__(self, lines: List[str]):
        self._rows = len(lines)
        self._cols = len(lines[0])
        self._grid = [[0] * self._cols for _ in range(self._rows)]
        self._flashing: Deque[Cell] = deque()
        self._flashed: Set[Cell] = set()
        self.flashes = 0

        self._init_grid(lines)

    def print(self):
        for r in self._grid:
            print(r)

    def _init_grid(self, lines: List[str]):
        for r, l in enumerate(lines):
            for c, char in enumerate(l):
                cur = int(char)
                self._grid[r][c] = cur
                if cur > 9:
                    self._flashing.append((r, c))

    def do_step(self):
        self._flashed = set()
        for r in range(self._rows):
            for c in range(self._cols):
                self._grid[r][c] += 1
                if self._grid[r][c] > 9:
                    self._flashing.append((r, c))
        self._flash()
        for (r, c) in self._flashed:
            self._grid[r][c] = 0
            self.flashes += 1

    def _flash(self):
        while self._flashing:
            for _ in range(len(self._flashing)):
                r, c = self._flashing.popleft()
                self._affect_neighbor((r, c))

    def _affect_neighbor(self, cell: Cell):
        r, c = cell
        if (r, c) in self._flashed or self._grid[r][c] <= 9:
            return
        dirs = [
            (0, 1),
            (1, 0),
            (-1, 0),
            (0, -1),
            (1, 1),
            (-1, -1),
            (1, -1),
            (-1, 1),
        ]
        self._flashed.add((r, c))
        for dr, dc in dirs:
            nr, nc = r + dr, c + dc
            if not (0 <= nr < self._rows and 0 <= nc < self._cols):
                continue
            self._grid[nr][nc] += 1
            if self._grid[nr][nc] > 9:
                self._flashing.append((nr, nc))


def read_input(input_file: str) -> List[str]:
    with open(input_file) as f:
        lines = [l.strip() for l in f.readlines()]
    return lines


Sample = List[List[int]]


def read_sample() -> Tuple[List[str], Sample, Sample]:
    with open("sample") as f:
        lines = f.read().splitlines()
        init_lines = lines[:5]
        init = [[0] * 5 for _ in range(5)]
        one = [[0] * 5 for _ in range(5)]
        two = [[0] * 5 for _ in range(5)]
        for r, l in enumerate(lines):
            if l == "---":
                continue
            for c, char in enumerate(l):
                if r < 5:
                    init[r][c] = int(char)
                elif r < 11:
                    one[(r % 5) - 1][c] = int(char)
                else:
                    two[(r % 5) - 2][c] = int(char)
    return init_lines, one, two


def assert_sample():
    init_lines, one, two = read_sample()
    octup = Oct(init_lines)
    octup.do_step()
    assert octup._grid == one, "step 1 is not good"
    octup.do_step()
    assert octup._grid == two, "step 2 is not good"
    print(octup.flashes)


def main(input_file: str) -> None:
    #  assert_sample()

    lines = read_input(input_file)
    octup = Oct(lines)
    for _ in range(100):
        octup.do_step()
    print(octup.flashes)


if __name__ == "__main__":
    main(sys.argv[1])
