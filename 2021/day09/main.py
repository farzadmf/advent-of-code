from functools import reduce
import sys
from typing import (
    Dict,
    List,
    Set,
    Tuple,
)

Cell = Tuple[int, int]
Path = Dict[Cell, Set[Cell]]


class HeightMap:
    def __init__(self, raw_values: List[str]) -> None:
        self.values: List[List[int]] = []
        self.low_points: List[Tuple[int, int]] = []
        self.risks: List[int] = []
        self.rows: int = 0
        self.cols: int = 0
        self.basins: Path = {}
        self.basin_sizes: List[int] = []

        self._init_values(raw_values)
        self._init_lows()
        self._init_risks()

    def _init_values(self, raw: List[str]) -> None:
        for l in raw:
            self.values.append([int(raw) for raw in l.strip()])

    def _init_lows(self):
        vals = self.values
        self.rows, self.cols = len(vals), len(vals[0])

        for r in range(self.rows):
            for c in range(self.cols):
                cur = vals[r][c]

                left = float("inf") if c == 0 else vals[r][c - 1]
                right = float("inf") if c == self.cols - 1 else vals[r][c + 1]
                up = float("inf") if r == 0 else vals[r - 1][c]
                down = float("inf") if r == self.rows - 1 else vals[r + 1][c]

                if cur < left and cur < right and cur < up and cur < down:
                    self.low_points.append((r, c))

    def _init_risks(self):
        for r, c in self.low_points:
            self.risks.append(self.values[r][c] + 1)

    def calc_basins(self):
        def dfs(r: int, c: int, val: int, path: Set[Cell]):
            cur = self.values[r][c]
            if cur == -1:
                return

            path.add((r, c))
            dirs = [(1, 0), (0, 1), (-1, 0), (0, -1)]
            self.values[r][c] = -1
            for dr, dc in dirs:
                nr, nc = r + dr, c + dc
                if not (0 <= nr < self.rows and 0 <= nc < self.cols):
                    continue
                nxt = self.values[nr][nc]
                if nxt <= val or nxt == 9:
                    continue
                dfs(nr, nc, self.values[nr][nc], path)
            self.values[r][c] = cur

        for r, c in self.low_points:
            self.basins[(r, c)] = set()
            path = self.basins[(r, c)]
            dfs(r, c, self.values[r][c], path)
            self.basin_sizes.append(len(path))

        self.basin_sizes.sort(reverse=True)

    def print_basins(self):
        for (r, c), path in self.basins.items():
            print(f"for {self.values[r][c]} @ ({r},{c}) -> total of {len(path)}")
            for pr, pc in path:
                print(f"    {self.values[pr][pc]} @ ({pr},{pc})")


def read_input(input_file: str) -> List[str]:
    lines: List[str] = []
    with open(input_file) as f:
        lines = f.readlines()
    return lines


def main(input_file: str) -> None:
    raw_vals = read_input(input_file)
    h_map = HeightMap(raw_vals)
    #  print(sum(h_map.risks))
    h_map.calc_basins()
    #  h_map.print_basins()
    print(reduce(lambda a, b: a * b, h_map.basin_sizes[:3]))


if __name__ == "__main__":
    main(sys.argv[1])
