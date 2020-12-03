import sys

import numpy as np

def input():
    with open(sys.argv[1]) as f:
        for line in f:
            yield line

class TreeField:
    def __init__(self, array):
        self.tree_field = np.array(array)

    def has_tree(self, x, y):
        return self.tree_field[y, x]

    def height(self):
        return self.tree_field.shape[0]

    def width(self):
        return self.tree_field.shape[1]

    def traverse(self, x_stride, y_stride):
        x_pos, y_pos = 0, 0
        trees_encountered = 0
        while y_pos < self.height():
            if self.has_tree(x_pos,y_pos):
                trees_encountered += 1
            y_pos += y_stride
            x_pos = (x_pos + x_stride) % self.width()
        return trees_encountered


    def __repr__(self):
        return '\n'.join([''.join(['#' if self.has_tree(x,y) else '.' for x in range(self.width())]) for y in range(self.height())])

def main():
    tree_field = []
    for line in input():
        row = []
        for c in line.strip():
            row.append(c == '#')
        tree_field.append(row)

    tree_field = TreeField(tree_field)

    x_stride = 3
    y_stride = 1
    trees_encountered = tree_field.traverse(x_stride, y_stride)

    print(f"Encountered {trees_encountered} trees")

if __name__ == '__main__':
    main()
