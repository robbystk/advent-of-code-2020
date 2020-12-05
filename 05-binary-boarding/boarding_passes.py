import sys

def input():
    with open(sys.argv[1]) as f:
        for line in f:
            yield line

def main():
    max_seat_id = 0
    for line in input():
        row_code = line[0:7]
        column_code = line[7:10]

        row_num = row_code.replace('F', '0').replace('B', '1')
        col_num = column_code.replace('L', '0').replace('R', '1')

        row_num = int(row_num, 2)
        col_num = int(col_num, 2)

        seat_id = row_num * 8 + col_num

        if seat_id > max_seat_id:
            max_seat_id = seat_id

    print(max_seat_id)

if __name__ == '__main__':
    main()
