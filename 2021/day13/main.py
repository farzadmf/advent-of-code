from collections import deque
import sys
from typing import (
    Dict,
    Deque,
    List,
    Tuple,
)

Point = Tuple[int, int]
Fold = Tuple[str, int]


class Table:
    def __init__(self, lines: List[str]) -> None:
        self.dots: List[Point] = []
        self.folds: List[Fold] = []
        self.max_x = 0
        self.max_y = 0
        self.rows = 0
        self.cols = 0
        self.lines: List[List[str]] = []

        self._read_input(lines)
        self._populate_lines()

    def print_debug(self) -> None:
        print("    ", end="")
        for c in range(self.cols):
            print(f"{c:02} ", end="")
        print()

        for r in range(self.rows):
            print(f"{r:02} ", end="")
            for c in range(self.cols):
                char = "  #" if self.lines[r][c] == "1" else "  ."
                print(char, end="")
            print()

    def print(self) -> None:
        for r in range(self.rows):
            for c in range(self.cols):
                char = "▇" if self.lines[r][c] == "1" else "."
                print(char, end="")
            print()

    def do_fold(self, fold: Fold) -> None:
        amount = fold[1]
        if fold[0] == "x":
            self._fold_x(amount)
        else:
            self._fold_y(amount)

    def _fold_y(self, amount: int) -> None:
        for r in range(amount):
            src_idx = self.rows - r - 1
            dst_idx = self.rows - (2 * amount) + r - 1
            src_row = "".join(self.lines[src_idx])
            dst_row = "".join(self.lines[dst_idx])
            src_bin = int(src_row, base=2)
            dst_bin = int(dst_row, base=2)
            res = src_bin | dst_bin
            res_str = bin(res)[2:].zfill(self.cols)
            self.lines[dst_idx] = [c for c in res_str]
            self.lines.pop()
        self.lines.pop()
        self.rows -= amount + 1

    def _fold_x(self, amount: int) -> None:
        for r in range(self.rows):
            row = self.lines[r]
            src_idx = self.cols - amount
            dst_idx = self.cols - (2 * amount) - 1
            src_cols = "".join(row[src_idx:])[::-1]
            dst_cols = "".join(row[dst_idx : src_idx - 1])
            src_bin = int(src_cols, base=2)
            dst_bin = int(dst_cols, base=2)
            res = src_bin | dst_bin
            res_str = bin(res)[2:].zfill(self.cols - (amount + 1))
            self.lines[r] = [c for c in res_str]
            for _ in range(amount):
                row.pop()
        self.cols -= amount + 1

    @property
    def dot_count(self) -> int:
        count = 0
        for r in range(self.rows):
            count += self.lines[r].count("1")
        return count

    def _read_input(self, lines: List[str]):
        i = 0
        while lines[i].strip() != "":
            x, y = lines[i].split(",")
            x, y = int(x), int(y)
            self.max_x = max(self.max_x, x)
            self.max_y = max(self.max_y, y)
            self.dots.append((x, y))
            i += 1

        i += 1
        while i < len(lines):
            direction, value = lines[i].split()[-1].split("=")
            self.folds.append((direction, int(value)))
            i += 1

    def _populate_lines(self):
        self.lines = [["0"] * (self.max_x + 1) for _ in range(self.max_y + 1)]
        self.rows = len(self.lines)
        self.cols = len(self.lines[0])

        for y in range(self.max_y + 1):
            for x in range(self.max_x + 1):
                if (x, y) in self.dots:
                    self.lines[y][x] = "1"


def read_input(input_file: str) -> List[str]:
    with open(input_file) as f:
        lines = f.read().splitlines()
        return lines


def main(input_file: str) -> None:
    lines = read_input(input_file)
    table = Table(lines)
    for f in table.folds:
        table.do_fold(f)
    table.print()


if __name__ == "__main__":
    main(sys.argv[1])
