import sys
from string import Template
import re

def input():
    with open(sys.argv[1]) as f:
        for line in f:
            yield line

def policy_pattern(policy_str):
    PATTERN_TEMPLATE = Template('^([^$letter]*$letter){$min_occurrences,$max_occurrences}[^$letter]*$$')

    (occurrences, _, letter) = policy_str.partition(' ')
    (min_occurrences, _, max_occurrences) = occurrences.partition('-')

    return re.compile(PATTERN_TEMPLATE.substitute(letter=letter, min_occurrences=min_occurrences, max_occurrences=max_occurrences))
    

def main():
    valid_count = 0
    total_count = 0

    for line in input():
        (policy, _, password) = line.partition(': ')
        pattern = policy_pattern(policy)
        if pattern.match(password):
            valid_count += 1
        total_count += 1

    print(valid_count)



if __name__ == '__main__':
    main()
