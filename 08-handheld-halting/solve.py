import sys

import re

def input():
    with open(sys.argv[1]) as f:
        for line in f:
            yield line

class Instruction:
    def __init__(self, action, value):
        self.action = action
        self.value = value
    
    def parse(line):
        instruction_pattern = re.compile('\A(acc|jmp|nop) ([+\-]\d+)\n\Z')
        matches = instruction_pattern.match(line)
        action = matches.group(1)
        value = int(matches.group(2))

        return Instruction(action, value)

    def __repr__(self):
        if self.action == 'nop':
            return 'nop'
        else:
            return f'{self.action} {self.value}'

    def execute(self, state):
        if self.action == 'nop':
            state.program_counter += 1
        elif self.action == 'acc':
            state.accumulator += self.value
            state.program_counter += 1
        elif self.action == 'jmp':
            state.program_counter += self.value
        else:
            raise f'Unknown action {self.action}'
        

class State:
    def __init__(self, instruction_sequence):
        self.instruction_sequence = instruction_sequence
        self.program_counter = 0
        self.accumulator = 0
        self.executed_previously = [False for _ in instruction_sequence]

    def current_instruction(self):
        return self.instruction_sequence[self.program_counter]

    def step(self):
        self.executed_previously[self.program_counter] = True
        self.current_instruction().execute(self)

    def __repr__(self):
        return f'State {{ program_counter: {self.program_counter}, accumulator: {self.accumulator} }}'

    def run_until_repeat(self):
        run = True
        while run:
            if self.executed_previously[self.program_counter]:
                run = False
            else:
                self.step()

        return self.accumulator

def main():
    instruction_sequence = []
    for line in input():
        instruction_sequence.append(Instruction.parse(line))

    # print(instruction_sequence)

    state = State(instruction_sequence)
    acc_at_loop = state.run_until_repeat()
    print(state)

    print(acc_at_loop)

if __name__ == '__main__':
    main()
