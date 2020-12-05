import sys

def input():
    with open(sys.argv[1]) as f:
        for line in f:
            yield line

def is_valid(passport):
    REQUIRED_KEYS = ['byr', 'iyr', 'eyr', 'hgt', 'hcl', 'ecl', 'pid']
    for key in REQUIRED_KEYS:
        if key not in passport:
            return False
    return True

def main():
    passport_list = [{}]
    for line in input():
        if line == '\n':
            passport_list.append({})
        else:
            for pair in line.strip().split(' '):
                (key, _, value) = pair.partition(':')
                passport_list[-1][key] = value

    # print(passport_list)
    
    valid_count = 0
    for passport in passport_list:
        if is_valid(passport):
            valid_count += 1

    print(valid_count)

if __name__ == '__main__':
    main()
