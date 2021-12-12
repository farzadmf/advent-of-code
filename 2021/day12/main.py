from typing import (
    Dict,
    List,
    Set,
)


class Graph:
    def __init__(self, lines: List[str]) -> None:
        self._connections: Dict[str, List[str]] = {}

        for l in lines:
            s, d = l.strip().split("-")
            self._connections.setdefault(s, [])
            self._connections.setdefault(d, [])
            self._connections[s].append(d)
            self._connections[d].append(s)

    def traverse(
        self,
        cave: str,
        visited: Set[str],
        second_small_allowed: bool,
    ) -> int:
        if cave == "end":
            return 1
        count = 0
        for neigh in self._connections[cave]:
            if neigh.isupper():
                count += self.traverse(
                    cave=neigh,
                    visited=visited,
                    second_small_allowed=second_small_allowed,
                )
            elif neigh not in visited:
                count += self.traverse(
                    cave=neigh,
                    visited=visited | {neigh},
                    second_small_allowed=second_small_allowed,
                )
            elif second_small_allowed and neigh != "start":
                count += self.traverse(
                    cave=neigh,
                    visited=visited,
                    second_small_allowed=False,
                )
        return count


def read_input(input_file: str) -> List[str]:
    with open(input_file) as f:
        return f.read().splitlines()


def assert_input_part(
    input_file: str,
    count: int,
    second_small_allowed: bool,
) -> None:
    lines = read_input(input_file)
    graph = Graph(lines)
    paths = graph.traverse(
        cave="start",
        visited=set(["start"]),
        second_small_allowed=second_small_allowed,
    )
    assert paths == count, f"count of {input_file} is {paths} instead of {count}"


def main() -> None:
    assert_input_part("input_tiny", 10, False)
    assert_input_part("input_small", 19, False)
    assert_input_part("input_medium", 226, False)

    assert_input_part("input_tiny", 36, True)
    assert_input_part("input_small", 103, True)
    assert_input_part("input_medium", 3509, True)

    # Help for part 1 from here: https://git.io/JDW4B
    lines = read_input("input")
    graph = Graph(lines)
    paths1 = graph.traverse(
        cave="start",
        visited=set(["start"]),
        second_small_allowed=False,
    )
    paths2 = graph.traverse(
        cave="start",
        visited=set(["start"]),
        second_small_allowed=True,
    )
    print(f"Part 1: {paths1}")
    print(f"Part 2: {paths2}")


if __name__ == "__main__":
    main()
