from collections import (
    deque,
    Counter,
)
import sys
from typing import (
    Dict,
    List,
    Tuple,
)

Rules = Dict[str, str]
Template = str
Pairs = List[Template]


def process(pair: Template, rules: Rules) -> Tuple[List[Template], str]:
    added = rules[pair]
    pair = f"{pair[0]}{rules[pair]}{pair[1]}"
    return [pair[:2], pair[1:]], added


def read_input(input_file: str) -> Tuple[Template, Pairs, Rules]:
    rules = {}
    pairs = []

    with open(input_file) as f:
        lines = f.read().splitlines()
        template = lines[0]
        for i in range(len(template) - 1):
            pairs.append(f"{template[i:i + 2]}")

        for l in lines[2:]:
            src, dst = l.split("->")
            rules[src.strip()] = dst.strip()

    return template, pairs, rules


def main(input_file: str) -> None:
    template, pairs, rules = read_input(input_file)
    counter = Counter(template)

    q = deque(pairs)
    for _ in range(10):
        for _ in range(len(q)):
            p = q.popleft()
            ps, added = process(p, rules)
            for p in ps:
                q.append(p)
            counter[added] += 1

    values = sorted(counter.values())
    print(values[-1] - values[0])


if __name__ == "__main__":
    main(sys.argv[1])
