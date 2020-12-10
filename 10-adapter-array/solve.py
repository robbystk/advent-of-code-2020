import sys

from collections import Counter

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

    print(delta_counts)

    print(delta_counts[1] * delta_counts[3])

if __name__ == '__main__':
    main()
