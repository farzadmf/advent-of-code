import sys
from typing import (
    List,
)


class HeightMap:
    def __init__(self, raw_values: List[str]) -> None:
        self.values: List[List[int]] = []
        self.low_points: List[int] = []
        self.risks: List[int] = []

        self.init_values(raw_values)
        self.init_lows()
        self.init_risks()

    def init_values(self, raw: List[str]) -> None:
        for l in raw:
            self.values.append([int(raw) for raw in l.strip()])

    def init_lows(self):
        vals = self.values
        rows, cols = len(vals), len(vals[0])

        for r in range(rows):
            for c in range(cols):
                cur = vals[r][c]

                left = float("inf") if c == 0 else vals[r][c - 1]
                right = float("inf") if c == cols - 1 else vals[r][c + 1]
                up = float("inf") if r == 0 else vals[r - 1][c]
                down = float("inf") if r == rows - 1 else vals[r + 1][c]

                if cur < left and cur < right and cur < up and cur < down:
                    self.low_points.append(cur)

    def init_risks(self):
        for p in self.low_points:
            self.risks.append(p + 1)


def read_input(input_file: str) -> List[str]:
    lines: List[str] = []
    with open(input_file) as f:
        lines = f.readlines()
    return lines


def main(input_file: str) -> None:
    raw_vals = read_input(input_file)
    h_map = HeightMap(raw_vals)
    print(sum(h_map.risks))


if __name__ == "__main__":
    main(sys.argv[1])
