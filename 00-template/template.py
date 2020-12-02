import sys

def input():
    with open(sys.argv[1]) as f:
        for line in f:
            yield line

def main():
    for line in input():
        pass

if __name__ == '__main__':
    main()
