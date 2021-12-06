from typing import (
    List,
)
import sys


def read_ages(input_file) -> List[int]:
    with open(input_file) as f:
        line = f.readlines()[0].strip()
    return [int(val) for val in line.split(",")]


def spawn(ages: List[int], days: int) -> int:
    for _ in range(days):
        n = len(ages)
        for i in range(n):
            if ages[i] == 0:
                ages.append(8)
                ages[i] = 7
            ages[i] -= 1
    return len(ages)


def main(input_file) -> None:
    ages = read_ages(input_file)
    print(spawn(ages, 80))


if __name__ == "__main__":
    main(sys.argv[1])
