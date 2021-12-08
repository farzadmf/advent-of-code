import sys
from typing import (
    List,
    NamedTuple,
)


class Display(NamedTuple):
    patterns: List[str]
    outputs: List[str]

    def __str__(self) -> str:
        return f"""
patterns = {self.patterns}
outputs = {self.outputs}
    """.strip()


def read_signals(input_file: str) -> List[Display]:
    displays = []
    with open(input_file) as f:
        lines = f.readlines()
        for l in lines:
            split = l.split("|")
            patterns = [p.strip() for p in split[0].strip().split(" ")]
            outputs = [o.strip() for o in split[1].strip().split(" ")]
            displays.append(
                Display(
                    patterns=patterns,
                    outputs=outputs,
                )
            )
    return displays


def count_unique_outputs(displays: List[Display]) -> int:
    count = 0
    for d in displays:
        for o in d.outputs:
            if len(o) == 2 or len(o) == 4 or len(o) == 3 or len(o) == 7:
                count += 1
    return count


def main(input_file: str) -> None:
    displays = read_signals(input_file)
    print(count_unique_outputs(displays))


if __name__ == "__main__":
    main(sys.argv[1])
