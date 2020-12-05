import sys

import re

def input():
    with open(sys.argv[1]) as f:
        for line in f:
            yield line

def check_year(year_str, min_year, max_year):
    YEAR_REGEX = re.compile('\d{4}')
    if not YEAR_REGEX.match(year_str):
        # print(f"invalid year: {year_str} doesn't have four digits.")
        return False

    year = int(year_str)
    if year > max_year or year < min_year:
        # print(f'invalid year: {year} is not between {min_year} and {max_year}')
        return False

    return True

def check_height(height_str):
    matches = re.compile('\A(\d+)(cm|in)\Z').match(height_str)
    if not matches:
        return False

    height = int(matches.group(1))
    suffix = matches.group(2)

    if suffix == 'cm':
        if 150 <= height and height <= 193:
            return True
        else:
            # print(f"invalid height: {height} isn't between 150 and 193")
            return False
    elif suffix == 'in':
        if 59 <= height and height <= 76:
            return True
        else:
            # print(f"invalid height: {height} isn't between 59 and 76")
            return False
    else:
        return False

def is_valid(passport):
    VALIDATORS = {
            'byr': (lambda v: check_year(v, 1920, 2002)),
            'iyr': (lambda v: check_year(v, 2010, 2020)),
            'eyr': (lambda v: check_year(v, 2020, 2030)),
            'hgt': check_height,
            'hcl': (lambda v: re.compile('#[0-9a-f]{6}').match(v)),
            'ecl': (lambda v: v in {'amb', 'blu', 'brn', 'gry', 'grn', 'hzl', 'oth'}),
            'pid': (lambda v: re.compile('\A\d{9}\Z').match(v))
        }

    for key, validator in VALIDATORS.items():
        if key not in passport:
            # print(f'{key} is missing')
            return False
        elif not validator(passport[key]):
            # print(f'invalid field {key}: {passport[key]}')
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
