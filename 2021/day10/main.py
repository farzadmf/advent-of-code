import sys
from typing import (
    List,
)

pairs = {
    "}": "{",
    ">": "<",
    ")": "(",
    "]": "[",
}

reverse_pairs = {
    "{": "}",
    "<": ">",
    "(": ")",
    "[": "]",
}

error_points = {
    "}": 1197,
    ">": 25137,
    ")": 3,
    "]": 57,
}

completion_points = {
    "}": 3,
    ">": 4,
    ")": 1,
    "]": 2,
}


def read_input(input_file: str) -> List[str]:
    lines = []
    with open(input_file) as f:
        input = f.readlines()
        for line in input:
            lines.append(line.strip())

    return lines


def read_small_corrupt() -> str:
    with open("./input_small_corrupt") as f:
        lines = f.read().strip()
    return lines


class Chunk:
    def __init__(self, line: str) -> None:
        self.line = line
        self.valid = True
        self.incomplete = False
        self.illegal = ""
        self.remain = ""

        self._set_valid()

    def _set_valid(self):
        stack = []
        for c in self.line:
            if c in pairs.values():
                stack.append(c)
            else:
                if stack[-1] != pairs[c]:
                    self.illegal = c
                    self.valid = False
                    break
                else:
                    stack.pop()
        if self.valid:
            self.incomplete = len(stack) != 0
        if self.incomplete:
            self.remain = "".join(stack)

    @property
    def error_point(self):
        return 0 if self.valid else error_points[self.illegal]

    @property
    def completion_point(self):
        points = 0
        for c in self.remain[::-1]:
            points = (points * 5) + completion_points[reverse_pairs[c]]

        return points

    def __str__(self) -> str:
        return f"{self.line} -> incomp? {self.incomplete} | valid? {self.valid}"


class Navigation:
    def __init__(self, lines: List[str]):
        self._lines = lines
        self._chunks: List[Chunk] = []
        self._corrupts: List[Chunk] = []
        self._incompletes: List[Chunk] = []
        self._completion_points: List[int] = []

        self.completion_points_val = 0
        self.error_points = 0

        self._init_chunks()
        self._calc_error_points()
        self._calc_completion_points()

    def _init_chunks(self):
        for line in self._lines:
            c = Chunk(line)
            if not c.valid:
                self._corrupts.append(c)
            elif c.incomplete:
                self._incompletes.append(c)

    def _calc_error_points(self):
        for c in self._corrupts:
            self.error_points += c.error_point

    def _calc_completion_points(self):
        for c in self._incompletes:
            self._completion_points.append(c.completion_point)

        n = len(self._completion_points)
        self.completion_points_val = sorted(self._completion_points)[n // 2]


def assert_corrupts(lines: List[str]) -> None:
    corrs = read_small_corrupt()
    corrupts = []
    for l in lines:
        c = Chunk(l)
        if not (c.valid):
            corrupts.append(l)
    assert "\n".join(sorted(corrupts)) == corrs, "corrupts are different"


def main(input_file: str) -> None:
    lines = read_input(input_file)
    #  assert_corrupts(lines)

    nav = Navigation(lines)
    #  print(nav.error_points)
    print(nav.completion_points_val)


if __name__ == "__main__":
    main(sys.argv[1])
