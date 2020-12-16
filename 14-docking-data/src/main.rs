use std::collections::HashMap;

const SET_LSB: u64 = 0b0000_0000_0000_0000_0000_0000_0000_0000_0001;
const RESET_LSB: u64 = 0b1111_1111_1111_1111_1111_1111_1111_1111_1110;

fn main() {
    let input_filename = &std::env::args().collect::<Vec<String>>()[1];
    let input = std::fs::read_to_string(input_filename).unwrap();

    let mut memory = HashMap::<usize, u64>::new();

    let mut zero_mask: u64 = 0;
    let mut one_mask: u64 = 0;

    for line in input.lines() {
        let parts = line.split('=').collect::<Vec<&str>>();

        let directive = parts[0].trim();
        let value = parts[1].trim();

        // println!("{}, {}", directive, value);

        if directive.starts_with("mask") {
            zero_mask = 0;
            one_mask = 0;
            for c in value.chars() {
                zero_mask <<= 1;
                one_mask <<= 1;
                match c {
                    'X' => {
                        zero_mask |= SET_LSB;
                        one_mask &= RESET_LSB;
                    }
                    '0' => {
                        zero_mask &= RESET_LSB;
                        one_mask &= RESET_LSB;
                    }
                    '1' => {
                        zero_mask |= SET_LSB;
                        one_mask |= SET_LSB;
                    },
                    _ => panic!("Invalid mask character {}", c)
                }
            }
        } else if directive.starts_with("mem[") {
            let address = directive[4..].strip_suffix(']').unwrap();
            let value: u64 = value.parse().unwrap();
            memory.insert(address.parse().unwrap(), value & zero_mask | one_mask);
        } else {
            panic!("Can't parse line: `{}`", line);
        }
        // println!("{:#038b}\n{:#038b}", zero_mask, one_mask);
    }

    let memory_sum: u64 = memory.values().sum();

    println!("{:?}", memory_sum);
}
