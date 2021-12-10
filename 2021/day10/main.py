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

points = {
    "}": 1197,
    ">": 25137,
    ")": 3,
    "]": 57,
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

    def __str__(self) -> str:
        return f"{self.line} -> incomp? {self.incomplete} | valid? {self.valid}"


class Navigation:
    def __init__(self, lines: List[str]):
        self.lines = lines
        self.chunks: List[Chunk] = []
        self.corrupts: List[Chunk] = []
        self.incompletes: List[Chunk] = []
        self.syntax_error = 0

        self._init_chunks()

    def _init_chunks(self):
        for line in self.lines:
            c = Chunk(line)
            if not c.valid:
                self.corrupts.append(c)
                self.syntax_error += points[c.illegal]
            elif c.incomplete:
                self.incompletes.append(c)


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
    print(nav.syntax_error)


if __name__ == "__main__":
    main(sys.argv[1])
