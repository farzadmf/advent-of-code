def read_input(name):
    with open(name) as f:
        return f.read().splitlines()


def challenge1():
    # lines = read_input("input_small")
    lines = read_input("input")

    cur_max = 0
    total_max = float("-inf")

    for l in lines:
        if not l:
            cur_max = 0
            continue
        cur_max += int(l)
        total_max = max(total_max, cur_max)

    print(total_max)


def challenge2():
    # lines = read_input("input_small")
    lines = read_input("input")

    cur_max = 0
    maxes = []

    for l in lines:
        if not l:
            maxes.append(cur_max)
            cur_max = 0
            continue
        cur_max += int(l)

    # Missed this one!
    maxes.append(cur_max)

    top_three = sorted(maxes, reverse=True)[:3]
    print(sum(top_three))


# challenge1()
challenge2()
