from collections import defaultdict
from typing import (
    Dict,
    List,
)
import sys


def read_values(input_file: str) -> List[int]:
    with open(input_file) as f:
        values = f.readlines()[0].split(",")
    return [int(v) for v in values]


def calc_fuel(values: List[int]) -> Dict[int, int]:
    max_pos = max(values)
    fuel = defaultdict(int)
    for pos in range(max_pos):
        for val in values:
            fuel[pos] += abs(val - pos)

    return fuel


def main(input_file: str) -> None:
    values = read_values(input_file)
    fuel = calc_fuel(values)
    print(min(fuel.values()))


if __name__ == "__main__":
    main(sys.argv[1])
