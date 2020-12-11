import sys

from copy import deepcopy

def input():
    with open(sys.argv[1]) as f:
        for line in f:
            yield line

def print_seating_area(seating_area):
    for row in seating_area:
        print(''.join(row))

    print()

def main():
    seating_area = []
    for line in input():
        seating_area.append([c for c in line.strip()])

    print_seating_area(seating_area)

    n_rows = len(seating_area)
    n_cols = len(seating_area[0])

    occupation_threshold = 0
    vacation_threshold = 4

    changing = True
    while changing:
        old_seating_area = deepcopy(seating_area)
        changing = False

        for i in range(n_rows):
            for j in range(n_cols):
                if old_seating_area[i][j] == 'L':
                    # seat is empty
                    occupied_neighbors = 0
                    for di in [-1, 0, 1]:
                        if i + di >= 0 and i + di < n_rows:
                            for dj in [-1, 0, 1]:
                                if j+dj >= 0 and j+dj < n_cols:
                                    if old_seating_area[i+di][j+dj] == '#':
                                        occupied_neighbors += 1
                    if occupied_neighbors <= occupation_threshold:
                        changing = True
                        seating_area[i][j] = '#'
                elif old_seating_area[i][j] == '#':
                    # seat is full
                    occupied_neighbors = 0
                    for di in [-1, 0, 1]:
                        if i + di >= 0 and i + di < n_rows:
                            for dj in [-1, 0, 1]:
                                if j+dj >= 0 and j+dj < n_cols:
                                    if old_seating_area[i+di][j+dj] == '#':
                                        occupied_neighbors += 1
                    if occupied_neighbors - 1 >= vacation_threshold:
                        changing = True
                        seating_area[i][j] = 'L'
                else:
                    # seat is not a seat
                    pass

        # print_seating_area(seating_area)

    occupied_seats = 0
    for i in range(n_rows):
        for j in range(n_cols):
            if seating_area[i][j] == '#':
                occupied_seats += 1

    print(occupied_seats)

if __name__ == '__main__':
    main()
