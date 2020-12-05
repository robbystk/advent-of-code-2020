import sys

def input():
    with open(sys.argv[1]) as f:
        for line in f:
            yield line

def main():
    seat_ids = []
    for line in input():
        row_code = line[0:7]
        column_code = line[7:10]

        row_num = row_code.replace('F', '0').replace('B', '1')
        col_num = column_code.replace('L', '0').replace('R', '1')

        row_num = int(row_num, 2)
        col_num = int(col_num, 2)

        seat_id = row_num * 8 + col_num
        seat_ids.append(seat_id)

    seat_ids.sort()

    for i in range(len(seat_ids) - 1):
        delta = seat_ids[i+1] - seat_ids[i]
        if delta == 2:
            print(seat_ids[i] + 1)
            break

if __name__ == '__main__':
    main()
