from collections import (
    deque,
    defaultdict,
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


def main(input_file: str, steps: int) -> None:
    template, pairs, rules = read_input(input_file)
    counter = Counter(template)

    # Help from here: https://git.io/JDEXZ
    # idea to not manipulate existing 'pair_counts' and create
    #   a new 'new_counts' and assign it after each loop
    cache: Dict[Template, Tuple[List[Template], str]] = {}
    pair_counts: Dict[str, int] = defaultdict(int)
    new_counts: Dict[str, int] = defaultdict(int)

    for p in pairs:
        cache[p] = process(p, rules)
        pair_counts[p] += 1

    for _ in range(steps):
        new_counts = defaultdict(int)
        for pair, count in pair_counts.items():
            if count <= 0:
                continue

            pairs, added = process(pair, rules)
            counter[added] += count
            for p in pairs:
                new_counts[p] += count

        pair_counts = new_counts

    values = sorted(counter.values())
    print(values[-1] - values[0])


if __name__ == "__main__":
    main(sys.argv[1], int(sys.argv[2]))
