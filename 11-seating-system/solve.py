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

OCCUPATION_THRESHOLD = 0
VACATION_THRESHOLD = 4

class SeatingArea:
    def __init__(self, seat_configuration):
        self.seat_config = seat_configuration
        self.n_rows = len(self.seat_config)
        self.n_cols = len(self.seat_config[0])

    def is_occupied(self, i, j):
        return self.seat_config[i][j] == '#'

    def is_seat(self, i, j):
        return self.seat_config[i][j] != '.'

    def count_occupied_neighbors(self, i, j):
        occupied_neighbors = 0
        for di in [-1, 0, 1]:
            if i + di >= 0 and i + di < self.n_rows:
                for dj in [-1, 0, 1]:
                    if j+dj >= 0 and j+dj < self.n_cols:
                        if self.is_occupied(i+di, j+dj):
                            occupied_neighbors += 1
        return occupied_neighbors

    def __repr__(self):
        return '\n'.join([''.join(row) for row in self.seat_config]) + '\n'

    def step(self):
        new_seat_config = deepcopy(self.seat_config)
        changed = False

        for i in range(self.n_rows):
            for j in range(self.n_cols):
                if self.is_seat(i, j):
                    if self.is_occupied(i, j):
                        if self.count_occupied_neighbors(i,j) - 1 >= VACATION_THRESHOLD:
                            new_seat_config[i][j] = 'L'
                            changed = True
                    else:
                        if self.count_occupied_neighbors(i,j) <= OCCUPATION_THRESHOLD:
                            new_seat_config[i][j] = '#'
                            changed = True

        self.seat_config = new_seat_config
        return changed

def main():
    seating_area = []
    for line in input():
        seating_area.append([c for c in line.strip()])

    seating_area = SeatingArea(seating_area)
    print(seating_area)

    changing = True
    while changing:
        changing = seating_area.step()

        # print_seating_area(seating_area)

    occupied_seats = 0
    for i in range(seating_area.n_rows):
        for j in range(seating_area.n_cols):
            if seating_area.is_occupied(i,j):
                occupied_seats += 1

    print(occupied_seats)

if __name__ == '__main__':
    main()
