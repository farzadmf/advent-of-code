#!/usr/bin/env python


from functools import reduce


def print_dots():
    width = reduce(lambda a, b: max(a, b[0]), dots, 0) + 1
    height = reduce(lambda a, b: max(a, b[1]), dots, 0) + 1
    for y in range(height):
        for x in range(width):
            if (x, y) in dots:
                print("▇", end="")
            else:
                print(" ", end="")
        print("")


with open("input") as f:
    rawdata = f.readlines()

dots = set()
folds = []
axis_labels = {"x": 0, "y": 1}
for l in rawdata:
    if l[0].isdigit():
        a, b = l.split(",")
        dots.add((int(a), int(b)))
    elif l.startswith("fold"):
        k, v = l.rsplit(" ", 1)[1].split("=")
        folds.append((axis_labels[k], int(v)))

part1_done = False
for fold in folds:
    axis, offset = fold
    folded = set()
    for dot in dots:
        if dot[axis] > offset:
            unchanged = dot[1 - axis]
            updated = offset - (dot[axis] - offset)
            if axis == axis_labels["x"]:
                folded.add((updated, unchanged))
            else:
                folded.add((unchanged, updated))
        else:
            folded.add(dot)
    dots = folded
    if not part1_done:
        print(f"Part 1: {len(folded)}")
        part1_done = True

print("Part 2:")
print_dots()
