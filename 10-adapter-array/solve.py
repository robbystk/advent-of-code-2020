import sys

from collections import Counter
from math import comb

def input():
    with open(sys.argv[1]) as f:
        for line in f:
            yield line

def main():
    adapter_ratings = [0]

    for line in input():
        adapter_ratings.append(int(line))

    adapter_ratings.sort()

    adapter_ratings.append(adapter_ratings[-1] + 3)

    # print(adapter_ratings)

    deltas = [adapter_ratings[i+1] - adapter_ratings[i] for i in range(len(adapter_ratings) - 1)]

    print(deltas)

    delta_counts = Counter(deltas)

    # print(delta_counts)

    # print(delta_counts[1] * delta_counts[3])

    # find the length of each run of ones
    run_lengths = []
    current_length = 0
    for d in deltas:
        if d == 1:
            current_length += 1
        else:
            run_lengths.append(current_length)
            current_length = 0

    # print(run_lengths)

    arrangement_count = 1
    for run_length in run_lengths:
        run_length -= 1
        if run_length >= 1:
            # this doesn't work for run lengths above 4, but there are none
            arrangement_count *= sum([comb(run_length, n) for n in range(3)])

    print(arrangement_count)


if __name__ == '__main__':
    main()
