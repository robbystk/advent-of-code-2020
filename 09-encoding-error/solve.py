import sys

from collections import deque

def input():
    with open(sys.argv[1]) as f:
        for line in f:
            yield line

def preamble_length():
    if len(sys.argv) >= 2:
        return int(sys.argv[2])
    else:
        return 25

def is_valid(last_n, n):
    for i, x in enumerate(last_n):
        for y in last_n[:i]:
            if n == x + y:
                return True
    return False

def main():
    preamble_len = preamble_length()
    last_n = [float('nan') for _ in range(preamble_len)]
    full_input = []

    first_invalid = None

    for i, line in enumerate(input()):
        n = int(line)
        valid = is_valid(last_n, n)
        last_n[i % preamble_len] = n
        full_input.append(n)
        if not valid and i >= preamble_len:
            first_invalid = n
            break

    print(first_invalid)

    range_sum = 0
    input_count = len(full_input)
    i = 0
    j = 1
    while range_sum != first_invalid:
        range_sum = sum(full_input[i:j])
        i += 1
        if i >= j:
            i = 0
            j += 1
            if j >= input_count:
                raise "didn't find a match"

    print(i - 1, j)

    summing_range = full_input[(i-1):j]
    smallest = min(summing_range)
    largest = max(summing_range)
    print(smallest + largest)

if __name__ == '__main__':
    main()
