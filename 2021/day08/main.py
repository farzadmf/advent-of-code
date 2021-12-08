import sys
from typing import (
    Dict,
    List,
)


class Display:
    def __init__(self, patterns: List[str], outputs: List[str]):
        self.outputs = outputs

        self.values: Dict[str, int] = {}
        self.one = ""
        self.four = ""
        self.seven = ""
        self.eight = ""

        self.zero = ""
        self.two = ""
        self.three = ""
        self.five = ""
        self.six = ""
        self.nine = ""

        self._get_uniques(patterns)
        self._get_segments(sorted(patterns, key=len, reverse=True))

    def get_output(self) -> int:
        res = 0
        for o in self.outputs:
            res = (10 * res) + self.values["".join(sorted(o))]
        return res

    def _get_uniques(self, patterns: List[str]):
        for p in patterns:
            if len(p) == 2:
                self.one = "".join(sorted(p))
                self.values[self.one] = 1
            elif len(p) == 3:
                self.seven = "".join(sorted(p))
                self.values[self.seven] = 7
            elif len(p) == 4:
                self.four = "".join(sorted(p))
                self.values[self.four] = 4
            elif len(p) == 7:
                self.eight = "".join(sorted(p))
                self.values[self.eight] = 8

        assert self.one != "", "one is empty"
        assert self.four != "", "four is empty"
        assert self.seven != "", "seven is empty"
        assert self.eight != "", "eight is empty"

    def _get_segments(self, patterns: List[str]):
        # Hints from: https://git.io/JDTU2
        for p in patterns:
            # 0, 9, 6 have 6 segments
            if len(p) == 6:
                # 1 is included in both 0 and 9, but not 6
                if (set(p) & set(self.one)) != set(self.one):
                    self.six = "".join(sorted(p))
                    self.values[self.six] = 6
                # 4 is included in 9, but not 0
                elif (set(p) & set(self.four)) != set(self.four):
                    self.zero = "".join(sorted(p))
                    self.values[self.zero] = 0
                else:
                    self.nine = "".join(sorted(p))
                    self.values[self.nine] = 9
            # 2, 3, 5 have 5 segments
            elif len(p) == 5:
                # 1 is included only in 3
                if (set(p) & set(self.one)) == set(self.one):
                    self.three = "".join(sorted(p))
                    self.values[self.three] = 3
                # 9 contains 5, but not 2
                elif len(set(p) - set(self.nine)) == 0:
                    self.five = "".join(sorted(p))
                    self.values[self.five] = 5
                else:
                    self.two = "".join(sorted(p))
                    self.values[self.two] = 2


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
            if len(o) == 2:
                count += 1
            elif len(o) == 3:
                count += 1
            elif len(o) == 4:
                count += 1
            elif len(o) == 7:
                count += 1
    return count


def assert_input_one_line() -> None:
    display = read_signals("input_one_line")[0]

    assert display.eight == "".join(sorted("acedgfb")), "eight is wrong"
    assert display.five == "".join(sorted("cdfbe")), "five is wrong"
    assert display.two == "".join(sorted("gcdfa")), "two is wrong"
    assert display.three == "".join(sorted("fbcad")), "three is wrong"
    assert display.seven == "".join(sorted("dab")), "seven is wrong"
    assert display.nine == "".join(sorted("cefabd")), "nine is wrong"
    assert display.six == "".join(sorted("cdfgeb")), "six is wrong"
    assert display.four == "".join(sorted("eafb")), "four is wrong"
    assert display.zero == "".join(sorted("cagedb")), "zero is wrong"
    assert display.one == "".join(sorted("ab")), "one is wrong"

    assert display.values["".join(sorted("acedgfb"))] == 8, "eight dict is wrong"
    assert display.values["".join(sorted("cdfbe"))] == 5, "five dict is wrong"
    assert display.values["".join(sorted("gcdfa"))] == 2, "two dict is wrong"
    assert display.values["".join(sorted("fbcad"))] == 3, "three dict is wrong"
    assert display.values["".join(sorted("dab"))] == 7, "seven dict is wrong"
    assert display.values["".join(sorted("cefabd"))] == 9, "nine dict is wrong"
    assert display.values["".join(sorted("cdfgeb"))] == 6, "six dict is wrong"
    assert display.values["".join(sorted("eafb"))] == 4, "four dict is wrong"
    assert display.values["".join(sorted("cagedb"))] == 0, "zero dict is wrong"
    assert display.values["".join(sorted("ab"))] == 1, "one dict is wrong"

    assert display.get_output() == 5353


def main(input_file: str) -> None:
    displays = read_signals(input_file)
    final = 0
    for d in displays:
        final += d.get_output()
    print(final)


if __name__ == "__main__":
    #  assert_input_one_line()
    main(sys.argv[1])
