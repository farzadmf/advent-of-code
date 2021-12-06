from typing import (
    Dict,
    Iterable,
    List,
)
import sys


def read_ages(input_file) -> List[int]:
    with open(input_file) as f:
        line = f.readlines()[0].strip()
    return [int(val) for val in line.split(",")]


def read_output(output_file) -> List[List[int]]:
    lines: List[List[int]] = []
    with open(output_file) as f:
        file_lines = f.readlines()
        for fl in file_lines:
            line_list = [int(val) for val in fl.split(",")]
            lines.append(sorted(line_list))
    return lines


def spawn(ages: List[int], days: int) -> int:
    for _ in range(days):
        n = len(ages)
        for i in range(n):
            if ages[i] == 0:
                ages.append(8)
                ages[i] = 7
            ages[i] -= 1
    return len(ages)


def dict_to_list(counts: Dict[int, int]) -> List[int]:
    res = []
    for k, v in counts.items():
        res += [k] * v
    return sorted(res)


#  def spawn2(ages: List[int], days: int) -> Iterable[Dict[int, int]]:
def spawn2(ages: List[int], days: int) -> int:
    counts = {}

    for i in range(10):
        counts[i] = 0

    for a in ages:
        counts[a] += 1

    #  yield counts

    for _ in range(days):
        counts[9] += counts[0]
        counts[7] += counts[0]
        counts[0] = 0

        for i in range(1, 10):
            if counts[i] > 0:
                counts[i - 1] += counts[i]
                counts[i] = 0

        #  yield counts
    return sum(counts.values())


def main(input_file: str, days: int) -> None:
    ages = read_ages(input_file)
    total = spawn2(ages, days)
    print(total)


def test_small_output() -> None:
    pass
    #  ages = read_ages("input_small")
    #  outputs = read_output("input_small_output")

    #  d = 0
    #  for my in spawn2(ages, 18):
    #      print("SPAWN2", dict_to_list(my))
    #      print("OUTPUT", outputs[d])
    #      print("-" * 40)
    #      d += 1


if __name__ == "__main__":
    #  test_small_output()
    main(sys.argv[1], int(sys.argv[2]))
