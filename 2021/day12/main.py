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

    def traverse(self) -> List[List[str]]:
        def dfs(
            cave: str,
            visited: Set[str],
            cur_path: List[str],
            all_paths: List[List[str]],
        ) -> None:
            if cave in visited:
                return
            if cave == "end":
                all_paths.append(cur_path + ["end"])
                return
            for neigh in self._connections[cave]:
                dfs(
                    cave=neigh,
                    visited=visited | {cave} if cave.islower() else visited,
                    cur_path=cur_path + [cave],
                    all_paths=all_paths,
                )

        all_paths = []
        dfs(
            cave="start",
            visited=set(),
            cur_path=[],
            all_paths=all_paths,
        )

        return all_paths


def read_input(input_file: str) -> List[str]:
    with open(input_file) as f:
        return f.read().splitlines()


def assert_input_part1(input_file: str, count: int) -> None:
    lines = read_input(input_file)
    graph = Graph(lines)
    paths = graph.traverse()
    assert len(paths) == count, f"count of {input_file} is {len(paths)} instead of {count}"


def main() -> None:
    assert_input_part1("input_tiny", 10)
    assert_input_part1("input_small", 19)
    assert_input_part1("input_medium", 226)

    # Help for part 1 from here: https://git.io/JDW4B
    lines = read_input("input")
    graph = Graph(lines)
    paths = graph.traverse()
    print(len(paths))


if __name__ == "__main__":
    main()
