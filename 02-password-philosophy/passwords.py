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
    

def main():
    valid_count = 0
    total_count = 0

    for line in input():
        (policy, _, password) = line.partition(': ')
        if part_one_valid(policy, password):
            valid_count += 1
        total_count += 1

    print(valid_count)

if __name__ == '__main__':
    main()
