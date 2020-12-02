import sys

def input():
    with open(sys.argv[1]) as f:
        for line in f:
            yield line

def main():
    entries = [int(line) for line in input()]

    for i in range(len(entries)):
        for j in range(i):
            for k in range(j):
                if entries[i] + entries[j] + entries[k] == 2020:
                    print(entries[i] * entries[j] * entries[k])
                    break

if __name__ == '__main__':
    main()
