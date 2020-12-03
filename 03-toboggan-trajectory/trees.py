import sys

import numpy as np

def input():
    with open(sys.argv[1]) as f:
        for line in f:
            yield line

def main():
    tree_field = []
    for line in input():
        row = []
        for c in line.strip():
            row.append(c == '#')
        tree_field.append(row)

    tree_field = np.array(tree_field)
    height, width = tree_field.shape

    # s = '\n'.join([''.join(['#' if tree_field[i,j] else '.' for j in range(width)]) for i in range(height)])

    x_pos = 0
    y_pos = 0
    x_stride = 3
    y_stride = 1
    trees_encountered = 0
    while y_pos < height:
        if tree_field[y_pos, x_pos]:
            trees_encountered += 1
        y_pos += y_stride
        x_pos = (x_pos + x_stride) % width

    print(f"Encountered {trees_encountered} trees")

if __name__ == '__main__':
    main()
