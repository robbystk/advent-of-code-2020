import sys

def input():
    with open(sys.argv[1]) as f:
        for line in f:
            yield line

def main():
    group_list = [set('abcdefghijklmnopqrstuvwxyz')]
    for line in input():
        if line == '\n':
            group_list.append(set('abcdefghijklmnopqrstuvwxyz'))
        else:
            group_list[-1] &= set(line.strip())

    count = 0
    for group in group_list:
        count += len(group)

    print(count)

if __name__ == '__main__':
    main()
