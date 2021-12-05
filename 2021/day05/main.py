from collections import defaultdict
from typing import (
    Dict,
    List,
    NamedTuple,
    Tuple,
)
import sys


class Point(NamedTuple):
    x: int
    y: int

    def __str__(self) -> str:
        return f"{self.x, self.y}"

    def __repr__(self) -> str:
        return f"{self.x, self.y}"


class Line:
    def __init__(self, st_point: Point, end_point: Point) -> None:
        self.st_point = st_point
        self.end_point = end_point
        self.cover_points: List[Point] = []

        self._populate_verticals()
        self._populate_horizontals()
        self._populate_diagnoals()

    def _populate_verticals(self):
        if self.st_point.x != self.end_point.x:
            return

        diff_y = abs(self.end_point.y - self.st_point.y)
        st_y = min(self.st_point.y, self.end_point.y)
        for dy in range(st_y, st_y + diff_y + 1):
            self.cover_points.append(
                Point(
                    x=self.st_point.x,
                    y=dy,
                )
            )

    def _populate_horizontals(self):
        if self.st_point.y != self.end_point.y:
            return

        diff_x = abs(self.end_point.x - self.st_point.x)
        st_x = min(self.st_point.x, self.end_point.x)
        for dx in range(st_x, st_x + diff_x + 1):
            self.cover_points.append(
                Point(
                    x=dx,
                    y=self.st_point.y,
                )
            )

    def _populate_diagnoals(self):
        first_p = self.st_point if self.st_point.x < self.end_point.x else self.end_point
        second_p = self.end_point if first_p == self.st_point else self.st_point

        if abs(first_p.y - second_p.y) != abs(first_p.x - second_p.x):
            return

        diff = second_p.y - first_p.y
        if diff < 0:
            rg = range(0, diff - 1, -1)
        else:
            rg = range(0, diff + 1)
        for d in rg:
            self.cover_points.append(
                Point(
                    x=first_p.x + abs(d),
                    y=first_p.y + d,
                )
            )

    def __str__(self) -> str:
        return f"""{self.st_point} -> {self.end_point} | covers: {self.cover_points}""".strip()


def print_lines(max_x: int, max_y: int, covers: Dict[Point, int]):
    for y in range(max_y + 1):
        for x in range(max_x + 1):
            if (x, y) in covers:
                print(covers[Point(x=x, y=y)], end=" ")
            else:
                print(".", end=" ")
        print()


def read_coords(input_file: str) -> Tuple[List[Line], int, int]:
    with open(input_file) as f:
        input_lines = f.readlines()

    lines: List[Line] = []
    max_x = max_y = float("-inf")
    for l in input_lines:
        parts = l.strip().split("->")
        x_st, y_st = parts[0].strip().split(",")
        x_end, y_end = parts[1].strip().split(",")
        x_st, y_st = int(x_st), int(y_st)
        x_end, y_end = int(x_end), int(y_end)
        max_x = max(max_x, x_st, x_end)
        max_y = max(max_y, y_st, y_end)
        st_point = Point(
            x=x_st,
            y=y_st,
        )
        end_point = Point(
            x=x_end,
            y=y_end,
        )
        lines.append(
            Line(
                st_point=st_point,
                end_point=end_point,
            )
        )

    return lines, int(max_x), int(max_y)


def main(input_file: str) -> None:
    #  lines, max_x, max_y = read_coords(input_file=input_file)
    lines, _, _ = read_coords(input_file=input_file)
    covers = defaultdict(int)
    counts = 0
    for l in lines:
        for cover in l.cover_points:
            covers[cover] += 1
            if covers[cover] == 2:
                counts += 1

    #  print_lines(
    #      max_x=max_x,
    #      max_y=max_y,
    #      covers=covers,
    #  )
    print(counts)


if __name__ == "__main__":
    main(sys.argv[1])
