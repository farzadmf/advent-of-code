with open("input.txt") as f:
    lines = f.read().splitlines()

bits = [{0: 0, 1: 0} for _ in range(len(lines[0]))]
for l in lines:
    for i, b in enumerate(l):
        bits[i][int(b)] += 1

gamma = ""
epsilon = ""
for b in bits:
    gamma = f"{gamma}{'1' if b[1] > b[0] else '0'}"
    epsilon = f"{epsilon}{'1' if b[1] < b[0] else '0'}"

gamma = int(gamma, 2)
epsilon = int(epsilon, 2)
print(gamma * epsilon)

# ---

bits = [{0: 0, 1: 0} for _ in range(len(lines[0]))]
for l in lines:
    for i, b in enumerate(l):
        bits[i][int(b)] += 1


def bit_count(idx, nums):
    """
    Given an index and a list of nums, returns the count of 0's and 1's in that index
    """
    bits = {0: 0, 1: 0}
    for n in nums:
        bits[int(n[idx])] += 1
    return bits


def ox_bit(counts):
    """
    Bit criteria for oxygen rating
    """
    return "1" if counts[1] >= counts[0] else "0"


def co_bit(counts):
    """
    Bit criteria for co2 rating
    """
    return "1" if counts[1] < counts[0] else "0"


def get_rating(criteria_fn):
    # We use 'remain' to add the ones we want to keep
    # In each step, we assign 'temp_lines' to that and reset 'remain'
    #   for next iteration
    temp_lines = lines[:]
    remain = []
    bits = len(lines[0])  # Bit length of the numbers
    while len(temp_lines) > 1:
        for i in range(bits):
            counts = bit_count(i, temp_lines)
            want_bit = criteria_fn(counts)
            for l in temp_lines:
                if l[i] == want_bit:
                    remain.append(l)
            temp_lines = remain[:]
            remain = []
            if len(temp_lines) == 1:
                break
    return temp_lines[0]


ox = int(get_rating(ox_bit), 2)
co = int(get_rating(co_bit), 2)
print(ox * co)
