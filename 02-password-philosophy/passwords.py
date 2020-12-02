import sys
from string import Template
import re

def input():
    with open(sys.argv[1]) as f:
        for line in f:
            yield line

def parse_policy(policy_str):
    (occurrences, _, letter) = policy_str.partition(' ')
    (min_occurrences, _, max_occurrences) = occurrences.partition('-')

    return (letter, min_occurrences, max_occurrences)

def policy_pattern(policy_str):
    PATTERN_TEMPLATE = Template('^([^$letter]*$letter){$min_occurrences,$max_occurrences}[^$letter]*$$')

    (letter, min_occurrences, max_occurrences) = parse_policy(policy_str)

    return re.compile(PATTERN_TEMPLATE.substitute(letter=letter, min_occurrences=min_occurrences, max_occurrences=max_occurrences))

def part_one_valid(policy, password):
    pattern = policy_pattern(policy)
    return pattern.match(password)
    
def part_two_valid(policy, password):
    (letter, first_position, second_position) = parse_policy(policy)
    # convert to zero-based indices
    first_position = int(first_position) - 1
    second_position = int(second_position) - 1
    # check both positions
    in_first = password[first_position] == letter
    in_second = password[second_position] == letter

    # valid if one or the other but not both positions have the letter
    return (in_first or in_second) and not (in_first and in_second)

def main():
    valid_count = 0
    total_count = 0

    for line in input():
        (policy, _, password) = line.partition(': ')
        if part_two_valid(policy, password):
            valid_count += 1
        total_count += 1

    print(valid_count)

if __name__ == '__main__':
    main()
