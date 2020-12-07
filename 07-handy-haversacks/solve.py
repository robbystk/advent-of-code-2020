import sys
import re

def input():
    with open(sys.argv[1]) as f:
        for line in f:
            yield line

path_cache = {}

def paths_to_color(rule_list, starting_color, desired_color):
    rv = []
    for content in rule_list[starting_color]:
        color = content[1]
        if color == desired_color:
            rv.append([desired_color, starting_color])
        elif color is None:
            next
        else:
            paths = paths_to_color(rule_list, color, desired_color)
            for path in paths:
                path.append(starting_color)
                rv.append(path)
    return rv

def bags_inside(rule_list, starting_color):
    rv = 0
    for content in rule_list[starting_color]:
        number = content[0]
        color = content[1]

        if color is None or number == 0:
            next
        else:
            rv += number * (bags_inside(rule_list, color) + 1)
    return rv

def main():
    rule_pattern = re.compile('\A(\d+) (.+) bags?\.?\Z')

    luggage_rules = {}
    for line in input():
        outer_color, _, contents = line.strip().partition(' bags contain ')
        rules = contents.split(', ')
        contents = []
        for rule in rules:
            if rule == 'no other bags.':
                contents.append((0, None))
            else:
                matches = rule_pattern.match(rule)
                number = int(matches.group(1))
                color = matches.group(2)
                contents.append((number, color))

        luggage_rules[outer_color] = contents
    
    # print(luggage_rules)

    desired_color = 'shiny gold'

    paths = []
    for color in luggage_rules:
        paths += paths_to_color(luggage_rules, color, desired_color)

    starting_colors = set()
    for path in paths:
        starting_colors.add(path[-1])

    # print(len(starting_colors))

    print(bags_inside(luggage_rules, 'shiny gold'))

if __name__ == '__main__':
    main()
