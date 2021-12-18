import sys
from typing import (
    List,
    Tuple,
    Optional,
)

lookup = {
    "0": "0000",
    "1": "0001",
    "2": "0010",
    "3": "0011",
    "4": "0100",
    "5": "0101",
    "6": "0110",
    "7": "0111",
    "8": "1000",
    "9": "1001",
    "A": "1010",
    "B": "1011",
    "C": "1100",
    "D": "1101",
    "E": "1110",
    "F": "1111",
}


def read_input(input_file: str) -> str:
    with open(input_file) as f:
        return f.readlines()[0].strip()


class Packet:
    def __init__(self, version: int, type_id: int) -> None:
        self.version = version
        self.type_id = type_id


class Literal(Packet):
    def __init__(self, version: int, type_id: int, bin_str: str, start: int) -> None:
        super().__init__(version, type_id)

        self.literal: int
        self.length: int

        self._read_str(bin_str, start)

    def _read_str(self, bin_str: str, start: int) -> None:
        lit_start = start + 6
        offset = 5
        literal = ""

        def read_val() -> None:
            nonlocal literal

            literal = f"{literal}{bin_str[lit_start + 1 : lit_start + offset]}"

        while bin_str[lit_start : lit_start + offset][0] == "1":
            read_val()
            lit_start += offset

        # Read the last one (with '0' prefix)
        read_val()
        lit_start += offset

        self.literal = int(literal, base=2)
        self.length = lit_start - start

    def __str__(self) -> str:
        return f"{self.literal}"

    def __repr__(self) -> str:
        return f"{self.literal}"


class Operator(Packet):
    def __init__(self, version: int, type_id: int, bin_str: str, start: int) -> None:
        super().__init__(version, type_id)

        self.literals: List[Literal] = []
        self.operators: List[Operator] = []
        self.length: int = 0
        self.type_length_id = int(bin_str[start + 6])

        self._read_str(bin_str, start)

    def _read_str(self, bin_str: str, start: int) -> None:
        if self.type_length_id == 0:
            length = int(bin_str[start + 7 : start + 22], base=2)
            sub_start = start + 22
            should_continue = True
            length_read = 0

            while should_continue:
                version = int(bin_str[sub_start : sub_start + 3], base=2)
                type_id = int(bin_str[sub_start + 3 : sub_start + 6], base=2)
                if type_id == 4:
                    lit = Literal(version, type_id, bin_str, sub_start)
                    self.literals.append(lit)
                    sub_start += lit.length
                    length_read += lit.length
                    if length_read == length:
                        should_continue = False
                else:
                    op = Operator(version, type_id, bin_str, sub_start)
                    self.operators.append(op)
                    # sub_offset += op.length
        else:
            count = int(bin_str[start + 7 : start + 18], base=2)
            sub_start = start + 18

            for _ in range(count):
                version = int(bin_str[sub_start : sub_start + 3], base=2)
                type_id = int(bin_str[sub_start + 3 : sub_start + 6], base=2)
                if type_id == 4:
                    lit = Literal(version, type_id, bin_str, sub_start)
                    self.literals.append(lit)
                    sub_start += lit.length
                else:
                    op = Operator(version, type_id, bin_str, sub_start)
                    self.operators.append(op)

    def __str__(self) -> str:
        return f"{self.literals}"

    def __repr__(self) -> str:
        return f"{self.literals}"


def get_literal(bin_str: str) -> Tuple[int, int]:
    start = 6
    offset = 5
    literal = ""

    def read_val(start: int) -> None:
        nonlocal literal

        literal = f"{literal}{bin_str[start + 1 : start + offset]}"

    while bin_str[start : start + offset][0] == "1":
        read_val(start)
        start += offset

    # Read the last one (with '0' prefix)
    read_val(start)
    start += offset

    return (int(literal, base=2), start)


def convert_to_binary(string: str) -> str:
    bin_str = ""
    for c in string:
        bin_str = f"{bin_str}{lookup[c]}"
    return bin_str


def main(input_file: str) -> None:
    line = read_input(input_file)
    bin_str = convert_to_binary(line)
    start = 0

    lits: List[Literal] = []
    ops: List[Operator] = []

    while start < len(bin_str):
        version = int(bin_str[start : start + 3], base=2)
        type_id = int(bin_str[start + 3 : start + 6], base=2)

        if type_id == 4:
            lit = Literal(version, type_id, bin_str, start)
            lits.append(lit)
            start += 100
        else:
            op = Operator(version, type_id, bin_str, start)
            ops.append(op)
            start += 100

        continue
        nxt = 0

        if type_id == 4:
            literal, nxt = get_literal(bin_str)
        else:
            type_length_id = int(bin_str[start + 6])
            if type_length_id == 0:
                length = int(bin_str[start + 7 : start + 22], base=2)
                sub_start = start + 22
                sub_offset = 0
                while sub_offset < length:
                    version = int(bin_str[sub_start : sub_start + 3], base=2)
                    type_id = int(bin_str[sub_start + 3 : sub_start + 6], base=2)
                    assert type_id == 4, f"type_id is NOT 4!!! is {type_id}"
                    literal, sub_nxt = get_literal(bin_str[sub_start + sub_offset :])
                    sub_offset += sub_nxt
                nxt = sub_start + sub_offset
            else:
                count = int(bin_str[start + 7 : start + 18], base=2)
                sub_start = start + 18
                sub_offset = 0
                for _ in range(count):
                    version = int(bin_str[sub_start : sub_start + 3], base=2)
                    type_id = int(bin_str[sub_start + 3 : sub_start + 6], base=2)
                    assert type_id == 4, f"type_id is NOT 4!!! is {type_id}"
                    literal, sub_nxt = get_literal(bin_str[sub_start + sub_offset :])
                    print("literal", literal)
                    sub_offset += sub_nxt
                nxt = sub_start + sub_offset
        while nxt % 8 != 0:
            nxt += 1

        start = nxt
    #  packet = Packet(line)
    #  print("version", packet.version)
    #  print("type_id", packet.type_id)
    #  print("literal", packet.literal)
    print(lits)
    print(ops)


if __name__ == "__main__":
    main(sys.argv[1] if len(sys.argv) == 2 else "day16/op3")
